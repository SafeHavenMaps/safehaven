<template>
  <NuxtPage v-if="state.loaded" />
  <NuxtPage v-else>
    <h1>API is not available</h1>
    <p>Please try again later</p>
  </NuxtPage>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'

await state.init()
</script>
