<template>
  <div class="flex flex-col gap-2">
    <label :for="id">{{ label }} <RequiredIndicator v-if="!props.optional" /></label>
    <InputNumber
      :id="props.id"
      :model-value="props.modelValue"
      :variant="props.variant ? 'filled': 'outlined'"
      :invalid="props.invalid || !isValidNumber(props.modelValue)"
      @update:model-value="updateValue"
    />
    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
import { isValidNumber } from '~/lib/validation'

const props = defineProps<{
  id: string
  label: string
  modelValue: number | undefined | null
  variant?: boolean
  invalid?: boolean
  helperText?: string
  optional?: boolean
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(value: undefined | number) {
  emit('update:modelValue', value)
}
</script>
