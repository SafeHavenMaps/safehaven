<template>
  <SelectButton
    v-model="state.activeFamily"
    :options="state.families"
    option-label="title"
    aria-labelledby="custom"
  >
    <template #option="slotProps">
      <div class="button-content flex align-items-center justify-between">
        <img
          class="mr-2"
          height="16"
          width="16"
          :src="imageSrc(slotProps.option.icon_hash)"
        >
        <span>{{ slotProps.option.title }}</span>
      </div>
    </template>
  </SelectButton>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'

function imageSrc(hash: string): string {
  return `/api/icons/families/${hash}`
}
</script>

<style scoped>
.button-content {
  z-index: 9;
}
</style>
