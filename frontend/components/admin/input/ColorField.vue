<template>
  <div class="flex flex-col gap-2">
    <span class="flex items-center gap-2">
      <InputText
        :id="props.id"
        :model-value="props.modelValue"
        :variant="props.variant ? 'filled': 'outlined'"
        :invalid="props.invalid"
        @update:model-value="updateValue"
      />
      <ColorPicker
        :id="`${props.id}_picker`"
        :model-value="props.modelValue"
        format="hex"
        @update:model-value="updateValue"
      />
      <label for="props.id">{{ props.label }}</label>
    </span>

    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  id: string
  label: string
  modelValue: string | undefined
  variant?: boolean
  invalid: boolean
  helperText?: string
}>()

const emit = defineEmits(['update:modelValue', 'update:invalid'])

function isHexColor(colorText: string | undefined) {
  if (colorText == undefined) return false
  return colorText.length === 7 && [...colorText.slice(1)].every(c => '0123456789abcdefABCDEF'.includes(c))
}

function updateValue(value: string | undefined) {
  if (value && value[0] !== '#')
    value = '#' + value
  emit('update:modelValue', value)
  emit('update:invalid', !isHexColor(value))
}
</script>
