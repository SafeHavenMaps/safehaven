import type { Middleware } from 'openapi-fetch'

export default function createAuthMiddleware(
  plainToken: string,
  authToken: string,
  onAuthError: () => Promise<void>,
): Middleware {
  let usedAuthToken = authToken

  return {

    async onRequest(params) {
      const { request } = params
      request.headers.set('X-SH-Plain-AccessToken', plainToken)
      request.headers.set('Authorization', `Bearer ${usedAuthToken}`)
      return request
    },

    async onResponse(params) {
      const { response } = params
      if (response.status === 401) {
        await onAuthError()
      }
      if (response.headers.has('x-sh-renew-token')) {
        usedAuthToken = response.headers.get('x-sh-renew-token')!
      }
      return response
    },
  }
}
