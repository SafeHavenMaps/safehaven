<template>
  <div id="map_container">
    <ol-map
      id="map"
      ref="mapRef"
      :load-tiles-while-animating="true"
      :load-tiles-while-interacting="true"
      @moveend="forceRefresh"
    >
      <ol-view
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
        :stop-event="false"
      >
        <ViewerMapMarker
          :callback-item="entity"
          :width="24"
          :height="38"
          :fill-color="entity.category.fill_color"
          :border-color="entity.category.border_color"
          :icon-hash="entity.category.icon_hash"
          :highlighted="isEntityHighlighted(entity)"
          @click="handleEntityClick"
        />
      </ol-overlay>

      <ol-overlay
        v-for="cluster in props.clusters"
        :key="cluster"
        :position="cluster.coordinates"
        :stop-event="false"
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
import { transform } from 'ol/proj.js'
import type { AppError, DisplayableCachedEntity, DisplayableCluster } from '~/lib'
import state from '~/lib/viewer-state'

const toast = useToast()

const props = defineProps<{
  center: Coordinate
  zoom: number
  entities: DisplayableCachedEntity[]
  clusters: DisplayableCluster[]
}>()

defineExpose({
  forceRefresh,
  goToGpsCoordinates,
  goToWebMercatorCoordinates,
})

const zoom = props.zoom
const center = props.center

const mapRef = ref<{ map: Map }>()
let map: Map | null = null
onMounted(() => {
  map = mapRef.value!.map
})

function isEntityHighlighted(entity: DisplayableCachedEntity) {
  return state.activeEntity?.entity.id === entity.entity_id
}

async function forceRefresh() {
  const { extent, currentZoom } = getExtentAndZoom()
  try {
    await state.refreshView(extent, currentZoom)
  }
  catch (error) {
    if ((error as AppError).error_code !== 'token_validation_error')
      toast.add({
        severity: 'error',
        summary: 'Erreur',
        detail: 'Impossible de rafraîchir la carte',
        life: 3000,
      })
  }
}

watch(
  () => state.activeFamily,
  forceRefresh,
)

function getExtentAndZoom() {
  const extent = map!.getView().getViewStateAndExtent().extent
  const currentZoom = map!.getView().getZoom()!
  return { extent, currentZoom }
}

function goToGpsCoordinates(coordinates: Coordinate, zoom: number) {
  const transformedCoordinates = transform(coordinates, 'EPSG:4326', 'EPSG:3857')

  map!.getView().animate({
    center: transformedCoordinates,
    zoom: zoom,
    duration: 1500,
  })
}

function goToWebMercatorCoordinates(coordinates: Coordinate, zoom: number) {
  map!.getView().animate({
    center: coordinates,
    zoom: zoom,
    duration: 1500,
  })
}

async function handleClusterClick(cluster: DisplayableCluster) {
  map!.getView().animate({
    center: cluster.coordinates,
    zoom: Math.min(map!.getView().getZoom()! + 2, map!.getView().getMaxZoom()!),
    duration: 500,
  })
}

async function handleEntityClick(entity: DisplayableCachedEntity) {
  try {
    await state.selectedCachedEntity(entity)
  }
  catch {
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: `Impossible de charger l'entité sélectionnée`,
      life: 3000,
    })
  }
}
</script>

<style scoped lang="scss">
#map_container,
#map {
  width: 100%;
  height: 100%;
}
</style>
