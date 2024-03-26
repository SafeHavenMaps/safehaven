import type { paths } from "./api";
import createClient from "openapi-fetch";

export default function useClient() {
  const config = useRuntimeConfig();
  const apiUrl = config.public.apiUrl;
  const client = createClient<paths>({ baseUrl: apiUrl });

  return client;
}
