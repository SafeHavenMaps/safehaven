import createClient from 'openapi-fetch'
import type { paths } from './api'
import createAuthMiddleware from './admin-auth-middleware'
import type {
  NewOrUpdateFamily,
  Category,
  NewOrUpdateCategory,
  Tag,
  NewOrUpdateTag,
  AdminEntity,
  AdminListedEntity,
  AdminNewOrUpdateEntity,
  AdminComment,
  AdminListedComment,
  AdminNewOrUpdateComment,
  NewOrUpdateAccessToken,
  User,
  NewOrUpdatedUser,
  SafeHavenOptions,
  ConfigurationOption,
  AdminSearchRequestBody,
  AccessTokenStats,
  AdminEntityWithRelations,
  SafeHavenVersion,
  Family,
  AdminPaginatedCachedEntities,
  AccessToken,
} from '~/lib'

// client as a closure
export default function useClient() {
  let alreadyLoggedOut = false

  const rawClient = createClient<paths>({
    baseUrl: '/',
    credentials: 'include', // Ensures cookies are sent with the request
  })

  // We declare logout first as it used as a callback by the middleware
  async function logout() {
    if (alreadyLoggedOut) return
    alreadyLoggedOut = true

    try {
      await rawClient.DELETE('/api/admin/session')
    }
    catch {
      // Ignore errors
    }
    const currentUrl = window.location.href
    // Do not redirect using navigateTo, use browser's native redirect
    window.location.href = `/admin/login?redirect=${encodeURIComponent(currentUrl)}`
  }

  // Install auth middleware to the stack.
  const authMiddleware = createAuthMiddleware(logout)
  rawClient.use(authMiddleware)

  return {
    rawClient: rawClient,

    // Auth session
    async login(username: string, password: string, remember_me: boolean) {
      const { data, error } = await rawClient.POST('/api/admin/session', {
        body: { password: password, username: username, remember_me: remember_me },
      })
      if (error) throw error
      alreadyLoggedOut = false
      return data.is_admin
    },

    logout,

    async check_login() {
      const { data, error } = await rawClient.GET('/api/admin/session')
      if (error) return null
      return data
    },

    // Home stats
    async getStats() {
      const { data, error } = await this.rawClient.GET('/api/admin/stats')
      if (error) throw error
      return data
    },

    // Options
    async getConfig(): Promise<SafeHavenOptions> {
      const { data, error } = await this.rawClient.GET('/api/admin/options')
      if (error) throw error
      return data
    },

    async getVersionInformation(): Promise<SafeHavenVersion> {
      const { data, error } = await this.rawClient.GET('/api/version')
      if (error) throw error
      return data
    },

    async updateConfig(name: string, config: ConfigurationOption): Promise<SafeHavenOptions> {
      const { data, error } = await this.rawClient.PUT(`/api/admin/options/{name}`, {
        params: { path: { name } },
        body: config,
      })
      if (error) throw error
      return data
    },

    async deleteConfig(name: string): Promise<SafeHavenOptions> {
      const { data, error } = await this.rawClient.DELETE(`/api/admin/options/{name}`, {
        params: { path: { name } },
      })
      if (error) throw error
      return data
    },

    // Users
    async listUsers(): Promise<User[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/users')
      if (error) throw error
      return data
    },

    async createUser(newUser: NewOrUpdatedUser): Promise<User> {
      const { data, error } = await this.rawClient.POST('/api/admin/users', {
        body: newUser,
      })
      if (error) throw error
      return data
    },

    async getUser(id: string): Promise<User> {
      const { data, error } = await this.rawClient.GET(`/api/admin/users/{id}`,
        {
          params: { path: { id } },
        })
      if (error) throw error
      return data
    },

    async changeSelfPassword(updatedUser: NewOrUpdatedUser): Promise<User> {
      const { data, error } = await this.rawClient.PUT('/api/admin/users/self/password', {
        body: updatedUser,
      })
      if (error) throw error
      return data
    },

    async updateUser(id: string, updatedUser: NewOrUpdatedUser): Promise<User> {
      const { data, error } = await this.rawClient.PUT(`/api/admin/users/{id}`, {
        params: { path: { id } },
        body: updatedUser,
      })
      if (error) throw error
      return data
    },

    async deleteUser(id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/users/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async listFamilies(): Promise<Family[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/families')
      if (error) throw error
      return data
    },

    async getFamily(id: string): Promise<Family> {
      const { data, error } = await this.rawClient.GET('/api/admin/families/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async createFamily(family: NewOrUpdateFamily): Promise<Family> {
      const { data, error } = await this.rawClient.POST('/api/admin/families', { body: family })
      if (error) throw error
      return data
    },

    async updateFamily(id: string, family: NewOrUpdateFamily): Promise<Family> {
      const { data, error } = await this.rawClient.PUT('/api/admin/families/{id}', {
        body: family,
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async deleteFamily(id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/families/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async getEntity(id: string): Promise<AdminEntityWithRelations> {
      const { data, error } = await this.rawClient.GET('/api/admin/entities/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async searchEntities(
      pagination: { page: number, page_size: number },
      search_request: AdminSearchRequestBody,
    ): Promise<AdminPaginatedCachedEntities> {
      const { data, error } = await this.rawClient.POST('/api/admin/entities/search', {
        params: { query: pagination },
        body: search_request,
      })
      if (error) throw error
      return data
    },

    async createEntity(entity: AdminNewOrUpdateEntity): Promise<AdminEntity> {
      const { data, error } = await this.rawClient.POST('/api/admin/entities', { body: entity })
      if (error) throw error
      return data
    },

    async updateEntity(id: string, entity: AdminNewOrUpdateEntity): Promise<AdminEntity> {
      const { data, error } = await this.rawClient.PUT('/api/admin/entities/{id}', {
        body: entity,
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async deleteEntity(id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/entities/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async getCategory(id: string): Promise<Category> {
      const { data, error } = await this.rawClient.GET('/api/admin/categories/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async listCategories(): Promise<Category[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/categories')
      if (error) throw error
      return data
    },

    async createCategory(category: NewOrUpdateCategory): Promise<Category> {
      const { data, error } = await this.rawClient.POST('/api/admin/categories', { body: category })
      if (error) throw error
      return data
    },

    async updateCategory(id: string, category: NewOrUpdateCategory): Promise<Category> {
      const { data, error } = await this.rawClient.PUT('/api/admin/categories/{id}', {
        body: category,
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async deleteCategory(id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/categories/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async getTag(id: string): Promise<Tag> {
      const { data, error } = await this.rawClient.GET('/api/admin/tags/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async listTags(): Promise<Tag[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/tags')
      if (error) throw error
      return data
    },

    async createTag(tag: NewOrUpdateTag): Promise<Tag> {
      const { data, error } = await this.rawClient.POST('/api/admin/tags', { body: tag })
      if (error) throw error
      return data
    },

    async updateTag(id: string, tag: NewOrUpdateTag): Promise<Tag> {
      const { data, error } = await this.rawClient.PUT('/api/admin/tags/{id}', {
        body: tag,
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async deleteTag(id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/tags/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async getComment(id: string): Promise<AdminComment> {
      const { data, error } = await this.rawClient.GET('/api/admin/comments/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async createComment(comment: AdminNewOrUpdateComment): Promise<AdminComment> {
      const { data, error } = await this.rawClient.POST('/api/admin/comments', { body: comment })
      if (error) throw error
      return data
    },

    async updateComment(id: string, comment: AdminNewOrUpdateComment): Promise<AdminComment> {
      const { data, error } = await this.rawClient.PUT('/api/admin/comments/{id}', {
        body: comment,
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async deleteComment(id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/comments/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async listPendingComments(): Promise<AdminListedComment[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/comments/pending')
      if (error) throw error
      return data
    },

    async listPendingEntities(): Promise<AdminListedEntity[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/entities/pending')
      if (error) throw error
      return data
    },

    async listEntityComments(id: string): Promise<AdminComment[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/entities/{id}/comments', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async registerEntityParent(parent_id: string, child_id: string): Promise<void> {
      const { data, error } = await this.rawClient.POST('/api/admin/entities/{parent_id}/parent/{child_id}', {
        params: { path: { parent_id, child_id } },
      })
      if (error) throw error
      return data
    },

    async removeEntityParent(parent_id: string, child_id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/entities/{parent_id}/parent/{child_id}', {
        params: { path: { parent_id, child_id } },
      })
      if (error) throw error
      return data
    },

    async listAccessTokens(): Promise<AccessToken[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/access_tokens')
      if (error) throw error
      return data
    },

    async createAccessToken(token: NewOrUpdateAccessToken): Promise<AccessToken> {
      const { data, error } = await this.rawClient.POST('/api/admin/access_tokens', { body: token })
      if (error) throw error
      return data
    },

    async getAccessToken(id: string): Promise<AccessToken> {
      const { data, error } = await this.rawClient.GET('/api/admin/access_tokens/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async getAccessTokenStats(id: string): Promise<AccessTokenStats> {
      const { data, error } = await this.rawClient.GET('/api/admin/access_tokens/{id}/stats', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async updateAccessToken(id: string, token: NewOrUpdateAccessToken): Promise<AccessToken> {
      const { data, error } = await this.rawClient.PUT('/api/admin/access_tokens/{id}', {
        body: token,
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async deleteAccessToken(id: string): Promise<void> {
      const { data, error } = await this.rawClient.DELETE('/api/admin/access_tokens/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async getEntitiesCommentsCounts() {
      const { data, error } = await this.rawClient.GET('/api/admin/stats/count-comments-entities')
      if (error) throw error
      return data
    },
  }
}
