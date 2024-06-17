import type { Middleware } from 'openapi-fetch'

export default function createAuthMiddleware(
  authToken: string,
  onAuthError: () => Promise<void>,
): Middleware {
  return {
    async onRequest(request, _options) {
      request.headers.set('Authorization', `Bearer ${authToken}`)
      return request
    },

    async onResponse(response, _options) {
      if (response.status === 401) {
        await onAuthError()
      }
      return response
    },
  }
}
