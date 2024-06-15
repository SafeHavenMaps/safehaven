<template>
  <div class="flex flex-column gap-2">
    <label :for="props.formField.key">{{ props.formField.display_name }}</label>
    <InputText
      v-if="props.formField.field_type=='SingleLineText'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (string | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent) || false"
      @update:model-value="updateFormField"
    />
    <Textarea
      v-if="props.formField.field_type=='MultiLineText'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (string | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent) || false"
      @update:model-value="updateFormField"
    />
    <!-- add fieldmetadata validation -->
    <!-- <Editor
      v-if="props.formField.field_type=='RichText'"
      :id="props.formField.key"
      :model-value="props.modelValue!"
      editor-style="height:320px"
      @update:model-value="updateValue"
    /> -->
    <InputNumber
      v-if="props.formField.field_type=='Number'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (number | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent) || false"
      @update:model-value="updateFormField"
    />
    <InputSwitch
      v-if="props.formField.field_type=='Boolean'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (boolean | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent) || false"
      @update:model-value="updateFormField"
    />
    <Calendar
      v-if="props.formField.field_type=='DiscreteScore'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (Date | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent) || false"
      date-format="dd/mm/yy"
      show-icon
      icon-display="input"
      show-button-bar
      @update:model-value="(value: Date | Date[] | (Date | null)[] | null | undefined) => updateFormField(value as (Date | undefined))"
    />
    <small v-if="props.formField.help">{{ props.formField.help }}</small>
  </div>
</template>

<!-- eslint-disable @typescript-eslint/no-explicit-any -->
<script setup lang="ts">
// import Editor from 'primevue/editor'
import Calendar from 'primevue/calendar'
import type { FieldContentMap, FormField } from '~/lib'

type FormProps<T extends FormField> = {
  fieldContent: FieldContentMap[T['field_type']]
  formField: T
  hasBeenEdited?: boolean
}

const props = defineProps<FormProps<FormField>>()

const emit = defineEmits(['update:FormField'])

function updateFormField(value: undefined | FieldContentMap[FormField['field_type']]) {
  emit('update:FormField', value)
}
</script>
