<template>
  <SelectButton
    v-model="state.activeFamily"
    :options="state.families"
    option-label="title"
    aria-labelledby="custom"
  >
    <template #option="slotProps">
      <div class="button-content flex items-center justify-between">
        <AppIcon
          :icon-name="slotProps.option.icon_hash"
          dynamic
          class="mr-2"
          size="16px"
        />
        <span>{{ slotProps.option.title }}</span>
      </div>
    </template>
  </SelectButton>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'
</script>

<style scoped>
.button-content {
  z-index: 9;
}
</style>
