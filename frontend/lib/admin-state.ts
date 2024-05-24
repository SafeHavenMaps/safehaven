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
  NewUser,
  ChangePassword,
} from '~/lib'

const client = useClient()

export class AppState {
  private optionsData: SafeHavenOptions | null = null
  private usersData: Family[] | null = null
  private familiesData: Family[] | null = null
  private categoriesData: Category[] | null = null
  private tagsData: Tag[] | null = null
  private accessTokensData: AccessToken[] | null = null
  private pendingEntitiesData: ListedEntity[] | null = null
  private pendingCommentsData: ListedComment[] | null = null

  get hasSafeMode() {
    return !!this.optionsData.safe_mode?.enabled
  }

  get safeMode() {
    return this.optionsData.safe_mode!.enabled
  }

  async login(username: string, password: string, remember_me: boolean) {
    await client.login(username, password, remember_me)
    return true
  }

  async logout(username: string, password: string, remember_me: boolean) {
    await client.login(username, password, remember_me)
    navigateTo('/admin/login?redirect=${encodeURIComponent(currentUrl)}')
  }

  async check_login(): Promise<boolean> {
    return await client.check_login()
  }

  async fetchConfig(): Promise<void> {
    this.optionsData = await client.getConfig()
  }

  get options(): SafeHavenOptions | null {
    return this.optionsData
  }

  async updateConfig(name: string, config: ConfigurationOption): Promise<void> {
    const updatedConfig = await client.updateConfig(name, config)
    this.optionsData = updatedConfig
  }

  // Users
  async fetchUsers(): Promise<void> {
    this.usersData = await client.listUsers()
  }

  get users(): User[] {
    return this.usersData!
  }

  async createUser(newUser: NewUser): Promise<void> {
    const createdUser = await client.createUser(newUser)
    this.usersData!.push(createdUser)
  }

  async getUser(id: string): Promise<User> {
    return await client.getUser(id)
  }

  async updateUserPassword(id: string, newPasswordDetails: ChangePassword): Promise<void> {
    const updatedUser = await client.changeUserPassword(id, newPasswordDetails)
    const index = this.usersData!.findIndex(u => u.id === id)
    if (index !== -1) {
      this.usersData![index] = updatedUser
    }
  }

  async updateSelfPassword(newPasswordDetails: ChangePassword): Promise<void> {
    const updatedUser = await client.changeSelfPassword(newPasswordDetails)
    const selfIndex = this.usersData!.findIndex(u => u.id === updatedUser.id)
    if (selfIndex !== -1) {
      this.usersData![selfIndex] = updatedUser
    }
  }

  async deleteUser(id: string): Promise<void> {
    await client.deleteUser(id)
    this.usersData = this.usersData!.filter(u => u.id !== id)
  }

  // Families
  async fetchFamilies() {
    this.familiesData = await client.listFamilies()
  }

  get families(): Family[] {
    return this.familiesData!
  }

  async createFamily(family: NewOrUpdateFamily) {
    const newFamily = await client.createFamily(family)
    this.familiesData!.push(newFamily)
  }

  async updateFamily(id: string, family: NewOrUpdateFamily) {
    const updatedFamily = await client.updateFamily(id, family)
    const index = this.familiesData!.findIndex(f => f.id === id)
    if (index !== -1) {
      this.familiesData![index] = updatedFamily
    }
  }

  async deleteFamily(id: string) {
    await client.deleteFamily(id)
    this.familiesData = this.familiesData!.filter(f => f.id !== id)
  }

  // Categories
  async fetchCategories() {
    this.categoriesData = await client.listCategories()
  }

  get categories(): Category[] {
    return this.categoriesData!
  }

  async createCategory(category: NewCategory) {
    const newCategory = await client.createCategory(category)
    this.categoriesData!.push(newCategory)
  }

  async updateCategory(id: string, category: UpdateCategory) {
    const updatedCategory = await client.updateCategory(id, category)
    const index = this.categoriesData!.findIndex(c => c.id === id)
    if (index !== -1) {
      this.categoriesData![index] = updatedCategory
    }
  }

  async deleteCategory(id: string) {
    await client.deleteCategory(id)
    this.categoriesData = this.categoriesData!.filter(c => c.id !== id)
  }

  // Tags
  async fetchTags() {
    this.tagsData = await client.listTags()
  }

  get tags(): Tag[] {
    return this.tagsData!
  }

  async createTag(tag: NewOrUpdateTag) {
    const newTag = await client.createTag(tag)
    this.tagsData!.push(newTag)
  }

  async updateTag(id: string, tag: NewOrUpdateTag) {
    const updatedTag = await client.updateTag(id, tag)
    const index = this.tagsData!.findIndex(t => t.id === id)
    if (index !== -1) {
      this.tagsData![index] = updatedTag
    }
  }

  async deleteTag(id: string) {
    await client.deleteTag(id)
    this.tagsData = this.tagsData!.filter(t => t.id !== id)
  }

  // Access Tokens
  async fetchAccessTokens() {
    this.accessTokensData = await client.listAccessTokens()
  }

  get accessTokens(): AccessToken[] {
    return this.accessTokensData!
  }

  async createAccessToken(token: NewOrUpdateAccessToken) {
    const newToken = await client.createAccessToken(token)
    this.accessTokensData!.push(newToken)
  }

  async updateAccessToken(id: string, token: NewOrUpdateAccessToken) {
    const updatedToken = await client.updateAccessToken(id, token)
    const index = this.accessTokensData!.findIndex(t => t.id === id)
    if (index !== -1) {
      this.accessTokensData![index] = updatedToken
    }
  }

  async deleteAccessToken(id: string) {
    await client.deleteAccessToken(id)
    this.accessTokensData = this.accessTokensData!.filter(t => t.id !== id)
  }

  // Entities
  async fetchEntities() {
    this.pendingEntitiesData = await client.listPendingEntities()
  }

  get entities(): ListedEntity[] {
    return this.pendingEntitiesData!
  }

  async getEntity(id: string) {
    return await client.getEntity(id)
  }

  async createEntity(entity: NewEntity) {
    return await client.createEntity(entity)
  }

  async updateEntity(id: string, entity: UpdateEntity) {
    return await client.updateEntity(id, entity)
  }

  async deleteEntity(id: string) {
    return await client.deleteEntity(id)
  }

  // Comments
  async fetchEntityComments(id: string) {
    return await client.listEntityComments(id)
  }

  get pendingComments(): ListedComment[] {
    return this.pendingCommentsData!
  }

  async getComment(id: string) {
    return await client.getComment(id)
  }

  async createComment(comment: NewComment) {
    return await client.createComment(comment)
  }

  async updateComment(id: string, comment: UpdateComment) {
    return await client.updateComment(id, comment)
  }

  async deleteComment(id: string) {
    return await client.deleteComment(id)
  }

  async registerEntityParent(parentId: string, childId: string): Promise<void> {
    await client.registerEntityParent(parentId, childId)
  }

  async removeEntityParent(parentId: string, childId: string): Promise<void> {
    await client.removeEntityParent(parentId, childId)
  }
}

const state = reactive(new AppState())
export default state
