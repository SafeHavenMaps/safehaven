import createClient from 'openapi-fetch'
import type { paths } from './api'
import createAuthMiddleware from './admin-auth-middleware'
import type {
  Family,
  Entity,
  Category,
  Tag,
  FetchedEntity,
  ListedEntity,
  ListedComment,
  NewCategory,
  UpdateCategory,
  NewOrUpdateTag,
  NewComment,
  UpdateComment,
  NewEntity,
  UpdateEntity,
  NewOrUpdateAccessToken,
  AccessToken,
  NewOrUpdateFamily,
  SHComment,
  User,
  NewUser,
  ChangePassword,
  SafeHavenOptions,
  ConfigurationOption,
} from '~/lib'

// client as a closure
export default function useClient() {
  const rawClient = createClient<paths>({ baseUrl: '/',
    credentials: 'include', // Ensures cookies are sent with the request
  })

  // Install auth middleware to the stack.
  const authMiddleware = createAuthMiddleware()
  rawClient.use(authMiddleware)

  return {
    rawClient: rawClient,

    // Auth session
    async login(username: string, password: string, remember_me: boolean) {
      const { data, error } = await rawClient.POST('/api/admin/session', {
        body: { password: password, username: username, remember_me: remember_me },
      })
      if (error) throw error
      return data
    },

    async logout() {
      const { data, error } = await rawClient.DELETE('/api/admin/session')
      if (error) throw error
      return data
    },

    async check_login() {
      const { error } = await rawClient.GET('/api/admin/session')
      if (error) return false
      return true
    },

    // Options
    async getConfig(): Promise<SafeHavenOptions> {
      const { data, error } = await this.rawClient.GET('/api/admin/options')
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

    // Users
    async listUsers(): Promise<User[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/users')
      if (error) throw error
      return data
    },

    async createUser(newUser: NewUser): Promise<User> {
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

    async changeSelfPassword(changePassword: ChangePassword): Promise<User> {
      const { data, error } = await this.rawClient.PUT('/api/admin/users/self/password', {
        body: changePassword,
      })
      if (error) throw error
      return data
    },

    async changeUserPassword(id: string, changePassword: ChangePassword): Promise<User> {
      const { data, error } = await this.rawClient.PUT(`/api/admin/users/{id}/password`, {
        params: { path: { id } },
        body: changePassword,
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

    async getEntity(id: string): Promise<FetchedEntity> {
      const { data, error } = await this.rawClient.GET('/api/map/entities/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async searchEntities(query: { search: string, page: number, page_size: number }): Promise<ListedEntity[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/entities/search', {
        params: { query: query },
      })
      if (error) throw error
      return data
    },

    async createEntity(entity: NewEntity): Promise<Entity> {
      const { data, error } = await this.rawClient.POST('/api/admin/entities', { body: entity })
      if (error) throw error
      return data
    },

    async updateEntity(id: string, entity: UpdateEntity): Promise<Entity> {
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

    async createCategory(category: NewCategory): Promise<Category> {
      const { data, error } = await this.rawClient.POST('/api/admin/categories', { body: category })
      if (error) throw error
      return data
    },

    async updateCategory(id: string, category: UpdateCategory): Promise<Category> {
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

    async getComment(id: string): Promise<SHComment> {
      const { data, error } = await this.rawClient.GET('/api/admin/comments/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },

    async createComment(comment: NewComment): Promise<SHComment> {
      const { data, error } = await this.rawClient.POST('/api/admin/comments', { body: comment })
      if (error) throw error
      return data
    },

    async updateComment(id: string, comment: UpdateComment): Promise<SHComment> {
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

    async listPendingComments(): Promise<ListedComment[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/comments/pending')
      if (error) throw error
      return data
    },

    async listPendingEntities(): Promise<ListedEntity[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/entities/pending')
      if (error) throw error
      return data
    },

    async listEntityComments(id: string): Promise<SHComment[]> {
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
  }
}
