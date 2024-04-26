import { reactive } from "vue";
import type * as api from "~/lib/api";
import useClient from "~/lib/client";
const client = useClient();

type BootstrapResponse = api.components["schemas"]["BootstrapResponse"];
type ViewResponse = api.components["schemas"]["CachedEntity"][];

export class AppState {
  initialized = false;

  private familiesData: BootstrapResponse["families"] | null = null;
  private categoriesData: BootstrapResponse["categories"] | null = null;
  private tagsData: BootstrapResponse["tags"] | null = null;
  private mapBootData: BootstrapResponse["map_boot"] | null = null;

  private viewData: ViewResponse | null = null;

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

  get view(): ViewResponse {
    if (!this.initialized) {
      throw new Error("App state not initialized");
    }
    return this.viewData!;
  }

  async initWithToken(token: string) {
    const data = await client.bootstrap(token);

    this.familiesData = data.families;
    this.categoriesData = data.categories;
    this.tagsData = data.tags;
    this.mapBootData = data.map_boot;

    this.initialized = true;
  }

  async refreshView(
    upperLeft: [number, number],
    lowerRight: [number, number]
  ) {
    if (!this.initialized) {
      throw new Error("App state not initialized");
    }
    this.viewData = await client.getEntitiesWithinBounds(upperLeft, lowerRight);
  }
}

const state = reactive(new AppState());
export default state;
