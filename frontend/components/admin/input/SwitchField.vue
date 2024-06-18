<template>
  <div class="flex flex-col gap-2">
    <span class="flex items-center gap-2">
      <ToggleSwitch
        :model-value="props.modelValue"
        :input-id="props.id"
        :disabled="props.disabled"
        @update:model-value="(value:boolean) => emit('update:modelValue', value)"
      />
      <label :for="props.id">{{ props.label }}</label>
    </span>

    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  id: string
  label: string
  modelValue: boolean
  disabled?: boolean
  helperText?: string
}>()

const emit = defineEmits(['update:modelValue'])
</script>
