<template>
  <SelectButton
    v-model="state.activeFamily"
    :options="state.families"
    option-label="title"
    aria-labelledby="custom"
  >
    <template #option="slotProps">
      <div class="button-content">
        <i :class="slotProps.option.icon" class="mr-2" />
        <span>{{ slotProps.option.title }}</span>
      </div>
    </template>
  </SelectButton>
</template>

<script setup lang="ts">
import state from "~/lib/state";
</script>

<style>
.button-content {
  z-index: 9;
}
</style>
