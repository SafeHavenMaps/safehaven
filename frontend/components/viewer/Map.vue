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
        v-for="entity in props.entities"
        :key="entity.id"
        :position="entity.coordinates"
      >
        <ViewerMapMarker
          :callback-item="entity"
          :width="24"
          :height="38"
          :fill="entity.category.fill_color"
          :stroke="entity.category.border_color"
          :highlighted="entity.entity_id === state.activeEntity?.entity.id"
          @click="handleEntityClick"
        />
      </ol-overlay>

      <ol-overlay
        v-for="cluster in props.clusters"
        :key="cluster"
        :position="cluster.coordinates"
      >
        <ViewerMapCluster
          :callback-item="cluster"
          :count="cluster.count"
          :seed="cluster.id"
          @click="handleClusterClick(cluster)"
        />
      </ol-overlay>
    </ol-map>
  </div>
</template>

<script setup lang="ts">
import type Map from 'ol/Map'
import type { Coordinate } from 'ol/coordinate'
import type { DisplayableCachedEntity, DisplayableCluster } from '~/lib'
import state from '~/lib/viewer-state'

const props = defineProps<{
  center: Coordinate
  zoom: number
  entities: DisplayableCachedEntity[]
  clusters: DisplayableCluster[]
}>()

const zoom = props.zoom
const center = props.center

const mapRef = ref<{ map: Map }>()
let map: Map | null = null
onMounted(() => {
  map = mapRef.value!.map
})

async function onMapMoveEnd() {
  const { extent, currentZoom } = getExtentAndZoom()
  await state.refreshView(extent, currentZoom)
}

watch(
  () => state.activeFamily,
  async () => {
    const { extent, currentZoom } = getExtentAndZoom()
    await state.refreshView(extent, currentZoom)
  },
)

function getExtentAndZoom() {
  const extent = map!.getView().getViewStateAndExtent().extent
  const currentZoom = map!.getView().getZoom()!
  return { extent, currentZoom }
}

function zoomTo(coordinates: Coordinate) {
  map!.getView().animate({
    center: coordinates,
    zoom: Math.min(map!.getView().getZoom()! + 2, map!.getView().getMaxZoom()!),
    duration: 500,
  })
}

function centerTo(coordinates: Coordinate) {
  map!.getView().animate({
    center: coordinates,
    duration: 500,
  })
}

async function handleClusterClick(cluster: DisplayableCluster) {
  zoomTo(cluster.coordinates)
}

async function handleEntityClick(entity: DisplayableCachedEntity) {
  centerTo(entity.coordinates)
  state.selectedCachedEntity(entity)
}
</script>

<style scoped lang="scss">
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>