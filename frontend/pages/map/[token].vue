<template>
  <div class="h-full flex flex-col">
    <ViewerNavbar
      :token="token"
      :show-map-button="false"
      :show-search-button="true"
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
      :pt="{ mask: '!w-full sm:!w-auto', pcCloseButton: 'shrink-0' }"
    >
      <template #header>
        <div
          v-if="state.activeEntity"
          class="flex items-center justify-start gap-2"
        >
          <CategoryTag
            :category="state.activeEntity!.category"
          />
          <div class="grow font-bold text-lg m-0">
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

    <Toast />
  </div>
</template>

<script setup lang="ts">
import type { Coordinate } from 'ol/coordinate'
import type { DisplayableCachedEntity, ViewerSearchedCachedEntity } from '~/lib'
import state from '~/lib/viewer-state'
import ViewerMap from '~/components/viewer/Map.vue'

const toast = useToast()

// Init state with url token
const route = useRoute()
const token = route.params.token as string
try {
  await state.bootstrapWithToken(token)
}
catch {
  toast.add({
    severity: 'error',
    summary: 'Erreur',
    detail: 'Impossible de charger la carte',
    life: 3000,
  })
  if (state.redirectUrl) {
    window.location.href = state.redirectUrl
  }
  else {
    window.location.href = '/404'
  }
}

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
  try {
    await state.selectEntity(entityId)
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

async function goToEntity(entity: ViewerSearchedCachedEntity) {
  try {
    await state.selectEntity(entity.entity_id)
  }
  catch {
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: `Impossible de charger l'entité sélectionnée`,
      life: 3000,
    })
  }

  const location = entity.locations[0]

  mapRef.value?.goToWebMercatorCoordinates([
    location.x,
    location.y,
  ], 14)
}
</script>
