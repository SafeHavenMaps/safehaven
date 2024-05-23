<template>
  <div class="h-full flex flex-column" />
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'

// Init state with url token
const route = useRoute()
const token = route.params.token as string
await state.bootstrapWithToken(token) // TODO: Redirect to 404 if token is invalid
</script>
