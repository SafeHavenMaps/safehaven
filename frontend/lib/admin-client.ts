import createClient from 'openapi-fetch'
import type { paths } from './api'

export default function useClient() {
  const config = useRuntimeConfig()
  const apiUrl = config.public.apiUrl
  const client = createClient<paths>({
    baseUrl: apiUrl,
    credentials: 'include', // Ensures cookies are sent with the request
  })

  return {
    authenticated: false,
    rawClient: client,

    async bootstrap(username: string, password: string) {
      const { error } = await client.POST('/api/admin/login', {
        body: { password: password, username: username },
      })
      // const _token_cookie = response.headers.getSetCookie()[0]

      if (error) throw error
    },
  }
}
