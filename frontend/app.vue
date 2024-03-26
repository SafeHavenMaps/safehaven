<template>
  <div>
    <h1>Hello</h1>
  </div>
</template>

<script setup lang="ts">
import useClient from "~/lib/client";
const client = useClient();

const { data } = await useAsyncData("health", async () => {
  return await client.GET("/api/health");
});

console.log(data);
</script>
