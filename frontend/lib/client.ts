import type { paths } from "./api";
import createClient from "openapi-fetch";

export default function useClient() {
  const config = useRuntimeConfig();
  const apiUrl = config.public.apiUrl;
  const client = createClient<paths>({ baseUrl: apiUrl });

  return {
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
      return data;
    },
  };
}
