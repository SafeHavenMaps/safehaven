<template>
  <div
    v-if="props.formField.field_type=='Boolean'"
    class="flex flex-col gap-2"
  >
    <span class="flex gap-2 items-center">
      <ToggleSwitch
        :id="props.formField.key"
        :model-value="(props.fieldContent as boolean)"
        @update:model-value="updateField"
      />
      <label :for="props.formField.key">{{ props.formField.display_name }}<RequiredIndicator /> </label>
    </span>
    <small v-if="props.formField.help">{{ props.formField.help }}</small>
  </div>
  <div
    v-else
    class="flex flex-col gap-2"
  >
    <label :for="props.formField.key">{{ props.formField.display_name }} <RequiredIndicator v-if="props.formField.mandatory" /></label>
    <InputText
      v-if="props.formField.field_type=='SingleLineText'"
      :id="props.formField.key"
      v-tooltip="formatTooltip[props.formField.field_type_metadata!.format]"
      :model-value="props.fieldContent as (string | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="!isValid && hasLostFocus"
      @update:model-value="updateField"
      @blur="onLostFocus"
    />
    <Textarea
      v-if="props.formField.field_type=='MultiLineText'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (string | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="!isValid && hasLostFocus"

      @update:model-value="updateField"
      @blur="onLostFocus"
    />
    <!-- add fieldmetadata validation -->
    <ViewerRichTextEditor
      v-if="props.formField.field_type=='RichText'"
      :id="props.formField.key"
      :model-value="(props.fieldContent as (string | undefined))"
      :invalid="!isValid && hasLostFocus"
      @update:model-value="updateField"
      @blur="onLostFocus"
    />
    <InputNumber
      v-if="props.formField.field_type=='Number'"
      :id="props.formField.key"
      :model-value="(props.fieldContent as (number | undefined))"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="!isValid && hasLostFocus"
      @update:model-value="updateField"
      @blur="onLostFocus"
    />
    <SelectButton
      v-if="props.formField.field_type=='DiscreteScore'"
      :id="props.formField.key"
      :model-value="(props.fieldContent as (number | undefined))"
      :options="[{ label: 'Mauvais', value: 0 },
                 { label: 'Plutôt mauvais', value: 3 },
                 { label: 'Plutôt bon', value: 7 },
                 { label: 'Bon', value: 10 }]"
      option-value="value"
      option-label="label"
      @update:model-value="updateField"
      @blur="onLostFocus"
    />
    <DatePicker
      v-if="props.formField.field_type=='Date'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (Date | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="!isValid && hasLostFocus"
      date-format="dd/mm/yy"
      show-icon
      icon-display="input"
      show-button-bar
      @update:model-value="(value: Date | Date[] | (Date | null)[] | null | undefined) => updateField(value as (Date | undefined))"
      @blur="onLostFocus"
    />
    <Select
      v-if="props.formField.field_type=='EnumSingleOption'"
      :id="props.formField.key"
      :model-value="(props.fieldContent as (string | undefined))"
      :options="props.formField.field_type_metadata?.options"
      :invalid="!isValid && hasLostFocus"
      option-value="value"
      option-label="label"
      @update:model-value="updateField"
      @blur="onLostFocus"
    />
    <MultiSelect
      v-if="props.formField.field_type=='EnumMultiOption'"
      :id="props.formField.key"
      :model-value="(props.fieldContent as (string[] | undefined))"
      :options="props.formField.field_type_metadata?.options"
      :invalid="!isValid && hasLostFocus"
      option-value="value"
      option-label="label"
      @update:model-value="updateField"
      @blur="onLostFocus"
    />
    <div
      v-if="props.formField.field_type=='EventList'"
      :id="props.formField.key"
      class="flex flex-col gap-1 border p-2 border-surface rounded-border"
      :invalid="!isValid && hasLostFocus"
      @blur="onLostFocus"
    >
      <div
        v-for="(event, ev_index) in (props.fieldContent as EntityOrCommentEvent[])"
        :key="ev_index"
        class="flex flex-col gap-2"
      >
        <span class="flex flex-wrap items-center gap-4">
          <Select
            v-model="event.type"
            placeholder="type d'évènement"
            class="grow"
            :options="props.formField.field_type_metadata?.event_types"
            option-value="value"
            option-label="label"
          />
          <!-- @update:model-value="new_type => (props.fieldContent as EntityOrCommentEvent[])[ev_index] = { type: new_type, date: event.date, details: event.details }" -->
          <DatePicker
            v-model="event.date"
            class="w-40"
            placeholder="jj/mm/aaaa"
            date-format="dd/mm/yy"
            show-icon
            icon-display="input"
            show-button-bar
          />

          <Button
            rounded
            outlined
            class="m-0 p-1"
            severity="primary"
            @click="(props.fieldContent as EntityOrCommentEvent[]).splice(ev_index, 1)"
          >
            <template #default>
              <AppIcon
                icon-name="delete"
                size="18px"
              />
            </template>
          </Button>
        </span>

        <span class="flex items-center gap-2">
          <label for="">Détails (optionels): </label>
          <Textarea
            v-model="event.details"
          />
        </span>
        <Divider
          class="m-0 mb-1"
          type="dashed"
        />
      </div>
      <span class="flex justify-center">
        <Button
          rounded
          size="small"
          outlined
          class="m-0 p-1 pr-2"
          @click="(props.fieldContent as EntityOrCommentEvent[]).push({ date: undefined, type: undefined, details: undefined })"
        >
          <template #default>
            <AppIcon
              icon-name="add"
            />
            Nouvel évènement
          </template>
        </Button>
      </span>
    </div>
    <small v-if="props.formField.help">{{ props.formField.help }}</small>
  </div>
</template>

<!-- eslint-disable @typescript-eslint/no-explicit-any -->
<script setup lang="ts">
import DatePicker from 'primevue/datepicker'
import validator from 'validator'
import type { EntityOrCommentEvent, FieldContentMap, FormField } from '~/lib'

type FormProps<T extends FormField> = {
  fieldContent: FieldContentMap[T['field_type']]
  formField: T
  hasBeenEdited?: boolean
}

const props = defineProps<FormProps<FormField>>()

const emit = defineEmits(['update:fieldContent', 'isValid'])
const hasLostFocus = ref(false)
const isValid = computed(() => {
  // if the field is missing, it's only valid if optional
  if (props.fieldContent == null || props.fieldContent === '')
    return !props.formField.mandatory

  if (!props.formField.field_type_metadata || !('format' in props.formField.field_type_metadata))
    return true

  switch (props.formField.field_type_metadata.format) {
    case 'url':
      return validator.isURL(props.fieldContent as string)
    case 'email':
      return validator.isEmail(props.fieldContent as string)
    case 'phone':
      return false // For now we will avoid making assumptions on phone number formats
    default:
      return false
  }
})

emit('isValid', isValid)

function updateField(value: undefined | FieldContentMap[FormField['field_type']]) {
  emit('update:fieldContent', value)
  emit('isValid', isValid)
}

function initializeValue() {
  switch (props.formField.field_type) {
    case 'EventList':
      emit('update:fieldContent', [{ date: undefined, type: undefined, details: undefined }])
      break
    case 'Boolean':
      emit('update:fieldContent', false)
  }
}

if (props.fieldContent == undefined) {
  initializeValue()
}

function onLostFocus() {
  hasLostFocus.value = true
}

const formatTooltip: Record<string, string | null> = { url: 'url valide attendu', email: 'email valide attendu', phone: 'Numéro de téléphone attendu', none: null }
</script>
