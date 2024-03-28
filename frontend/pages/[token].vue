<template>
  <div id="map_container">
    <ol-map
      :load-tiles-while-animating="true"
      :load-tiles-while-interacting="true"
      style="width: 100%; height: 100%"
    >
      <ol-view
        ref="view"
        :center="center"
        :zoom="zoom"
        :projection="projection"
      />

      <ol-tile-layer>
        <ol-source-osm />
      </ol-tile-layer>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import state from "~/lib/state";
import { transform } from "ol/proj.js";
const projection = "EPSG:3857";

const route = useRoute();
const token = route.params.token as string;
await state.initWithToken(token);

const center = transform(
  [state.mapBoot.center_lng, state.mapBoot.center_lat],
  "EPSG:4326",
  projection
);
const zoom = state.mapBoot.zoom;
</script>

<style scoped lang="scss">
#map_container {
  width: 100%;
  height: 100%;
}
</style>
