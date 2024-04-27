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
  private mapBootData: BootstrapResponse["map_boot"] | null = null;

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
    this.mapBootData = data.map_boot;

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

  get mapBoot(): BootstrapResponse["map_boot"] {
    return this.mapBootData!;
  }

  startCenter() {
    // The configuration files is in WGS84 (EPSG:4326)
    // The map is in Web Mercator (EPSG:3857)
    // We need to transform the coordinates
    return transform(
      [state.mapBoot.center_lng, state.mapBoot.center_lat],
      "EPSG:4326", // WGS84
      "EPSG:3857" // Web Mercator
    );
  }

  startZoom() {
    return state.mapBoot.zoom;
  }

  async refreshView(extent: Extent, zoomLevel: number) {
    const zoom = Math.round(zoomLevel);
    const newViewData = await client.getEntitiesWithinBounds(
      {
        xmin: extent[0],
        ymin: extent[1],
        xmax: extent[2],
        ymax: extent[3],
      },
      zoom
    );

    // Step 1: Identify and filter out entities that are no longer present
    const existingEntityIds = new Set(newViewData.entities.map((ne) => ne.id));
    this.viewData.entities = this.viewData.entities.filter((e) =>
      existingEntityIds.has(e.id)
    );

    // Step 2: Add new entities that are not already in viewData
    const currentEntityIds = new Set(this.viewData.entities.map((e) => e.id));
    const newEntities = newViewData.entities.filter(
      (ne) => !currentEntityIds.has(ne.id)
    );
    this.viewData.entities.push(
      ...newEntities.map((entity) => ({
        ...entity,
        coordinates: [entity.web_mercator_x, entity.web_mercator_y],
        family: this.familiesLookupTable[entity.family_id],
        category: this.categoriesLookupTable[entity.category_id],
      }))
    );

    // Step 3: Identify and filter out clusters that are no longer present
    const existingClusterIds = new Set(newViewData.clusters.map((nc) => nc.id));
    this.viewData.clusters = this.viewData.clusters.filter((c) =>
      existingClusterIds.has(c.id)
    );

    // Step 4: Add new clusters that are not already in viewData
    const currentClusterIds = new Set(this.viewData.clusters.map((c) => c.id));
    const newClusters = newViewData.clusters.filter(
      (nc) => !currentClusterIds.has(nc.id)
    );
    this.viewData.clusters.push(
      ...newClusters.map((cluster) => ({
        ...cluster,
        coordinates: [cluster.center_x, cluster.center_y],
      }))
    );
  }
}

const state = reactive(new AppState());
export default state;
