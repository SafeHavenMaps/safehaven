<template>
  <div
    class="p-grid p-dir-col"
    style="height: 100%; width: 100%; display: flex; flex-direction: column"
  >
    <div class="p-col-fixed">
      <ClientNavBar />
    </div>
    <div
      ref="containerRef"
      class="p-col"
      style="height: 100%; width: 100%; flex-grow: 1; overflow: auto"
    >
      <Sidebar
        v-model:visible="state.hasActiveEntity"
        :modal="false"
        position="left"
        :style="dynamicStyles"
      >
        <pre>{{ state.activeEntity }}</pre>
      </Sidebar>
      <div class="app-body">
        <ClientMap
          :center="state.startCenter()"
          :zoom="state.startZoom()"
          :entities="state.entities"
          :clusters="state.clusters"
          @entity-click="(e) => state.selectedCachedEntity(e)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import state from '~/lib/state'
import ClientMap from '~/components/client/Map.vue'

// Init state with url token
const route = useRoute()
const token = route.params.token as string
const containerRef = ref<HTMLElement | null>(null)

const dynamicStyles = computed(() => {
  if (containerRef.value) {
    const height = `${containerRef.value.clientHeight}px`
    const top = containerRef.value.getBoundingClientRect().top + 'px'
    // const top = `${navBarRef.value.clientHeight}px`;
    return { height, top }
  }
  return {} // Return default/fallback styles if needed
})

await state.initWithToken(token) // TODO: Redirect to 404 if token is invalid
</script>

<style scoped>
.app-body {
  height: 100%;
  width: 100%;
}
</style>
