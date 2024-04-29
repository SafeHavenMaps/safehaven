<template>
  <div id="map_container">
    <ol-map
      id="map"
      ref="mapRef"
      :load-tiles-while-animating="true"
      :load-tiles-while-interacting="true"
      @moveend="onMapMoveEnd"
    >
      <ol-view
        ref="viewRef"
        :center="center"
        :zoom="zoom"
        :max-zoom="20"
        projection="EPSG:3857"
      />

      <ol-tile-layer>
        <ol-source-osm />
      </ol-tile-layer>

      <ol-overlay
        :position="entity.coordinates"
        v-for="entity in props.entities"
        :key="entity.id"
      >
        <ClientMapMarker
          :callbackItem="entity"
          :width="24"
          :height="38"
          :fill="entity.category.fill_color"
          :stroke="entity.category.border_color"
          @click="handleEntityClick"
        />
      </ol-overlay>

      <ol-overlay
        :position="cluster.coordinates"
        v-for="cluster in props.clusters"
        :key="cluster"
      >
        <ClientMapCluster
          :callbackItem="cluster"
          :count="cluster.count"
          :seed="cluster.id"
          @click="handleClusterClick(cluster)"
        />
      </ol-overlay>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import type Map from "ol/Map";
import type { Coordinate } from "ol/coordinate";
import type { Extent } from "ol/extent";
import type {
  DisplayableCachedEntity,
  DisplayableCluster,
} from "~/lib";

const props = defineProps<{
  center: Coordinate;
  zoom: number;
  entities: DisplayableCachedEntity[];
  clusters: DisplayableCluster[];
}>();

let zoom = props.zoom;
let center = props.center;

const emit = defineEmits<{
  move: [extent: Extent, zoom: number];
  entityClick: [entity: DisplayableCachedEntity];
}>();

const mapRef = ref<{ map: Map }>();
let map: Map | null = null;
onMounted(() => {
  map = mapRef.value!.map;
});

async function onMapMoveEnd() {
  const extent = map!.getView().getViewStateAndExtent().extent;
  const currentZoom = map!.getView().getZoom()!;
  emit("move", extent, currentZoom);
}

function zoomTo(coordinates: Coordinate) {
  map!.getView().animate({
    center: coordinates,
    zoom: Math.min(map!.getView().getZoom()! + 2, map!.getView().getMaxZoom()!),
    duration: 500,
  });
}

async function handleClusterClick(cluster: DisplayableCluster) {
    zoomTo(cluster.coordinates)
}

async function handleEntityClick(entity: DisplayableCachedEntity) {
  zoomTo(entity.coordinates)
  emit("entityClick", entity);
}
</script>

<style scoped lang="scss">
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>
