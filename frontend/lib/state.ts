import { reactive } from "vue";
import type * as api from "~/lib/api";
import useClient from "~/lib/client";
const client = useClient();
import { transform, transformExtent } from "ol/proj.js";
import type { Coordinate } from "ol/coordinate";
import type { Extent } from "ol/extent";

type BootstrapResponse = api.components["schemas"]["BootstrapResponse"];
type ViewResponse = api.components["schemas"]["EntitiesAndClusters"];

export class AppState {
  initialized = false;

  private familiesData: BootstrapResponse["families"] | null = null;

  private categoriesData: BootstrapResponse["categories"] | null = null;
  private tagsData: BootstrapResponse["tags"] | null = null;
  private mapBootData: BootstrapResponse["map_boot"] | null = null;

  private familiesLookupTable: Record<string, BootstrapResponse["families"][number]> = {};
  private categoriesLookupTable: Record<string, BootstrapResponse["categories"][number]> = {};
  private tagsLookupTable: Record<string, BootstrapResponse["tags"][number]> = {};

  private viewData: ViewResponse = {
    entities: [],
    clusters: [],
  };

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
      "EPSG:3857"  // Web Mercator
    )
  }

  startZoom() {
    return state.mapBoot.zoom;
  }

  get view() {
    return {
      entities: this.viewData.entities.map((entity) => {
        return {
          ...entity,
          coordinates: [
            entity.web_mercator_x,
            entity.web_mercator_y,
          ],
          family: this.familiesLookupTable[entity.family_id],
          category: this.categoriesLookupTable[entity.category_id],
        };
      }),
      clusters: this.viewData.clusters.map((cluster) => {
        return {
          ...cluster,
          coordinates: [
            cluster.center_x,
            cluster.center_y,
          ],
        };
      }),
    };
  }

  async refreshView(extent: Extent, zoomLevel: number) {
    const zoom = Math.round(zoomLevel);
    this.viewData = await client.getEntitiesWithinBounds(
      {
        xmin: extent[0], ymin: extent[1],
        xmax: extent[2], ymax: extent[3],
      },
      zoom
    );
  }
}

const state = reactive(new AppState());
export default state;
