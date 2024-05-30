<template>
  <div class="flex flex-column gap-2">
    <label :for="id">{{ label }}</label>
    <InputText
      :id="props.id"
      :model-value="props.modelValue"
      :variant="props.variant ? 'filled': 'outlined'"
      :invalid="props.invalid"
      @update:model-value="updateValue"
    />
    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  id: string
  label: string
  modelValue: string | undefined | null
  variant?: boolean
  invalid: boolean
  helperText?: string
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(value: undefined | string) {
  emit('update:modelValue', value)
}
</script>
