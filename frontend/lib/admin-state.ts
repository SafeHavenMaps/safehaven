import { reactive } from 'vue'
import useClient from '~/lib/admin-client'
import type {
  Category,
  Family,
  Tag,
  NewOrUpdateFamily,
  NewCategory,
  UpdateCategory,
  NewOrUpdateTag,
  NewComment,
  UpdateComment,
  NewEntity,
  UpdateEntity,
  AccessToken,
  NewOrUpdateAccessToken,
  ListedEntity,
  ListedComment,
  SafeHavenOptions,
  ConfigurationOption,
  User,
  NewOrUpdatedUser,
} from '~/lib'

export class AppState {
  private client = useClient()

  private optionsData: SafeHavenOptions | null = null
  private usersData: User[] | null = null
  private familiesData: Family[] | null = null
  private categoriesData: Category[] | null = null
  private tagsData: Tag[] | null = null
  private accessTokensData: AccessToken[] | null = null
  private pendingEntitiesData: ListedEntity[] | null = null
  private pendingCommentsData: ListedComment[] | null = null

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
  async fetchFamilies() {
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

  // Categories
  async fetchCategories() {
    this.categoriesData = await this.client.listCategories()
  }

  get categories(): Category[] {
    return this.categoriesData!
  }

  async fetchCategory(id: string) {
    return await this.client.getCategory(id)
  }

  async createCategory(category: NewCategory) {
    const newCategory = await this.client.createCategory(category)
    this.categoriesData!.push(newCategory)
  }

  async updateCategory(id: string, category: UpdateCategory) {
    const updatedCategory = await this.client.updateCategory(id, category)
    const index = this.categoriesData!.findIndex(c => c.id === id)
    if (index !== -1) {
      this.categoriesData![index] = updatedCategory
    }
  }

  async deleteCategory(id: string) {
    await this.client.deleteCategory(id)
    this.categoriesData = this.categoriesData!.filter(c => c.id !== id)
  }

  // Tags
  async fetchTags() {
    this.tagsData = await this.client.listTags()
  }

  get tags(): Tag[] {
    return this.tagsData!
  }

  async fetchTag(id: string) {
    return await this.client.getTag(id)
  }

  async createTag(tag: NewOrUpdateTag) {
    const newTag = await this.client.createTag(tag)
    this.tagsData!.push(newTag)
  }

  async updateTag(id: string, tag: NewOrUpdateTag) {
    const updatedTag = await this.client.updateTag(id, tag)
    const index = this.tagsData!.findIndex(t => t.id === id)
    if (index !== -1) {
      this.tagsData![index] = updatedTag
    }
  }

  async deleteTag(id: string) {
    await this.client.deleteTag(id)
    this.tagsData = this.tagsData!.filter(t => t.id !== id)
  }

  // Access Tokens
  async fetchAccessTokens() {
    this.accessTokensData = await this.client.listAccessTokens()
  }

  get accessTokens(): AccessToken[] {
    return this.accessTokensData!
  }

  async fetchAccessToken(id: string) {
    return await this.client.getAccessToken(id)
  }

  async createAccessToken(token: NewOrUpdateAccessToken) {
    const newToken = await this.client.createAccessToken(token)
    this.accessTokensData!.push(newToken)
  }

  async updateAccessToken(id: string, token: NewOrUpdateAccessToken) {
    const updatedToken = await this.client.updateAccessToken(id, token)
    const index = this.accessTokensData!.findIndex(t => t.id === id)
    if (index !== -1) {
      this.accessTokensData![index] = updatedToken
    }
  }

  async deleteAccessToken(id: string) {
    await this.client.deleteAccessToken(id)
    this.accessTokensData = this.accessTokensData!.filter(t => t.id !== id)
  }

  // Entities
  async fetchEntities() {
    this.pendingEntitiesData = await this.client.listPendingEntities()
  }

  get entities(): ListedEntity[] {
    return this.pendingEntitiesData!
  }

  async getEntity(id: string) {
    return await this.client.getEntity(id)
  }

  async createEntity(entity: NewEntity) {
    return await this.client.createEntity(entity)
  }

  async updateEntity(id: string, entity: UpdateEntity) {
    return await this.client.updateEntity(id, entity)
  }

  async deleteEntity(id: string) {
    return await this.client.deleteEntity(id)
  }

  // Comments
  async fetchEntityComments(id: string) {
    return await this.client.listEntityComments(id)
  }

  get pendingComments(): ListedComment[] {
    return this.pendingCommentsData!
  }

  async getComment(id: string) {
    return await this.client.getComment(id)
  }

  async createComment(comment: NewComment) {
    return await this.client.createComment(comment)
  }

  async updateComment(id: string, comment: UpdateComment) {
    return await this.client.updateComment(id, comment)
  }

  async deleteComment(id: string) {
    return await this.client.deleteComment(id)
  }

  async registerEntityParent(parentId: string, childId: string): Promise<void> {
    await this.client.registerEntityParent(parentId, childId)
  }

  async removeEntityParent(parentId: string, childId: string): Promise<void> {
    await this.client.removeEntityParent(parentId, childId)
  }
}

const state = reactive(new AppState())
export default state
