<template>
  <div class="flex flex-col gap-2">
    <label :for="id">{{ label }} <RequiredIndicator v-if="!props.optional" /></label>
    <InputText
      v-if="textLength==undefined || textLength=='short'"
      :id="props.id"
      :model-value="props.modelValue"
      :variant="props.variant ? 'filled': 'outlined'"
      :invalid="(!props.optional && !isValidText(props.modelValue)) || props.invalid"
      @update:model-value="updateValue"
    />
    <Textarea
      v-if="textLength=='long'"
      :id="props.id"
      :model-value="props.modelValue"
      :variant="props.variant ? 'filled': 'outlined'"
      :invalid="(!props.optional && !isValidText(props.modelValue)) || props.invalid"
      @update:model-value="updateValue"
    />
    <Editor
      v-if="textLength=='editor'"
      :id="props.id"
      :model-value="props.modelValue!"
      editor-style="height:320px"
      :invalid="(!props.optional && !isValidRichText(props.modelValue)) || props.invalid"
      @update:model-value="updateValue"
    />
    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
import Editor from 'primevue/editor'
import { isValidRichText, isValidText } from '~/lib/validation'

const props = defineProps<{
  id: string
  label: string
  modelValue: string | undefined | null
  variant?: boolean
  invalid?: boolean
  optional?: boolean
  helperText?: string
  textLength?: 'short' | 'long' | 'editor'
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(value: undefined | string) {
  emit('update:modelValue', value)
}
</script>
