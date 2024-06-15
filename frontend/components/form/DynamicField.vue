<template>
  <div
    v-if="props.formField.field_type=='Boolean'"
    class="flex flex-column gap-2"
  >
    <span class="flex gap-2 align-items-center">
      <InputSwitch
        :id="props.formField.key"
        :model-value="(props.fieldContent as (boolean | undefined))"
        @update:model-value="updateField"
      />
      <label :for="props.formField.key">{{ props.formField.display_name }}</label>
    </span>
    <small v-if="props.formField.help">{{ props.formField.help }}</small>
  </div>
  <div
    v-else
    class="flex flex-column gap-2"
  >
    <label :for="props.formField.key">{{ props.formField.display_name }}</label>
    <InputText
      v-if="props.formField.field_type=='SingleLineText'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (string | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent) || false"
      @update:model-value="updateField"
    />
    <Textarea
      v-if="props.formField.field_type=='MultiLineText'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (string | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent) || false"
      @update:model-value="updateField"
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
      :model-value="(props.fieldContent as (number | undefined))"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent)"
      @update:model-value="updateField"
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
    />
    <Calendar
      v-if="props.formField.field_type=='Date'"
      :id="props.formField.key"
      :model-value="props.fieldContent as (Date | undefined)"
      :variant="props.hasBeenEdited ? 'filled': 'outlined'"
      :invalid="(props.formField.mandatory && !props.fieldContent)"
      date-format="dd/mm/yy"
      show-icon
      icon-display="input"
      show-button-bar
      @update:model-value="(value: Date | Date[] | (Date | null)[] | null | undefined) => updateField(value as (Date | undefined))"
    />
    <Dropdown
      v-if="props.formField.field_type=='EnumSingleOption'"
      :id="props.formField.key"
      :model-value="(props.fieldContent as (string | undefined))"
      :options="props.formField.field_type_metadata?.options"
      option-value="value"
      option-label="label"
      @update:model-value="updateField"
    />
    <MultiSelect
      v-if="props.formField.field_type=='EnumMultiOption'"
      :id="props.formField.key"
      :model-value="(props.fieldContent as (string[] | undefined))"
      :options="props.formField.field_type_metadata?.options"
      option-value="value"
      option-label="label"
      @update:model-value="updateField"
    />
    <div
      v-if="props.formField.field_type=='EventList'"
      :id="props.formField.key"
      class="flex flex-column gap-2 border-1 p-2 surface-border border-round"
    >
      <div
        v-for="(event, ev_index) in (props.fieldContent as EntityOrCommentEvent[])"
        :key="ev_index"
        class="flex flex-column gap-2"
      >
        <span class="flex align-items-center gap-3">
          <Dropdown
            v-model="event.type"
            class="flex-grow-1"
            :options="props.formField.field_type_metadata?.event_types"
            option-value="value"
            option-label="label"
          />
          <!-- @update:model-value="new_type => (props.fieldContent as EntityOrCommentEvent[])[ev_index] = { type: new_type, date: event.date, details: event.details }" -->
          <Calendar
            v-model="event.date"
            class="w-10rem"
            :invalid="!event.date"
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

        <span class="flex align-items-center gap-2">
          <label for="">Détails (optionels): </label>
          <InputText
            v-model="event.details"
          />
        </span>
      </div>
      <span class="flex justify-content-center">
        <Button
          rounded
          class="m-0 p-0"
          @click="(props.fieldContent as EntityOrCommentEvent[]).push({ date: undefined, type: undefined, details: undefined })"
        >
          <template #default>
            <AppIcon
              icon-name="add"
            />
          </template>
        </Button>
      </span>
    </div>
    <small v-if="props.formField.help">{{ props.formField.help }}</small>
  </div>
</template>

<!-- eslint-disable @typescript-eslint/no-explicit-any -->
<script setup lang="ts">
// import Editor from 'primevue/editor'
import Calendar from 'primevue/calendar'
import type { EntityOrCommentEvent, FieldContentMap, FormField } from '~/lib'

type FormProps<T extends FormField> = {
  fieldContent: FieldContentMap[T['field_type']]
  formField: T
  hasBeenEdited?: boolean
}

const props = defineProps<FormProps<FormField>>()

const emit = defineEmits(['update:fieldContent'])

function updateField(value: undefined | FieldContentMap[FormField['field_type']]) {
  emit('update:fieldContent', value)
}

if (props.formField.field_type == 'EventList' && props.fieldContent == undefined) {
  emit('update:fieldContent', [])
}
</script>
