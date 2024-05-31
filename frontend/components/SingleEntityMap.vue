<template>
  <div id="map_container">
    <ol-map
      id="map"
      ref="mapRef"
      :load-tiles-while-animating="true"
      :load-tiles-while-interacting="true"
      :interactions="[]"
      :controls="[]"
    >
      <ol-view
        :center="entity.coordinates"
        :zoom="13"
        :min-zoom="13"
        :max-zoom="13"
        projection="EPSG:3857"
      />

      <ol-tile-layer>
        <ol-source-osm />
      </ol-tile-layer>

      <ol-overlay
        :position="props.entity.coordinates"
        :stop-event="true"
      >
        <ViewerMapMarker
          :callback-item="null"
          :width="24"
          :height="38"
          :fill="props.entity.category.fill_color"
          :stroke="props.entity.category.border_color"
          :highlighted="false"
          :icon-url="`/api/icons/${props.entity.category.icon_hash}`"
        />
      </ol-overlay>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import type { Coordinate } from 'ol/coordinate'

type EntitySubset = {
  id: string
  coordinates: Coordinate
  category: {
    fill_color: string
    border_color: string
    icon_hash: string | null | undefined
  }
}

const props = defineProps<{
  entity: EntitySubset
}>()
</script>

<style scoped lang="scss">
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>
