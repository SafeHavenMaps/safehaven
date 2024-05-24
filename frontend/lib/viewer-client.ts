import createClient from 'openapi-fetch'
import type { paths } from './api'
import createAuthMiddleware from './viewer-auth-middleware'
import type { FetchedEntity, PaginatedCachedEntities, PaginatedCachedEntitiesWithLocation } from '~/lib'

export default function useClient() {
  const rawClient = createClient<paths>({ baseUrl: '/' })

  return {
    rawClient: rawClient,

    async checkStatus() {
      const { data, error } = await rawClient.GET('/api/status')
      if (error) throw error
      return data
    },

    async bootstrap(token: string) {
      const { data, error } = await rawClient.GET('/api/bootstrap/{token}', {
        params: { path: { token }, query: { referrer: document.referrer } },
      })
      if (error) throw error

      // Install auth middleware to the stack. If it fails, ejects it.
      const authMiddleware = createAuthMiddleware(data.signed_token, () => {
        rawClient.eject(authMiddleware)
        // ToDo: Handle lifecycle of the app when the token is invalid
        // Maybe refresh it using the bootstrapped token?
        // For now, the only fix is refreshing the page for the user
      })
      rawClient.use(authMiddleware)

      return data
    },

    async getEntitiesWithinBounds(
      rectangle: {
        xmin: number
        ymin: number
        xmax: number
        ymax: number
      },
      zoomLevel: number,
      familyId: string,
      activeCategories: string[],
      activeRequiredTags: string[],
      activeHiddenTags: string[],
    ) {
      const { data, error } = await rawClient.POST('/api/map/view', {
        body: {
          xmin: rectangle.xmin,
          ymin: rectangle.ymin,
          xmax: rectangle.xmax,
          ymax: rectangle.ymax,
          zoom_level: zoomLevel,
          family_id: familyId,
          active_categories: activeCategories,
          active_required_tags: activeRequiredTags,
          active_hidden_tags: activeHiddenTags,
        },
      })
      if (error) throw error

      return data
    },

    async fetchEntity(id: string): Promise<FetchedEntity> {
      const { data, error } = await rawClient.GET('/api/map/entities/{id}', {
        params: { path: { id } },
      })
      if (error) throw error

      return data
    },

    async searchEntitiesWithLocations(
      query: string,
      familyId: string,
      activeCategories: string[],
      activeRequiredTags: string[],
      activeHiddenTags: string[],
    ): Promise<PaginatedCachedEntitiesWithLocation> {
      const { data, error } = await rawClient.POST('/api/map/search', {
        body: {
          search_query: query,
          family_id: familyId,
          page: 1,
          page_size: 5,
          active_categories: activeCategories,
          active_required_tags: activeRequiredTags,
          active_hidden_tags: activeHiddenTags,
          require_locations: true,
        },
      })
      if (error) throw error

      return {
        ...data,
        entities: data.entities.map(entity => ({
          ...entity,
          web_mercator_x: entity.web_mercator_x!,
          web_mercator_y: entity.web_mercator_y!,
        })),
      }
    },

    async searchEntities(
      query: string,
      familyId: string,
      activeCategories: string[],
      activeRequiredTags: string[],
      activeHiddenTags: string[],
    ): Promise<PaginatedCachedEntities> {
      const { data, error } = await rawClient.POST('/api/map/search', {
        body: {
          search_query: query,
          family_id: familyId,
          page: 1,
          page_size: 5,
          active_categories: activeCategories,
          active_required_tags: activeRequiredTags,
          active_hidden_tags: activeHiddenTags,
          require_locations: false,
        },
      })
      if (error) throw error

      return data
    },
  }
}
