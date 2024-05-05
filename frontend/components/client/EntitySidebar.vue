<template>
  <Sidebar
        v-model:visible="state.hasActiveEntity"
        :modal="false"
        position="left"
      >
        <pre>{{ state.activeEntity }}</pre>
  </Sidebar>
</template>

<script  setup lang="ts">
import state from '~/lib/state'
</script>
