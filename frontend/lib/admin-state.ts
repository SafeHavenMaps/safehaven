import { reactive } from 'vue'
import useClient from '~/lib/admin-client'
import type {
  Family,
  NewOrUpdateFamily,
  SafeHavenOptions,
  ConfigurationOption,
  User,
  NewOrUpdatedUser,
} from '~/lib'

export class AppState {
  public client = useClient()
  // For categories, users, tags, access tokens, entities and comments the admin client shall be used directly

  private optionsData: SafeHavenOptions | null = null
  private usersData: User[] | null = null
  private familiesData: Family[] | null = null

  public userAdminCount: number | null = null

  public is_admin: boolean | null = null
  public username: string | null = null

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
    return this.optionsData!
  }

  async updateConfig(name: string, config: ConfigurationOption): Promise<void> {
    const updatedConfig = await this.client.updateConfig(name, config)
    this.optionsData = updatedConfig
  }

  get hasSafeMode() {
    return !!this.optionsData!.safe_mode.enabled
  }

  get safeMode() {
    return this.optionsData!.safe_mode.enabled
  }

  // Users
  async fetchUsers(): Promise<void> {
    this.usersData = await this.client.listUsers()
    this.userAdminCount = state.users.filter(user => user.is_admin).length
  }

  get users(): User[] {
    return this.usersData!
  }

  async getUser(id: string): Promise<User> {
    return await this.client.getUser(id)
  }

  async createUser(newUser: NewOrUpdatedUser): Promise<void> {
    const createdUser = await this.client.createUser(newUser)
    this.usersData!.push(createdUser)
  }

  async updateUser(id: string, updatedUser: NewOrUpdatedUser): Promise<void> {
    const returnedUser = await this.client.updateUser(id, updatedUser)
    const index = this.usersData!.findIndex(u => u.id === id)
    if (index !== -1) {
      this.usersData![index] = returnedUser
    }
  }

  async updateSelfPassword(newPasswordDetails: string): Promise<void> {
    const updatedUSer = { name: state.username!, password: newPasswordDetails, is_admin: state.is_admin! }
    const returnedUser = await this.client.changeSelfPassword(updatedUSer)
    const selfIndex = this.usersData!.findIndex(u => u.id === returnedUser.id)
    if (selfIndex !== -1) {
      this.usersData![selfIndex] = returnedUser
    }
  }

  async deleteUser(id: string): Promise<void> {
    await this.client.deleteUser(id)
    this.usersData = this.usersData!.filter(u => u.id !== id)
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
