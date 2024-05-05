<template>
  <div class="h-full flex flex-column">
    <ClientNavBar class="flex-none"/>
    <div
      ref="containerRef"
      class="h-full"
    >
      <ClientMap 
        class="h-full"
        :center="state.startCenter()"
        :zoom="state.startZoom()"
        :entities="state.entities"
        :clusters="state.clusters"
        @entity-click="(e) => state.selectedCachedEntity(e)"
      />
    </div>
  </div>
  <ClientEntitySideBar :style="fitContainer()"/>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import state from '~/lib/state'

// Init state with url token
const route = useRoute()
const token = route.params.token as string
const containerRef = ref<HTMLElement | null>(null)


function fitContainer() {
  if (containerRef.value) {
    const height = `${containerRef.value.clientHeight}px`
    const top = containerRef.value.getBoundingClientRect().top + 'px'
    return { height, top, "position": "absolute"}
  }
  return {} // Return default/fallback styles if needed
}

await state.initWithToken(token) // TODO: Redirect to 404 if token is invalid
</script>
