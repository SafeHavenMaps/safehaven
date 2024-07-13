import { reactive } from 'vue'
import { transform } from 'ol/proj.js'
import type { Extent } from 'ol/extent'
import { purify_lenient } from './dompurify'
import useClient from '~/lib/viewer-client'
import type {
  Category,
  Family,
  Tag,
  DisplayableCachedEntity,
  DisplayableCluster,
  FetchedEntity,
  InitConfig,
  EnumFilter,
  PublicPermissions,
  ViewerPaginatedCachedEntities,
} from '~/lib'

type ViewData = {
  entities: DisplayableCachedEntity[]
  clusters: DisplayableCluster[]
}

type AllowedCategory = Category & { allowed: boolean }
type AllowedTag = Tag & { allowed: boolean }

export class AppState {
  public initialized = false

  private _client: ReturnType<typeof useClient> | null = null

  get client() {
    return this._client!
  }

  public initConfig: InitConfig | null = null
  public permissions: PublicPermissions | null = null

  private familiesData: Family[] | null = null
  private categoriesData: AllowedCategory[] | null = null
  private tagsData: AllowedTag[] | null = null

  private familiesLookupTable: Record<string, Family> = {}
  private categoriesLookupTable: Record<string, Category> = {}
  private tagsLookupTable: Record<string, Tag> = {}

  private viewData: ViewData = {
    entities: [],
    clusters: [],
  }

  private activeFamilyId: string | null = null

  private _activeEntity: FetchedEntity | null = null

  public filteringTags: (Tag & { active: boolean | null })[] = []
  public filteringCategories: (Category & { active: boolean })[] = []
  public filteringEnums: EnumFilter[] = []

  get mapSource() {
    return {
      light: {
        url: this.initConfig?.cartography_source.light_map_url,
        attribution: this.initConfig?.cartography_source.light_map_attributions,
      },
      dark: {
        url: this.initConfig?.cartography_source.dark_map_url,
        attribution: this.initConfig?.cartography_source.dark_map_attributions,
      },
    }
  }

  get activeFilteringCategories() {
    return this.filteringCategories.filter(c => c.active).map(c => c.id)
  }

  get activeRequiredTags() {
    return this.filteringTags.filter(t => t.active).map(t => t.id)
  }

  get activeFilteringEnums(): Record<string, string[]> {
    return Object
      .fromEntries(
        this.filteringEnums
          .filter(f => f.active.length > 0)
          .map(f => [f.key, f.active]),
      )
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
      .filter(c => c.family_id === family.id && c.allowed)
      .map((c) => {
        return {
          ...c,
          active: c.default_status,
        }
      })

    if (this.permissions?.can_list_with_enum_constraints)
      this.filteringEnums = family.entity_form.fields
        .filter(
          f => f.indexed
          && !f.privately_indexed
          && (f.field_type === 'EnumMultiOption' || f.field_type === 'EnumSingleOption'))
        .map((f) => {
          return {
            key: f.key,
            title: f.display_name,
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            values: (f.field_type_metadata as any).options.map((v: any) => {
              return {
                label: v.label,
                value: v.value,
              }
            }),
            active: [],
          }
        })
  }

  get online() {
    return this.initConfig?.status === 'ok'
  }

  get hasSafeModeEnabled() {
    return !!this.initConfig?.safe_mode.enabled
  }

  get safeMode() {
    return this.initConfig!.safe_mode
  }

  get hCaptchaSiteKey() {
    return this.initConfig?.safe_mode.hcaptcha_sitekey
  }

  get title() {
    return this.initConfig!.general.title!
  }

  get subtitle() {
    return this.initConfig!.general.subtitle ?? null
  }

  get logo() {
    return this.initConfig!.general.logo_url ?? null
  }

  get loaded() {
    return this.initConfig !== null
  }

  get hasInformation() {
    return !!this.initConfig?.general.information
  }

  get getSanitizedPopup() {
    if (!this.initConfig?.init_popup.popup) {
      return null
    }

    return {
      siteTitle: this.initConfig!.general.title!,
      sanitizedContent: purify_lenient(this.initConfig!.init_popup.popup!),
      sanitizedCheckbox: this.initConfig!.init_popup.popup_check_text
        ? purify_lenient(this.initConfig!.init_popup.popup_check_text!)
        : null,
    }
  }

  get redirectUrl() {
    return this.initConfig!.general.redirect_url
  }

  async getSanitizedInformation() {
    if (!this.initConfig?.general.information) {
      return null
    }

    return purify_lenient(this.initConfig!.general.information!)
  }

  async init() {
    this._client = useClient()
    this.initConfig = await this.client.checkStatus()
    this.client.onAuthenticationFailed(async () => {
      // It is impossible to recover from this error, since the token is deemed invalid
      // and the user needs to use a valid URL from now on. If the redirect url is set,
      // we will redirect the user to the configured URL. Otherwise, we will show an error.
      if (this.redirectUrl) {
        window.location.href = this.redirectUrl
      }
      else {
        throw createError({
          statusCode: 404,
          statusMessage: 'Page Not Found',
          fatal: true,
        })
      }
    })
  }

  async bootstrapWithToken(token: string) {
    if (this.initialized) return

    const data = await this.client.bootstrap(token)

    this.permissions = {
      can_list_entities: data.can_list_entities,
      can_access_entity: data.can_access_entity,
      can_add_entity: data.can_add_entity,
      can_access_comments: data.can_access_comments,
      can_add_comment: data.can_add_comment,
      can_list_without_query: data.can_list_without_query,
      can_list_with_filters: data.can_list_with_filters,
      can_list_with_enum_constraints: data.can_list_with_enum_constraints,
    }

    this.familiesData = data.families
      .sort((a, b) => a.sort_order - b.sort_order)

    this.categoriesData = data.categories
      .map((category) => {
        return {
          ...category,
          allowed: data.allowed_categories.includes(category.id),
        }
      })
      .sort((a, b) => a.title.localeCompare(b.title))

    this.tagsData = data.tags.map((tag) => {
      return {
        ...tag,
        allowed: data.allowed_tags.includes(tag.id),
      }
    })

    if (this.permissions.can_list_with_filters)
      this.filteringTags = this.tagsData
        .filter(tag => tag.allowed)
        .filter(tag => tag.is_filter)
        .map((tag) => {
          return {
            ...tag,
            active: tag.default_filter_status ? null : false,
          }
        })

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

  get categories(): AllowedCategory[] {
    return this.categoriesData!
  }

  get tags(): Tag[] {
    return this.tagsData!
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
        state.initConfig!.cartography_init.center_lng!,
        state.initConfig!.cartography_init.center_lat!,
      ],
      'EPSG:4326', // WGS84
      'EPSG:3857', // Web Mercator
    )
  }

  startZoom() {
    return state.initConfig!.cartography_init.zoom
  }

  async selectedCachedEntity(cacheEntity: DisplayableCachedEntity) {
    this._activeEntity = await this.client.fetchEntity(
      cacheEntity.entity_id,
      this.activeFilteringCategories,
      this.activeRequiredTags,
      this.activeHiddenTags,
    )
  }

  async selectEntity(id: string) {
    this._activeEntity = await this.client.fetchEntity(
      id,
      this.activeFilteringCategories,
      this.activeRequiredTags,
      this.activeHiddenTags,
    )
  }

  async searchEntities(query: string, page: number, pageSize: number, require_locations?: boolean): Promise<ViewerPaginatedCachedEntities> {
    if (this.permissions?.can_list_without_query || query.length >= 4)
      return await this.client.searchEntities(
        query,
        this.activeFamilyId!,
        this.activeFilteringCategories,
        this.activeRequiredTags,
        this.activeHiddenTags,
        page,
        pageSize,
        this.activeFilteringEnums,
        require_locations ?? false,
      )
    else
      return { entities: [], response_current_page: 0, total_pages: 0, total_results: 0 }
  }

  getCategory(category_id: string) {
    return this.categories.find(c => c.id === category_id)!
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
      this.activeFilteringEnums,
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
        coordinates: [entity.web_mercator_x!, entity.web_mercator_y!],
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
