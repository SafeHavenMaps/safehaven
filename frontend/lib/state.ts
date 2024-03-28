import { reactive } from "vue";
import type * as api from "~/lib/api";
import useClient from "~/lib/client";
const client = useClient();

type BootstrapResponse = api.components["schemas"]["BootstrapResponse"];

export class AppState {
  initialized = false;

  private familiesData: BootstrapResponse["families"] | null = null;
  private categoriesData: BootstrapResponse["categories"] | null = null;
  private tagsData: BootstrapResponse["tags"] | null = null;
  private mapBootData: BootstrapResponse["map_boot"] | null = null;

  get families(): BootstrapResponse["families"] {
    if (!this.initialized) {
      throw new Error("App state not initialized");
    }
    return this.familiesData!;
  }

  get categories(): BootstrapResponse["categories"] {
    if (!this.initialized) {
      throw new Error("App state not initialized");
    }
    return this.categoriesData!;
  }

  get tags(): BootstrapResponse["tags"] {
    if (!this.initialized) {
      throw new Error("App state not initialized");
    }
    return this.tagsData!;
  }

  get mapBoot(): BootstrapResponse["map_boot"] {
    if (!this.initialized) {
      throw new Error("App state not initialized");
    }
    return this.mapBootData!;
  }

  async initWithToken(token: string) {
    const data = await client.bootstrap(token);

    this.familiesData = data.families;
    this.categoriesData = data.categories;
    this.tagsData = data.tags;
    this.mapBootData = data.map_boot;

    this.initialized = true;
  }
}

const state = reactive(new AppState());
export default state;
