import type { paths } from "./api";
import createClient from "openapi-fetch";

const client = createClient<paths>({ baseUrl: "/" });
export default client;
