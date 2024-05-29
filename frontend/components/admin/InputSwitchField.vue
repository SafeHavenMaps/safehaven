<template>
  <span class="flex align-items-center gap-2">
    <InputSwitch
      :model-value="props.modelValue"
      :input-id="props.id"
      :disabled="props.disabled"
      @update:model-value="(value:boolean) => emit('update:modelValue', value)"
    />
    <label :for="props.id">{{ props.label }}</label>
  </span>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'

const props = defineProps<{
  id: string
  label: string
  modelValue: boolean
  disabled?: boolean
}>()

const emit = defineEmits(['update:modelValue'])
</script>
