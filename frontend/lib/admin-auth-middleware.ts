import type { Middleware } from 'openapi-fetch'

// Openapi-fetch middleware function to check for 401 errors on admin pages and redirect to the admin login page
export default function createAuthMiddleware(logout_callback: () => Promise<void>): Middleware {
  return {
    async onResponse(response, _options) {
      if (response.status === 401) {
        await logout_callback()
      }
      return response
    },
  }
}
