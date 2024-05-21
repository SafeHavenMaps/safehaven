<template>
  <SelectButton
    v-model="state.activeFamily"
    :options="state.families"
    option-label="title"
    aria-labelledby="custom"
  >
    <template #option="slotProps">
      <div class="button-content flex align-items-center justify-between  ">
        <div
          class="mr-2"
          :style="{
            width: '16px',
            height: '16px',
          }"
          v-html="slotProps.option.icon"
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
