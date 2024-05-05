<template>
  <ClientNavBar />
  <div
    ref="containerRef"
    class="h-full w-full"
  >
    <ClientMap
      :center="state.startCenter()"
      :zoom="state.startZoom()"
      :entities="state.entities"
      :clusters="state.clusters"
      @entity-click="(e) => state.selectedCachedEntity(e)"
    />
  </div>
  <Sidebar
        v-model:visible="state.hasActiveEntity"
        :modal="false"
        position="left"
        :style="fitContainer()"
      >
        <pre>{{ state.activeEntity }}</pre>
  </Sidebar>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import state from '~/lib/state'
import ClientMap from '~/components/client/Map.vue'

// Init state with url token
const route = useRoute()
const token = route.params.token as string
const containerRef = ref<HTMLElement | null>(null)


function fitContainer() {
  if (containerRef.value) {
    const height = `${containerRef.value.clientHeight}px`
    const top = containerRef.value.getBoundingClientRect().top + 'px'
    // const top = `${navBarRef.value.clientHeight}px`;
    return { height, top }
  }
  return {} // Return default/fallback styles if needed
}

await state.initWithToken(token) // TODO: Redirect to 404 if token is invalid
</script>
