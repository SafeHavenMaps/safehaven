<template>
  <Dialog
    v-model:visible="drawVisible"
    modal
    maximizable
    dismissable-mask
    :header="$t('cmp.polygonDrawer.header')"
    content-class="h-96"
  >
    <div class="flex flex-col gap-1 w-full h-full">
      <ol-map
        :load-tiles-while-animating="true"
        :load-tiles-while-interacting="true"
        class="w-full h-full"
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

        <ol-vector-layer>
          <ol-source-vector
            :features="vectorFeatures"
            :projection="projection"
          >
            <template
              v-for="(feature, index) in vectorFeatures"
              :key="index"
            >
              <ol-feature :feature="feature" />
            </template>

            <ol-interaction-draw
              v-if="drawEnable"
              :type="drawType"
              @drawend="drawend"
            >
              <ol-interaction-snap />
            </ol-interaction-draw>
          </ol-source-vector>
        </ol-vector-layer>
      </ol-map>

      <span class="flex items-center gap-2">
        <small
          v-if="maxPolygons"
          class="text-muted-color"
        >
          {{ maxPolygons }} {{ maxPolygons>1 ? $t('cmp.polygonDrawer.polygons') : $t('cmp.polygonDrawer.polygon') }} {{ $t('cmp.polygonDrawer.max') }}
        </small></span>

      <span class="flex justify-end items-center gap-2">
        <Button
          :label="$t('cmp.polygonDrawer.clear')"
          severity="warn"
          outlined
          @click="clearPolygons"
        />
        <Button
          :label="$t('cmp.polygonDrawer.cancel')"
          severity="secondary"
          @click="drawVisible=false"
        />
        <Button
          :label="$t('cmp.polygonDrawer.save')"
          @click="save()"
        />
      </span>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import type { Polygon } from 'ol/geom'
import type { DrawEvent } from 'ol/interaction/Draw'
import { transform } from 'ol/proj'
import Feature from 'ol/Feature'
import PolygonGeom from 'ol/geom/Polygon'
import state from '~/lib/admin-state'

const props = defineProps<{
  // polygonList contains a list of polygons, each polygon being a list of coordinate, each coordinate being a list of 2 numbers
  polygonList: [number, number][][]
  maxPolygons?: number
}>()

const localPolygonList = ref<[number, number][][]>(JSON.parse(JSON.stringify(props.polygonList)))

defineExpose({
  show,
})

const emit = defineEmits<{
  'update:polygonList': [value: [number, number][][]]
}>()

await state.fetchConfig()
const center = ref(transform(
  [
    state.options.cartography_init.center_lng!,
    state.options.cartography_init.center_lat!,
  ],
  'EPSG:4326', // WGS84 (GPS long lat)
  'EPSG:3857', // Web Mercator (used in map tiles)
))
const projection = ref('EPSG:3857')
const zoom = ref(state.options.cartography_init.zoom)

const drawEnable = ref(true)
const drawType = ref('Polygon')

const drawVisible = ref(false)

function show() {
  drawVisible.value = true
  localPolygonList.value = JSON.parse(JSON.stringify(props.polygonList))
}

const drawend = (event: DrawEvent) => {
  const coordinates = (event.feature.getGeometry() as Polygon)!.getCoordinates()[0]
  localPolygonList.value.push(coordinates as [number, number][])
  if (props.maxPolygons && props.maxPolygons < localPolygonList.value.length)
    localPolygonList.value.shift()
}

const clearPolygons = () => {
  localPolygonList.value = []
}

// Generate OpenLayers features from props.polygonList
const vectorFeatures = computed(() => {
  return localPolygonList.value.map((polygon) => {
    const feature = new Feature({
      geometry: new PolygonGeom([polygon]),
    })
    return feature
  })
})

function save() {
  emit('update:polygonList', localPolygonList.value)
  drawVisible.value = false
}
</script>
