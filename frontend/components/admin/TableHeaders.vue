<template>
  <div class="flex align-items-center justify-content-between">
    <h4 class="flex m-0 align-items-end ">
      <AppIcon
        :icon-name="props.modelIcon"
        class="mr-2"
      />
      {{ props.modelDisplayName }}
    </h4>
    <Button
      outlined
      rounded
      severity="success"
      @click="navigateTo(`/admin/${props.modelName}/new`)"
    >
      <template #icon>
        <AppIcon icon-name="add" />
      </template>
    </Button>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  modelName: string
  modelDisplayName: string
  modelIcon: string
}>()
</script>
