import type { Middleware } from 'openapi-fetch'

export default function createAuthMiddleware(
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
