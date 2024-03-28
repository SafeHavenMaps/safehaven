<template>
  <NuxtPage v-if="isApiAvailable" />

  <NuxtPage v-else>
    <!-- TODO: Stylize this error page, give more info, include auto retry -->
    <h1>API is not available</h1>
    <p>Please try again later</p>
  </NuxtPage>
</template>

<script setup lang="ts">
import useClient from "~/lib/client";
const client = useClient();
const isApiAvailable = !client.checkHealth();
</script>
