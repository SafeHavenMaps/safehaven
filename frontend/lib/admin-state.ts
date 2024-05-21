import { reactive } from 'vue'
import useClient from '~/lib/admin-client'
import type {
  CartographyInitConfig,
  Category,
  Family,
  Tag,
  Status,
} from '~/lib'

const client = useClient()

export class AppState {
  initialized = false

  private familiesData: Family[] | null = null
  private categoriesData: Category[] | null = null
  private tagsData: Tag[] | null = null
  private cartographyInitConfigData: CartographyInitConfig | null = null

  private status: Status | null = null

  get online() {
    return this.status?.status === 'ok'
  }

  get hasSafeMode() {
    return !!this.status?.safe_mode
  }

  get safeMode() {
    return this.status!.safe_mode
  }

  async initWithUsernamePassword(username: string, password: string, remember_me: boolean) {
    await client.bootstrap(username, password, remember_me)
    this.initialized = true
  }

  async fetchFamilies() {
    this.familiesData = await client.getFamily()
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
}

const state = reactive(new AppState())
export default state
