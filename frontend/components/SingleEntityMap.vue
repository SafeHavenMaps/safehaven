<template>
  <div id="map_container">
    <ol-map
      id="map"
      ref="mapRef"
      :load-tiles-while-animating="true"
      :load-tiles-while-interacting="true"
      :interactions="props.locked ? [] : undefined"
      :controls="props.locked ? [] : undefined"
    >
      <ol-view
        :center="props.coordinates"
        :zoom="zoom"
        :min-zoom="locked ? zoom : 2"
        :max-zoom="locked ? zoom : 18"
        projection="EPSG:3857"
      />

      <ol-tile-layer>
        <ol-source-osm />
      </ol-tile-layer>

      <ol-overlay
        :position="props.coordinates"
        :stop-event="false"
      >
        <ViewerMapMarker
          :callback-item="null"
          :width="24"
          :height="38"
          :fill="props.fillColor"
          :stroke="props.borderColor"
          :highlighted="false"
          :icon-url="iconUrl"
        />
      </ol-overlay>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import type { Coordinate } from 'ol/coordinate'

const props = defineProps<{
  coordinates: Coordinate
  fillColor: string
  borderColor: string
  iconHash: string | null | undefined
  zoom: number
  locked: boolean
}>()

const iconUrl = computed(() => {
  return props.iconHash ? `/api/icons/${props.iconHash}` : null
})
</script>

<style scoped lang="scss">
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>
