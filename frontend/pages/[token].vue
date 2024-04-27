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
        projection="EPSG:3857"
      />

      <ol-tile-layer>
        <ol-source-osm />
      </ol-tile-layer>

      <ol-overlay
        :position="entity.coordinates"
        v-for="entity in state.view.entities"
        :key="entity"
      >
        <MapMarker
          :width="24"
          :height="38"
          :fill="entity.category.fill_color"
          :stroke="entity.category.border_color"
        />
      </ol-overlay>


      <ol-overlay
        :position="entity.coordinates"
        v-for="entity in state.view.clusters"
        :key="entity"
      >
        <MapMarker
          :width="24"
          :height="38"
          fill="black"
          stroke="black"
        />
      </ol-overlay>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import state from "~/lib/state";
import type Map from "ol/Map";

// Init state with url token
const route = useRoute();
const token = route.params.token as string;
await state.initWithToken(token); // TODO: Redirect to 404 if token is invalid

let center = state.startCenter();
let zoom = state.startZoom();

// Properties to access the map and its view
const mapRef = ref<{ map: Map }>();
let map: Map | null = null;

onMounted(() => {
  map = mapRef.value!.map;
});

async function onMapMoveEnd() {
  const extent = map!.getView().getViewStateAndExtent().extent;
  const currentZoom = map!.getView().getZoom()!;
  state.refreshView(extent, currentZoom);
  console.log(currentZoom);
}
</script>

<style scoped lang="scss">
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>
