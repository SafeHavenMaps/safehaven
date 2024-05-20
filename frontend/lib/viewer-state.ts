import { reactive } from 'vue'
import { transform } from 'ol/proj.js'
import type { Extent } from 'ol/extent'
import useClient from '~/lib/viewer-client'
import type {
  CartographyInitConfig,
  Category,
  Family,
  Tag,
  DisplayableCachedEntity,
  DisplayableCluster,
  FetchedEntity,
  Status,
} from '~/lib'

type ViewData = {
  entities: DisplayableCachedEntity[]
  clusters: DisplayableCluster[]
}

export class AppState {
  initialized = false

  private _client: ReturnType<typeof useClient> | null = null

  get client() {
    return this._client!
  }

  private familiesData: Family[] | null = null

  private categoriesData: Category[] | null = null
  private tagsData: Tag[] | null = null
  private cartographyInitConfigData: CartographyInitConfig | null = null

  private familiesLookupTable: Record<string, Family> = {}
  private categoriesLookupTable: Record<string, Category> = {}
  private tagsLookupTable: Record<string, Tag> = {}

  private viewData: ViewData = {
    entities: [],
    clusters: [],
  }

  private activeFamilyId: string | null = null

  private _activeEntity: FetchedEntity | null = null

  private filteringTags: (Tag & { active: boolean | null })[] = []
  private filteringCategories: (Category & { active: boolean })[] = []

  get activeFilteringCategories() {
    return this.filteringCategories.filter(c => c.active).map(c => c.id)
  }

  get activeRequiredTags() {
    return this.filteringTags.filter(t => t.active).map(t => t.id)
  }

  get activeHiddenTags() {
    return this.filteringTags.filter(t => t.active === false).map(t => t.id)
  }

  get activeEntity() {
    if (this._activeEntity === null) {
      return null
    }

    return {
      ...this._activeEntity!,
      family: this.familiesLookupTable[this._activeEntity!.entity.family_id],
      category: this.categoriesLookupTable[this._activeEntity!.entity.category_id],
      tags: this._activeEntity!.entity.tags.map(tagId => this.tagsLookupTable[tagId]),
    }
  }

  private status: Status | null = null

  get entities() {
    return this.viewData.entities
  }

  get clusters() {
    return this.viewData.clusters
  }

  get activeFamily() {
    return this.familiesLookupTable[this.activeFamilyId!]
  }

  set activeFamily(family: Family) {
    this.activeFamilyId = family.id
    this.filteringCategories = this.categories
      .filter(c => c.family_id === family.id)
      .map((c) => {
        return {
          ...c,
          active: c.default_status,
        }
      })
  }

  get online() {
    return this.status?.status === 'ok'
  }

  get hasSafeMode() {
    return !!this.status?.safe_mode
  }

  get safeMode() {
    return this.status!.safe_mode
  }

  get title() {
    return this.status!.general.title!
  }

  get subtitle() {
    return this.status!.general.subtitle ?? null
  }

  get logo() {
    return this.status!.general.logo_url ?? null
  }

  get loaded() {
    return this.status !== null
  }

  async init() {
    this._client = useClient()
    this.status = await this.client.checkStatus()
  }

  async bootstrapWithToken(token: string) {
    const data = await this.client.bootstrap(token)

    this.familiesData = data.families
    this.categoriesData = data.categories
    this.tagsData = data.tags
    this.filteringTags = this.tagsData.map((tag) => {
      return {
        ...tag,
        active: tag.default_filter_status ? null : false,
      }
    })
    this.cartographyInitConfigData = data.cartography_init_config

    this.familiesData.forEach((family) => {
      this.familiesLookupTable[family.id] = family
    })

    this.categoriesData.forEach((category) => {
      this.categoriesLookupTable[category.id] = category
    })

    this.tagsData.forEach((tag) => {
      this.tagsLookupTable[tag.id] = tag
    })

    if (this.familiesData.length === 0) {
      throw new Error('No families available')
    }

    this.activeFamily = this.familiesData[0]

    this.initialized = true
  }

  get families(): Family[] {
    return this.familiesData!
  }

  get categories(): Category[] {
    return this.categoriesData!
  }

  get tags(): Tag[] {
    return this.tagsData!
  }

  get cartographyInitConfig(): CartographyInitConfig {
    return this.cartographyInitConfigData!
  }

  get hasActiveEntity() {
    return this.activeEntity !== null
  }

  set hasActiveEntity(value: boolean) {
    if (!value) {
      this._activeEntity = null
    }
  }

  startCenter() {
    // The configuration files is in WGS84 (EPSG:4326)
    // The map is in Web Mercator (EPSG:3857)
    // We need to transform the coordinates
    return transform(
      [
        state.cartographyInitConfig.center_lng,
        state.cartographyInitConfig.center_lat,
      ],
      'EPSG:4326', // WGS84
      'EPSG:3857', // Web Mercator
    )
  }

  startZoom() {
    return state.cartographyInitConfig.zoom
  }

  async selectedCachedEntity(cacheEntity: DisplayableCachedEntity) {
    this._activeEntity = await this.client.fetchEntity(cacheEntity.entity_id)
  }

  async selectEntity(id: string) {
    this._activeEntity = await this.client.fetchEntity(id)
  }

  async refreshView(extent: Extent, zoomLevel: number) {
    const zoom = Math.round(zoomLevel)
    const newViewData = await this.client.getEntitiesWithinBounds(
      {
        xmin: extent[0],
        ymin: extent[1],
        xmax: extent[2],
        ymax: extent[3],
      },
      zoom,
      this.activeFamilyId!,
      this.activeFilteringCategories,
      this.activeRequiredTags,
      this.activeHiddenTags,
    )

    // Step 1: Identify and filter out entities that are no longer present
    const existingEntityIds = new Set(newViewData.entities.map(ne => ne.id))
    this.viewData.entities = this.viewData.entities.filter(e =>
      existingEntityIds.has(e.id),
    )

    // Step 2: Add new entities that are not already in viewData
    const currentEntityIds = new Set(this.viewData.entities.map(e => e.id))
    const newEntities = newViewData.entities.filter(
      ne => !currentEntityIds.has(ne.id),
    )
    this.viewData.entities.push(
      ...newEntities.map(entity => ({
        ...entity,
        coordinates: [entity.web_mercator_x, entity.web_mercator_y],
        family: this.familiesLookupTable[entity.family_id],
        category: this.categoriesLookupTable[entity.category_id],
        highlighted: false,
      })),
    )

    // Step 3: Identify and filter out clusters that are no longer present
    const existingClusterIds = new Set(newViewData.clusters.map(nc => nc.id))
    this.viewData.clusters = this.viewData.clusters.filter(c =>
      existingClusterIds.has(c.id),
    )

    // Step 4: Add new clusters that are not already in viewData
    const currentClusterIds = new Set(this.viewData.clusters.map(c => c.id))
    const newClusters = newViewData.clusters.filter(
      nc => !currentClusterIds.has(nc.id),
    )
    this.viewData.clusters.push(
      ...newClusters.map(cluster => ({
        ...cluster,
        coordinates: [cluster.center_x, cluster.center_y],
      })),
    )
  }
}

const state = reactive(new AppState())
export default state
