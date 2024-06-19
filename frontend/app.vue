<template>
  <NuxtLoadingIndicator />

  <NuxtLayout>
    <NuxtPage v-if="state.loaded" />
    <NuxtPage v-else>
      <h1 class="text-xl">
        API is not available
      </h1>
      <p>Please try again later</p>
    </NuxtPage>
  </NuxtLayout>

  <Toast />
</template>

<script setup lang="ts">
import { useHead } from '#imports'
import state from '~/lib/viewer-state'

await state.init()

useHead({
  title: state.title,
  link: [{ rel: 'icon', href: state.logo ?? '/default_favicon.ico' }],
})
</script>
