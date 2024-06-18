<template>
  <div class="flex flex-column gap-2">
    <label :for="id">{{ label }} <RequiredIndicator v-if="!optional" /></label>
    <InputText
      v-if="textLength==undefined || textLength=='short'"
      :id="props.id"
      :model-value="props.modelValue"
      :variant="props.variant ? 'filled': 'outlined'"
      :invalid="(!props.optional && !props.modelValue) || props.invalid"
      @update:model-value="updateValue"
    />
    <Textarea
      v-if="textLength=='long'"
      :id="props.id"
      :model-value="props.modelValue"
      :variant="props.variant ? 'filled': 'outlined'"
      :invalid="(!props.optional && !props.modelValue) || props.invalid"
      @update:model-value="updateValue"
    />
    <Editor
      v-if="textLength=='editor'"
      :id="props.id"
      :model-value="props.modelValue!"
      editor-style="height:320px"
      @update:model-value="updateValue"
    />
    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
import Editor from 'primevue/editor'

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
