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
        :projection="state.mapBoot.display_projection"
      />

      <ol-tile-layer>
        <ol-source-osm />
      </ol-tile-layer>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import state from "~/lib/state";
import { transform, transformExtent } from "ol/proj.js";
import type Map from "ol/Map";

// Init state with url token
const route = useRoute();
const token = route.params.token as string;
await state.initWithToken(token); // TODO: Redirect to 404 if token is invalid

// Init map with center and zoom
const center = transform(
  [state.mapBoot.center_lng, state.mapBoot.center_lat],
  "EPSG:4326",
  state.mapBoot.display_projection,
);
const zoom = state.mapBoot.zoom;

// Properties to access the map and its view
const mapRef = ref<{ map: Map }>();
let map: Map | null = null;

onMounted(() => {
  map = mapRef.value!.map;
});

async function getCurrentCoordinates() {
  const extent = map!.getView().calculateExtent(map!.getSize());
  const transformedExtent = transformExtent(
    extent,
    state.mapBoot.display_projection,
    "EPSG:4326",
  );
  return {
    upperLeft: [transformedExtent[0], transformedExtent[3]],
    lowerRight: [transformedExtent[2], transformedExtent[1]],
  };
}

async function onMapMoveEnd() {
  const coordinates = await getCurrentCoordinates();
  console.log("Current coordinates", coordinates);
}
</script>

<style scoped lang="scss">
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>
