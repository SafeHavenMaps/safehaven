<template>
  <NuxtPage v-if="isApiAvailable" />

  <NuxtPage v-else>
    <!-- TODO: Stylize this error page, give more info, include auto retry -->
    <h1>API is not available</h1>
    <p>Please try again later</p>
  </NuxtPage>
</template>

<script setup lang="ts">
const isApiAvailable = true;
</script>
