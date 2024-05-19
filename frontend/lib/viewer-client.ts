import createClient from 'openapi-fetch'
import type { paths } from './api'
import createAuthMiddleware from './viewer-auth-middleware'
import type { FetchedEntity } from '~/lib'

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
    ) {
      const { data, error } = await rawClient.POST('/api/map/view', {
        body: {
          xmin: rectangle.xmin,
          ymin: rectangle.ymin,
          xmax: rectangle.xmax,
          ymax: rectangle.ymax,
          zoom_level: zoomLevel,
          family_id: familyId,
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
  }
}
