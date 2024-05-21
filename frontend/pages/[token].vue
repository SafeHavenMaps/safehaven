<template>
  <div class="h-full flex flex-column">
    <ViewerNavbar
      @filters-changed="refreshMap"
    />
    <div
      ref="containerRef"
      class="h-full"
    >
      <ViewerMap
        ref="mapRef"
        class="h-full"
        :center="state.startCenter()"
        :zoom="state.startZoom()"
        :entities="state.entities"
        :clusters="state.clusters"
        @entity-click="(e: DisplayableCachedEntity) => state.selectedCachedEntity(e)"
      />
    </div>
    <ViewerEntitySidebar :style="fitContainer()" />
  </div>
</template>

<script setup lang="ts">
import type { DisplayableCachedEntity } from '~/lib'
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

async function refreshMap() {
  await mapRef.value?.forceRefresh()
}
</script>
