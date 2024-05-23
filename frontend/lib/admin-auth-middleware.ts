import type { Middleware } from 'openapi-fetch'

// Openapi-fetch middleware function to check for 401 errors on admin pages and redirect to the admin login page
export default function createAuthMiddleware(): Middleware {
  return {
    async onResponse(response, _options) {
      if (response.status === 401) {
        navigateTo('/admin/login?redirect=${encodeURIComponent(currentUrl)}')
      }
      return response
    },
  }
}
