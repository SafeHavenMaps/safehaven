import type { paths } from "./api";
import createClient, { type Middleware } from "openapi-fetch";

function createAuthMiddleware(
  authToken: string,
  onAuthError: () => void
): Middleware {
  return {
    async onRequest(req, options) {
      req.headers.set("Authorization", `Bearer ${authToken}`);
      return req;
    },

    async onResponse(res, options) {
      if (res.status === 401) {
        onAuthError();
      }
      return res;
    },
  };
}

export default function useClient() {
  const config = useRuntimeConfig();
  const apiUrl = config.public.apiUrl;
  const client = createClient<paths>({ baseUrl: apiUrl });

  return {
    authenticated: false,
    rawClient: client,

    async checkHealth() {
      const { data } = await client.GET("/api/health");
      return data?.status === "ok";
    },

    async bootstrap(token: string) {
      const { data, error } = await client.GET("/api/bootstrap/{token}", {
        params: { path: { token } },
      });
      if (error) throw error;

      // Install auth middleware to the stack. If it fails, ejects it.
      this.authenticated = true;
      const authMiddleware = createAuthMiddleware(
        data.signed_token,
        () => {
          client.eject(authMiddleware);
          // ToDo: Handle lifecycle of the app when the token is invalid
          // Maybe refresh it using the bootstrapped token?
          // For now, the only fix is refreshing the page for the user
        }
      );
      client.use(authMiddleware);

      return data;
    },

    async getEntitiesWithinBounds(
      upperLeft: [number, number],
      lowerRight: [number, number],
      zoomLevel: number
    ) {
      if (!this.authenticated) {
        throw new Error("Not authenticated");
      }

      const { data, error } = await client.POST("/api/map/view", {
        body: {
          upper_left_lat: upperLeft[0],
          upper_left_lon: upperLeft[1],
          lower_right_lat: lowerRight[0],
          lower_right_lon: lowerRight[1],
          zoom_level: zoomLevel,
        },
      });
      if (error) throw error;

      return data;
    }
  };
}
