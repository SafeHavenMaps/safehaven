import createClient from 'openapi-fetch'
import type { paths } from './api'
import type { FetchedEntity, ErrorResponse } from '~/lib'

// Decorator that checks authentication for crud api calls and redirect to the login page if non authenticated or unauthorized
function _requiresAuth(_target: ClientService, _key: string, descriptor: PropertyDescriptor): PropertyDescriptor {
  const originalMethod = descriptor.value

  descriptor.value = async function (...args: unknown[]) {
    if (!(this as ClientService).authenticated) {
      navigateTo('/admin/login')
      return
    }

    // Call the original method
    const { data, error } = await originalMethod.apply(this, args)

    if (error?.error_code == '401') {
      navigateTo('/admin/login')
      return
    }

    if (error) throw error
    return data
  }

  return descriptor
}

class ClientService {
  authenticated: boolean
  private rawClient: ReturnType<typeof createClient<paths>>

  constructor() {
    this.authenticated = false
    this.rawClient = createClient<paths>({
      baseUrl: '/',
      credentials: 'include', // Ensures cookies are sent with the request
    })
  }

  async bootstrap(username: string, password: string): Promise<void> {
    const { error } = await this.rawClient.POST('/api/admin/login', {
      body: { password: password, username: username },
    })

    if (error) throw error

    // Set authenticated to true if login is successful
    this.authenticated = true
  }

  // @requiresAuth
  async fetchEntity(id: string): Promise<{ data: FetchedEntity | undefined, error: ErrorResponse | undefined }> {
    const { data, error } = await this.rawClient.GET('/api/map/entities/{id}', {
      params: { path: { id } },
    })

    return { data, error }
  }
}

// Export the function to use the client
export default function useClient(): ClientService {
  return new ClientService()
}
