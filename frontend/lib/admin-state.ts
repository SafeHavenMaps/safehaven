import { reactive } from 'vue'
import type { DataTableFilterMeta } from 'primevue/datatable'
import { purify_lenient } from './dompurify'
import useClient from '~/lib/admin-client'
import type {
  Family,
  NewOrUpdateFamily,
  SafeHavenOptions,
  ConfigurationOption,
  Tag,
  Category,
  NewOrUpdateTag,
  NewOrUpdateCategory,
  CategoryRecord,
  FamilyRecord,
  TagRecord,
  EnumFilter,
  SafeHavenVersion,
} from '~/lib'

interface Identifiable {
  id: string | number
}
export class AppState {
  public client = useClient()
  // For categories, users, tags, access tokens, entities and comments the admin client shall be used directly

  public versionInformation: SafeHavenVersion | null = null
  private optionsData: SafeHavenOptions | null = null
  private familiesData: Family[] | null = null
  public familyRecord: FamilyRecord = {}
  private categoriesData: Category[] | null = null
  public categoryRecord: CategoryRecord = {}
  private tagsData: Tag[] | null = null
  public tagRecord: TagRecord = {}

  private countsByFamilyData: Record<string, number[]> = {}
  private countsByCategoryData: Record<string, number[]> = {}

  public is_admin: boolean | null = null
  public username: string | null = null

  public tablesSelectedColumns: Record<string, string[]> = {}
  public tablesFilters: Record<string, DataTableFilterMeta> = {}
  public tablesQueryParams: Record<string, {
    search_query: string
    currentPage: number
    pageSize: number
    categoryFilteringList?: (Category & { active: boolean })[]
    tagFilteringList?: (Tag & { active: boolean | null })[]
    enumsFilteringList: EnumFilter[]
  }> = {}

  private compareIds(array1: Identifiable[] | null, array2: Identifiable[] | null): boolean {
    const ids1 = array1?.map(item => item.id).sort()
    const ids2 = array2?.map(item => item.id).sort()
    if (ids1 == undefined || ids2 == undefined || ids1.length !== ids2.length) return false
    for (let i = 0; i < ids1.length; i++) {
      if (ids1[i] !== ids2[i]) return false
    }
    return true
  }

  // Auth
  async login(username: string, password: string, remember_me: boolean) {
    this.is_admin = await this.client.login(username, password, remember_me)
    this.username = username
    return true
  }

  async logout() {
    await this.client.logout()
  }

  async check_login(): Promise<boolean> {
    const claims = await this.client.check_login()
    if (claims === null) return false
    this.is_admin = claims.is_admin
    this.username = claims.username
    return true
  }

  // Config
  async fetchConfig(): Promise<void> {
    this.optionsData = await this.client.getConfig()
  }

  async fetchVersionInformation(): Promise<void> {
    this.versionInformation = await this.client.getVersionInformation()
  }

  get options(): SafeHavenOptions {
    const raw_options: SafeHavenOptions = this.optionsData ?? {
      general: {
        subtitle: 'Carte associative',
        title: 'SafeHaven',
        information: null,
        logo_url: null,
        redirect_url: null,
      },
      cartography_source: {
        dark_map_attributions: '',
        dark_map_url: '',
        light_map_attributions: '',
        light_map_url: '',
      },
      init_popup: {
        popup: null,
        popup_check_text: null,
      },
      safe_mode: {
        enabled: false,
        hcaptcha_secret: '',
        hcaptcha_sitekey: '',
      },
      cartography_init: {
        center_lat: 0,
        center_lng: 0,
        zoom: 0,
      },
      cartography_cluster: {
        characteristic_distance: 0,
        declustering_speed: 0,
        minimal_cluster_size: 0,
      },
    }

    if (raw_options.general.information != undefined)
      raw_options.general.information = purify_lenient(raw_options.general.information)

    return raw_options
  }

  async updateConfig(name: string, config: ConfigurationOption): Promise<void> {
    const updatedConfig = await this.client.updateConfig(name, config)
    this.optionsData = updatedConfig
  }

  async deleteConfig(name: string) {
    const updatedConfig = await this.client.deleteConfig(name)
    this.optionsData = updatedConfig
  }

  get hasSafeMode() {
    return !!this.optionsData!.safe_mode.enabled
  }

  get safeMode() {
    return this.optionsData!.safe_mode.enabled
  }

  // Families
  async fetchFamilies(): Promise<void> {
    this.familiesData = await this.client.listFamilies()
    this.familyRecord = this.familiesData.reduce((families, family) => {
      families[family.id] = family
      return families
    }, {} as FamilyRecord)
  }

  get families(): Family[] {
    return this.familiesData!
  }

  familyById(id: string): Family {
    return this.familyRecord[id]
  }

  async fetchFamily(id: string) {
    return await this.client.getFamily(id)
  }

  async createFamily(family: NewOrUpdateFamily) {
    await this.client.createFamily(family)
  }

  async updateFamily(id: string, family: NewOrUpdateFamily) {
    await this.client.updateFamily(id, family)
  }

  async deleteFamily(id: string) {
    await this.client.deleteFamily(id)
  }

  // Categories
  async fetchCategories(): Promise<void> {
    const categories = await this.client.listCategories()
    if (!this.compareIds(categories, toRaw(this.categoriesData))) {
      this.tablesQueryParams = {}
    }
    this.categoriesData = categories
    this.categoryRecord = this.categories.reduce((categories, category) => {
      categories[category.id] = category
      return categories
    }, {} as CategoryRecord)
  }

  get categories(): Category[] {
    return this.categoriesData!
  }

  async fetchCategory(id: string): Promise<Category> {
    return await this.client.getCategory(id)
  }

  async createCategory(category: NewOrUpdateCategory): Promise<Category> {
    return await this.client.createCategory(category)
  }

  async updateCategory(id: string, category: NewOrUpdateCategory): Promise<void> {
    await this.client.updateCategory(id, category)
    this.tablesQueryParams = {}
  }

  async deleteCategory(id: string): Promise<void> {
    await this.client.deleteCategory(id)
  }

  // Tags
  async fetchTags(): Promise<void> {
    const tags = await this.client.listTags()
    if (!this.compareIds(tags, this.tagsData)) {
      this.tablesQueryParams = {}
    }
    this.tagsData = tags
    this.tagRecord = this.tags.reduce((tags, tag) => {
      tags[tag.id] = tag
      return tags
    }, {} as TagRecord)
  }

  get tags(): Tag[] {
    return this.tagsData!
  }

  async fetchTag(id: string): Promise<Tag> {
    return await this.client.getTag(id)
  }

  async createTag(tag: NewOrUpdateTag): Promise<Tag> {
    return await this.client.createTag(tag)
  }

  async updateTag(id: string, tag: NewOrUpdateTag): Promise<void> {
    await this.client.updateTag(id, tag)
    this.tablesQueryParams = {}
  }

  async deleteTag(id: string): Promise<void> {
    await this.client.deleteTag(id)
    this.tablesQueryParams = {}
  }

  // Stats
  async getEntitiesCommentsCounts() {
    const [countsByFamily, countsByCategory] = await this.client.getEntitiesCommentsCounts()
    this.countsByFamilyData = countsByFamily
    this.countsByCategoryData = countsByCategory
  }

  get countsByFamily() {
    return this.countsByFamilyData ?? {}
  }

  get countsByCategory() {
    return this.countsByCategoryData ?? {}
  }
}

const state = reactive(new AppState())
export default state
