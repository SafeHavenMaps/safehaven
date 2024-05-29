<template>
  <div class="flex flex-column gap-2">
    <label :for="id">{{ label }}</label>
    <InputText
      :id="props.id"
      :value="props.modelValue"
      :variant="props.variant ? 'filled': 'outlined'"
      :invalid="props.invalid"
      @input="updateValue"
    />
    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'

const props = defineProps<{
  id: string
  label: string
  modelValue: string | undefined | null
  variant?: boolean
  invalid: boolean
  helperText?: string
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(event: Event) {
  emit('update:modelValue', (event.target as HTMLInputElement).value)
}
</script>
