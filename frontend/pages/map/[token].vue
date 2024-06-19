<template>
  <div class="h-full flex flex-col">
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

    <Drawer
      v-model:visible="state.hasActiveEntity"
      :modal="false"
      :dismissable="false"
      :style="fitContainer()"
      position="left"
      class="!w-full sm:!w-[30rem]"
      :pt="{ mask: '!w-full sm:!w-auto' }"
    >
      <template #header>
        <div
          v-if="state.activeEntity"
          class="flex items-center justify-start gap-2"
        >
          <CategoryTag
            :category="state.activeEntity!.category"
          />
          <div class="grow font-bold m-0">
            {{ state.activeEntity!.entity.display_name }}
          </div>
          <div class="grow" />
        </div>
      </template>
      <ViewerCommentAddForm
        :family="state.activeEntity!.family"
        :entity="state.activeEntity!.entity"
      />
      <ViewerCommonEntityDisplayer
        v-if="state.activeEntity"
        :entity="state.activeEntity!"
        :categories="state.categories"
        @entity-selected="displayEntityId"
      />
    </Drawer>

    <StartPopup />
  </div>
</template>

<script setup lang="ts">
import type { Coordinate } from 'ol/coordinate'
import type { ViewerCachedEntity, DisplayableCachedEntity } from '~/lib'
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

async function goToEntity(entity: ViewerCachedEntity) {
  await state.selectEntity(entity.entity_id)

  mapRef.value?.goToWebMercatorCoordinates([
    entity.web_mercator_x,
    entity.web_mercator_y,
  ], 14)
}
</script>
