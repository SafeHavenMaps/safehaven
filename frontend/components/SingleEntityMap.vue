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
        projection="EPSG:3857"
      />

      <ol-tile-layer>
        <ol-source-osm />
      </ol-tile-layer>

      <ol-overlay
        v-for="(coord, idx) in props.coordinates"
        :key="idx"
        :position="coord"
        :stop-event="true"
      >
        <ViewerMapMarker
          :callback-item="null"
          :width="24"
          :height="38"
          :highlighted="false"
          :border-color="props.borderColor"
          :fill-color="props.fillColor"
          :icon-hash="props.iconHash"
        />
      </ol-overlay>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import type Map from 'ol/Map'
import type { Coordinate } from 'ol/coordinate'

const mapRef = ref<{ map: Map }>()
let map: Map | null = null
onMounted(() => {
  map = mapRef.value!.map
  const view = map.getView()

  if (props.coordinates.length === 1) {
    view.setCenter(props.coordinates[0])
    view.setZoom(props.zoom)
    view.setMinZoom(props.locked ? props.zoom : 2)
    view.setMaxZoom(props.locked ? props.zoom : 20)
  }
  else {
    const extent = [Infinity, Infinity, -Infinity, -Infinity]
    props.coordinates.forEach((coord) => {
      extent[0] = Math.min(extent[0], coord[0])
      extent[1] = Math.min(extent[1], coord[1])
      extent[2] = Math.max(extent[2], coord[0])
      extent[3] = Math.max(extent[3], coord[1])
    })
    // Add padding to the extent
    const paddingParcentage = 50 / 100
    const dx = (extent[2] - extent[0]) * paddingParcentage
    const dy = (extent[3] - extent[1]) * paddingParcentage
    extent[0] -= dx
    extent[1] -= dy
    extent[2] += dx
    extent[3] += dy

    view.fit(extent)
  }
})

const props = defineProps<{
  coordinates: Coordinate[]
  borderColor?: string | undefined
  fillColor?: string | undefined
  iconHash?: string | null | undefined
  zoom: number
  locked: boolean
}>()
</script>

<style scoped>
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>
