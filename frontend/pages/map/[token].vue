<template>
  <div class="h-full flex flex-column">
    <ViewerNavbar
      @filters-changed="refreshMap"
      @location-chosen="goToGpsCoordinates"
      @entity-chosen="goToEntity"
    />
    <div
      ref="containerRef"
      class="h-full"
    >
      <ViewerMap
        ref="mapRef"
        class="h-full"
        :center="state.startCenter()"
        :zoom="state.startZoom()!"
        :entities="state.entities"
        :clusters="state.clusters"
        @entity-click="(e: DisplayableCachedEntity) => state.selectedCachedEntity(e)"
      />
    </div>

    <Sidebar
      v-model:visible="state.hasActiveEntity"
      :modal="false"
      :style="fitContainer()"
      position="left"
      class="w-full md:w-20rem lg:w-30rem"
    >
      <template #header>
        <div
          v-if="state.activeEntity"
          class="flex align-items-center gap-2"
        >
          <CategoryTag :category="state.activeEntity!.category" />
          <h3 class="m-0">
            {{ state.activeEntity!.entity.display_name }}
          </h3>
        </div>
      </template>

      <ViewerCommonEntityDisplayer
        v-if="state.activeEntity"
        :entity="state.activeEntity!"
        :categories="state.categories"
        @entity-selected="displayEntityId"
      />
    </Sidebar>
  </div>
</template>

<script setup lang="ts">
import type { Coordinate } from 'ol/coordinate'
import type { CachedEntity, DisplayableCachedEntity } from '~/lib'
import state from '~/lib/viewer-state'
import ViewerMap from '~/components/viewer/Map.vue'

// Init state with url token
const route = useRoute()
const token = route.params.token as string
await state.bootstrapWithToken(token) // TODO: Redirect to 404 if token is invalid

const mapRef = ref<typeof ViewerMap>()

// Compute the dynamic positioning of the sidebar
const containerRef = ref<HTMLElement | null>(null)
function fitContainer() {
  if (containerRef.value) {
    const height = `${containerRef.value.clientHeight}px`
    const top = containerRef.value.getBoundingClientRect().top + 'px'
    return { height, top, position: 'absolute' }
  }
  return {} // Return default/fallback styles if needed
}

function goToGpsCoordinates(coordinates: Coordinate) {
  mapRef.value?.goToGpsCoordinates(coordinates, 13)
}

async function refreshMap() {
  await mapRef.value?.forceRefresh()
}

async function displayEntityId(entityId: string) {
  await state.selectEntity(entityId)
}

async function goToEntity(entity: CachedEntity) {
  await state.selectEntity(entity.entity_id)

  mapRef.value?.goToWebMercatorCoordinates([
    entity.web_mercator_x,
    entity.web_mercator_y,
  ], 14)
}
</script>
