import { reactive } from "vue";
import type * as api from "~/lib/api";
import useClient from "~/lib/client_admin";
const client = useClient();
import { transform, transformExtent } from "ol/proj.js";
import type { Coordinate } from "ol/coordinate";
import type { Extent } from "ol/extent";

type BootstrapResponse = api.components["schemas"]["BootstrapResponse"];

type ViewData = {
  entities: (api.components["schemas"]["CachedEntity"] & {
    coordinates: Coordinate;
    family: BootstrapResponse["families"][number];
    category: BootstrapResponse["categories"][number];
  })[];
  clusters: (api.components["schemas"]["Cluster"] & {
    coordinates: Coordinate;
  })[];
};

export class AppState {
  initialized = false;

  private familiesData: BootstrapResponse["families"] | null = null;

  private categoriesData: BootstrapResponse["categories"] | null = null;
  private tagsData: BootstrapResponse["tags"] | null = null;
  private mapBootData: BootstrapResponse["cartography_init_config"] | null =
    null;

  private familiesLookupTable: Record<
    string,
    BootstrapResponse["families"][number]
  > = {};
  private categoriesLookupTable: Record<
    string,
    BootstrapResponse["categories"][number]
  > = {};
  private tagsLookupTable: Record<string, BootstrapResponse["tags"][number]> =
    {};

  private viewData: ViewData = {
    entities: [],
    clusters: [],
  };

  get entities() {
    return this.viewData.entities;
  }

  get clusters() {
    return this.viewData.clusters;
  }

  async initWithToken(token: string) {
    const data = await client.bootstrap(token);

    this.familiesData = data.families;
    this.categoriesData = data.categories;
    this.tagsData = data.tags;
    this.mapBootData = data.cartography_init_config;

    this.familiesData.forEach((family) => {
      this.familiesLookupTable[family.id] = family;
    });

    this.categoriesData.forEach((category) => {
      this.categoriesLookupTable[category.id] = category;
    });

    this.tagsData.forEach((tag) => {
      this.tagsLookupTable[tag.id] = tag;
    });

    this.initialized = true;
  }

  get families(): BootstrapResponse["families"] {
    return this.familiesData!;
  }

  get categories(): BootstrapResponse["categories"] {
    return this.categoriesData!;
  }

  get tags(): BootstrapResponse["tags"] {
    return this.tagsData!;
  }

  get mapBoot(): BootstrapResponse["cartography_init_config"] {
    return this.mapBootData!;
  }
}

const state = reactive(new AppState());
export default state;
