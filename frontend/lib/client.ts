import type { FetchedEntity } from '~/lib'
import type { paths } from './api'
import createClient, { type Middleware } from 'openapi-fetch'

function createAuthMiddleware(
  authToken: string,
  onAuthError: () => void,
): Middleware {
  return {
    async onRequest(req, _options) {
      req.headers.set('Authorization', `Bearer ${authToken}`)
      return req
    },

    async onResponse(res, _options) {
      if (res.status === 401) {
        onAuthError()
      }
      return res
    },
  }
}

export default function useClient() {
  const config = useRuntimeConfig()
  const apiUrl = config.public.apiUrl
  const client = createClient<paths>({ baseUrl: apiUrl })

  return {
    authenticated: false,
    rawClient: client,

    async checkStatus() {
      const { data, error } = await client.GET('/api/status')
      if (error) throw error
      return data
    },

    async bootstrap(token: string) {
      const { data, error } = await client.GET('/api/bootstrap/{token}', {
        params: { path: { token }, query: { referrer: document.referrer } },
      })
      if (error) throw error

      // Install auth middleware to the stack. If it fails, ejects it.
      this.authenticated = true
      const authMiddleware = createAuthMiddleware(data.signed_token, () => {
        client.eject(authMiddleware)
        // ToDo: Handle lifecycle of the app when the token is invalid
        // Maybe refresh it using the bootstrapped token?
        // For now, the only fix is refreshing the page for the user
      })
      client.use(authMiddleware)

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
      if (!this.authenticated) {
        throw new Error('Not authenticated')
      }

      const { data, error } = await client.POST('/api/map/view', {
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
      if (!this.authenticated) {
        throw new Error('Not authenticated')
      }

      const { data, error } = await client.GET('/api/map/entities/{id}', {
        params: { path: { id } },
      })
      if (error) throw error

      return data
    },
  }
}
