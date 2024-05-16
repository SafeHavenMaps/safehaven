import createClient from 'openapi-fetch'
import type { paths } from './api'
import createAuthMiddleware from './auth-middleware'

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

    async bootstrap(username: string, password: string) {
      const { data, error } = await client.POST('/admin/login', {
        body: { password: password, username: username },
      })
      if (error) throw error

      // Install auth middleware to the stack. If it fails, ejects it.
      this.authenticated = true
      const authMiddleware = createAuthMiddleware(data.token, () => {
        client.eject(authMiddleware)
        // ToDo: Handle lifecycle of the app when the token is invalid
        // Maybe refresh it using the bootstrapped token?
        // For now, the only fix is refreshing the page for the user
      })
      client.use(authMiddleware)
      return data
    },
  }
}
