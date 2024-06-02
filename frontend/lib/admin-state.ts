import { reactive } from 'vue'
import type { DataTableFilterMeta } from 'primevue/datatable'
import { purify_lenient } from './dompurify'
import useClient from '~/lib/admin-client'
import type {
  Family,
  NewOrUpdateFamily,
  SafeHavenOptions,
  ConfigurationOption,
} from '~/lib'

export class AppState {
  public client = useClient()
  // For categories, users, tags, access tokens, entities and comments the admin client shall be used directly

  private optionsData: SafeHavenOptions | null = null
  private familiesData: Family[] | null = null

  public userAdminCount: number | null = null

  public is_admin: boolean | null = null
  public username: string | null = null

  public tablesSelectedColumns: Record<string, string[]> = {}
  public tablesFilters: Record<string, DataTableFilterMeta> = {}

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

  get options(): SafeHavenOptions {
    const raw_options = this.optionsData!
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
  }

  get families(): Family[] {
    return this.familiesData!
  }

  async fetchFamily(id: string) {
    return await this.client.getFamily(id)
  }

  async createFamily(family: NewOrUpdateFamily) {
    const newFamily = await this.client.createFamily(family)
    this.familiesData!.push(newFamily)
  }

  async updateFamily(id: string, family: NewOrUpdateFamily) {
    const updatedFamily = await this.client.updateFamily(id, family)
    const index = this.familiesData!.findIndex(f => f.id === id)
    if (index !== -1) {
      this.familiesData![index] = updatedFamily
    }
  }

  async deleteFamily(id: string) {
    await this.client.deleteFamily(id)
    this.familiesData = this.familiesData!.filter(f => f.id !== id)
  }
}

const state = reactive(new AppState())
export default state
