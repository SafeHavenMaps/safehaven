<template>
  <div class="p-grid p-dir-col" style="height: 100%; width: 100%; display: flex; flex-direction: column;">
    <div class="p-col-fixed">
      <ClientNavBar
        @change="triggerRefresh"
      />
    </div>
    <div class="p-col" style="height: 100%; width: 100%; flex-grow: 1; overflow: auto;">
      <Sidebar
        v-model:visible="state.hasActiveEntity"
        :modal="false"
        position="left"
      >
        <pre>{{ state.activeEntity }}</pre>
      </Sidebar>
      <div class="app-body">
        <ClientMap
          ref="mapRef"
          :center="state.startCenter()"
          :zoom="state.startZoom()"
          :entities="state.entities"
          :clusters="state.clusters"
          @move="mapMoved"
          @entity-click="e => state.selectedCachedEntity(e)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import state from "~/lib/state";
import ClientMap from "~/components/client/Map.vue";
import type { Extent } from "ol/extent";

// Init state with url token
const route = useRoute();
const token = route.params.token as string;
await state.initWithToken(token); // TODO: Redirect to 404 if token is invalid

// Ref to ClientMap
const mapRef = ref();

async function triggerRefresh() {
  console.log("refresh !")
}

async function mapMoved(extent: Extent, zoom: number) {
  state.refreshView(extent, zoom);
}
</script>

<style scoped>
.app-body {
  height: 100%;
  width: 100%;
}
</style>
