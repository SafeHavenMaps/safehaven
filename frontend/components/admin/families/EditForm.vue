<template>
  <div>
    <form
      class="flex flex-col gap-4 mx-6 my-2"
      style="width:68rem;"
      @submit.prevent="onSave"
    >
      <Fieldset
        v-for="page in Array.from({ length: page_count }, (_, i) => i + 1)"
        :key="`Page ${page}`"
        :legend="page == page_count ? $t('cmp.admin.families.editform.newPage') : $t('cmp.admin.families.editform.page', { page })"
        :toggleable="true"
        class="page-fieldset"
        style=""
        :pt="{
          legend: {
            class: '!border !border-surface',
          },
          content: {
            class: '!flex !flex-col justify-center items-stretch',
          },
        }"
        @dragover.prevent
        @drop="onDropPage($event, page)"
      >
        <Fieldset
          v-for="(field, index) in getFieldsForPage(page)"
          :key="field.key"
          :draggable="true"
          class="field-fieldset draggable-item !my-2"
          :legend="field.display_name"
          :toggleable="true"
          :pt="{
            legend: {
              class: '!border !border-surface !flex !items-center !justify-between !pl-3',
            },
            content: {
              class: '!flex !flex-col !justify-center !gap-4 !ml-1',
            },
          }"
          @dragstart="onDragStart($event, field)"
          @dragover.prevent
          @drop="onDropField($event, index, page)"
        >
          <template #legend>
            {{ field.display_name }}
            <Button
              v-tooltip.top="$t('cmp.admin.families.editform.fieldDeleteTooltip', { kindName: props.kindName })"
              text
              class="m-0 p-0 ml-2"
              severity="primary"
              size="small"
              @click="(event: Event) =>
                confirm.require({
                  target: event.currentTarget as HTMLElement,
                  group: 'delete',
                  message: $t('cmp.admin.families.editform.confirmDeleteField'),
                  objectId: `${field.display_name}`,
                  icon: 'warning',
                  rejectClass: 'p-button-secondary p-button-outlined p-button-sm',
                  acceptClass: 'p-button-sm',
                  rejectLabel: $t('cmp.admin.families.editform.rejectDelete'),
                  acceptLabel: $t('cmp.admin.families.editform.acceptDelete'),
                  reject: () => {},
                  accept: () => onFieldDelete(field.key),
                } as ExtendedConfirmationOptions)"
            >
              <template #default>
                <AppIcon icon-name="delete" />
              </template>
            </Button>
          </template>

          <span class="flex gap-8 my-2">
            <div
              class="flex flex-col gap-4 mr-8"
              style="min-width: 50%;"
            >
              <span class="flex items-center gap-2 grow">
                <label :for="'display_name_' + index">{{ $t('cmp.admin.families.editform.fieldTitle') }}:</label>
                <InputText
                  :id="'display_name_' + index"
                  v-model="field.display_name"
                  :variant="hasFieldAttributeBeenEdited(index, 'display_name') ? 'filled' : 'outlined'"
                  :invalid="!field.display_name"
                  class="mr-6 grow"
                  :placeholder="$t('cmp.admin.families.editform.fieldTitlePlaceholder')"
                />
                <Button
                  :label="$t('cmp.admin.families.editform.editKey')"
                  outlined
                  @click="() => {
                    editKeyField = field
                    editKeyKey = field.key
                    editKeyVisible = true
                  }"
                />
              </span>

              <span class="flex items-center gap-2">
                <label :for="'field_type_' + index">{{ $t('cmp.admin.families.editform.fieldType') }}: </label>
                <Select
                  :id="'field_type_' + index"
                  v-model="field.field_type"
                  :options="fieldTypeOptions"
                  option-label="label"
                  option-value="value"
                  @update:model-value="() => onFieldTypeChange(field)"
                />

                <Select
                  v-if="['SingleLineText'].includes(field.field_type)"
                  :id="'field_type_' + index"
                  v-model="(field.field_type_metadata as StringFieldTypeMetadata).format"
                  :options="fieldStringTypeOptions"
                  option-label="label"
                  option-value="value"
                />
                <Button
                  v-if="['EnumSingleOption', 'EnumMultiOption'].includes(field.field_type)"
                  :id="'field_type_metadata_' + index"
                  v-model="field.field_type_metadata"
                  :label="$t('cmp.admin.families.editform.options')"
                  class="border border-black/10"
                  severity="secondary"
                  @click="() => {
                    editOptionsField = field
                    editOptionsOptions = JSON.parse(JSON.stringify((field.field_type_metadata as OptionsFieldTypeMetadata).options))
                    editOptionsVisible = true
                  }"
                />
                <Button
                  v-if="['EventList'].includes(field.field_type)"
                  :id="'field_type_metadata_' + index"
                  v-model="field.field_type_metadata"
                  :label="$t('cmp.admin.families.editform.events')"
                  class="border border-black/10"
                  severity="secondary"
                  @click="() => {
                    editEventsField = field
                    editEventsEvents = JSON.parse(JSON.stringify((field.field_type_metadata as EventsFieldTypeMetadata).event_types))
                    editEventsVisible = true
                  }"
                />
                <Badge
                  v-tooltip.bottom="$t('cmp.admin.families.editform.fieldChangeWarning')"
                  value="!"
                />
              </span>
              <span class="flex items-center gap-2">
                <label :for="'help_' + index">{{ $t('cmp.admin.families.editform.helpText') }}:</label>
                <Textarea
                  :id="'help_' + index"
                  v-model="field.help"
                  auto-resize
                  rows="1"
                  :variant="hasFieldAttributeBeenEdited(index, 'help') ? 'filled' : 'outlined'"
                  class="flex grow"
                />
              </span>
            </div>

            <div class="flex flex-col gap-4">
              <span class="flex items-center gap-2">
                <label :for="'display_weight_' + index">{{ $t('cmp.admin.families.editform.displayOrder') }}: </label>
                <Select
                  :id="'display_weight_' + index"
                  :model-value="display_indexes[field.key]"
                  option-label="label"
                  option-value="value"
                  :options="[{ value: 'notDisplayed', label: 'Non-affiché publiquement' }, ...Array.from({ length: max_display_index }, (_, i) => ({ value: i + 1, label: i + 1 }))]"
                  @update:model-value="(new_index : 'notDisplayed' | number) => onDisplayIndexChange(field, new_index) "
                /> <!--  field.user_facing -->
              </span>
              <span class="flex items-center gap-8">
                <AdminInputSwitchField
                  :id="'mandatory_' + index"
                  v-model="field.mandatory"
                  :label="$t('cmp.admin.families.editform.required')"
                  :variant="hasFieldAttributeBeenEdited(index, 'mandatory')"
                  :disabled="field.field_type == 'Boolean'"
                />

                <AdminInputSwitchField
                  v-if="kind == 'entity' && indexableTypes.includes(field.field_type)"
                  :id="'indexed_' + index"
                  v-model="field.indexed"
                  :label="searchableTypes.includes(field.field_type) ? $t('cmp.admin.families.editform.searchable') : $t('cmp.admin.families.editform.filterable')"
                  :variant="hasFieldAttributeBeenEdited(index, 'indexed')"
                  :disabled="display_indexes[field.key]=='notDisplayed'
                    && searchableTypes.includes(field.field_type)"
                  @update:model-value="onIndexableChange(field)"
                />

                <AdminInputSwitchField
                  v-if="kind == 'entity' && filterableTypes.includes(field.field_type)"
                  :id="'privately_indexed_' + index"
                  v-model="field.privately_indexed"
                  :label="$t('cmp.admin.families.editform.adminFilterOnly')"
                  :variant="hasFieldAttributeBeenEdited(index, 'privately_indexed')"
                  :disabled="display_indexes[field.key] == 'notDisplayed' || !field.indexed"
                  @update:model-value="onIndexableChange(field)"
                />
              </span>
              <span class="flex items-center gap-6">
                <AdminInputSwitchField
                  :id="'is_categories_restricted_' + index"
                  :model-value="!(field.categories == null)"
                  :label="$t('cmp.admin.families.editform.restrictedByCategories')"
                  :variant="hasFieldAttributeBeenEdited(index, 'categories')"
                  @update:model-value="(is_categories_restricted: boolean) => {
                    if (is_categories_restricted) { field.categories = [] }
                    else { field.categories = null }
                  }"
                />
                <MultiSelect
                  v-if="!(field.categories == null)"
                  :id="'field_type_' + index"
                  v-model="field.categories"
                  :max-selected-labels="1"
                  :selected-items-label="$t('cmp.admin.families.editform.selectedCategories')"
                  class="w-full"
                  :options="props.categories"
                  option-label="title"
                  option-value="id"
                />
              </span>
            </div>

          </span>
        </Fieldset>

        <div class="flex justify-center">
          <Button
            outlined
            rounded
            severity="primary"
            :label="$t('cmp.admin.families.editform.newField')"
            @click="() => {
              addFieldFormPage = page
              addFieldVisible = true
              addFieldKey = ''
              addFieldTitle = ''
            }"
          >
            <template #icon>
              <AppIcon
                icon-name="add"
                class="mr-2"
              />
            </template>
          </Button>
        </div>
      </Fieldset>

      <span class="flex gap-1 justify-end">
        <NuxtLink to="/admin/families">
          <Button
            :label="$t('cmp.admin.families.editform.cancel')"
            severity="secondary"
            :loading="processingRequest"
            :disabled="processingRequest"
          />
        </NuxtLink>
        <Button
          :label="$t('cmp.admin.families.editform.save')"
          type="submit"
          :loading="processingRequest"
          :disabled="processingRequest || anyFieldTitleOrKeyEmpty"
        />
      </span>
    </form>

    <!-- Dialogs -->
    <Dialog
      v-if="editOptionsVisible"
      v-model:visible="editOptionsVisible"
      modal
      dismissable-mask
      :closable="false"
      :header="$t('cmp.admin.families.editform.editFieldOptions', { display_name: editOptionsField!.display_name })"
    >
      <div class="flex flex-col gap-4">
        <div
          v-for="(option, index) in editOptionsOptions"
          :key="index"
          class="flex items-center gap-2"
        >
          <label :for="'option_label_' + index">{{ $t('cmp.admin.families.editform.optionLabel') }}:</label>
          <InputText
            :id="'option_label_' + index"
            v-model="option.label"
            :placeholder="$t('cmp.admin.families.editform.optionLabelPlaceholder')"
          />
          <label
            v-tooltip.bottom="$t('cmp.admin.families.editform.optionHiddenTooltip')"
            :for="'option_hidden_' + index"
          >{{ $t('cmp.admin.families.editform.hidden') }}:</label>
          <ToggleSwitch
            :id="'option_hidden_' + index"
            v-model="option.hidden"
          />
          <label :for="'option_value_' + index">{{ $t('cmp.admin.families.editform.optionKey') }}:</label>
          <InputText
            :id="'option_value_' + index"
            v-model="option.value"
            :placeholder="$t('cmp.admin.families.editform.optionKeyPlaceholder')"
          />
          <Badge
            v-tooltip.bottom="$t('cmp.admin.families.editform.optionKeyWarning', { kindName: props.kindName })"
            value="!"
          />
          <Button
            v-tooltip.top="$t('cmp.admin.families.editform.optionDeleteTooltip', { kindName: props.kindName })"
            rounded
            outlined
            class="m-0 p-1 ml-2"
            severity="primary"
            @click="editOptionsOptions.splice(index, 1)"
          >
            <template #default>
              <AppIcon
                icon-name="delete"
                size="18px"
              />
            </template>
          </Button>
        </div>
        <span class="flex justify-center">
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.addOption')"
            @click="editOptionsOptions.push({ label: '', value: '', hidden: false })"
          />
        </span>

        <div class="flex justify-end gap-2">
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.cancel')"
            severity="secondary"
            @click="editOptionsVisible = false"
          />
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.confirm')"
            @click="() => {
              editOptionsField!.field_type_metadata = { options: editOptionsOptions }
              editOptionsVisible = false
            }"
          />
        </div>
      </div>
    </Dialog>

    <Dialog
      v-if="editEventsVisible"
      v-model:visible="editEventsVisible"
      modal
      dismissable-mask
      :closable="false"
      :header="$t('cmp.admin.families.editform.editFieldEvents', { display_name: editEventsField!.display_name })"
    >
      <div class="flex flex-col gap-4">
        <div
          v-for="(event, index) in editEventsEvents"
          :key="index"
          class="flex items-center gap-2"
        >
          <label :for="'event_label_' + index">{{ $t('cmp.admin.families.editform.eventLabel') }}:</label>
          <InputText
            :id="'event_label_' + index"
            v-model="event.label"
            :placeholder="$t('cmp.admin.families.editform.eventLabelPlaceholder')"
          />
          <label :for="'event_color_' + index">{{ $t('cmp.admin.families.editform.eventColor') }}:</label>
          <InputText
            :id="'event_color_' + index"
            v-model="event.color"
          />
          <ColorPicker
            :id="'event_color_picker_' + index"
            v-model="event.color"
            format="hex"
          />
          <label :for="'event_value_' + index">{{ $t('cmp.admin.families.editform.eventKey') }}:</label>
          <InputText
            :id="'event_value_' + index"
            v-model="event.value"
            :placeholder="$t('cmp.admin.families.editform.eventKeyPlaceholder')"
          />
          <Badge
            v-tooltip.bottom="$t('cmp.admin.families.editform.eventKeyWarning', { kindName: props.kindName })"
            value="!"
          />
          <Button
            v-tooltip.top="$t('cmp.admin.families.editform.eventDeleteTooltip', { kindName: props.kindName })"
            rounded
            outlined
            class="m-0 p-1 ml-2"
            severity="primary"
            @click="editEventsEvents.splice(index, 1)"
          >
            <template #default>
              <AppIcon
                icon-name="delete"
                size="18px"
              />
            </template>
          </Button>
        </div>
        <span class="flex justify-center">
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.addEvent')"
            @click="editEventsEvents.push({ label: '', value: '', color: '#FFFFFF' })"
          />
        </span>

        <div class="flex justify-end gap-2">
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.cancel')"
            severity="secondary"
            @click="editEventsVisible = false"
          />
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.confirm')"
            @click="() => {
              editEventsField!.field_type_metadata = { event_types: editEventsEvents }
              editEventsVisible = false
            }"
          />
        </div>
      </div>
    </Dialog>

    <Dialog
      v-if="editKeyVisible"
      v-model:visible="editKeyVisible"
      modal
      dismissable-mask
      :closable="false"
      :header="$t('cmp.admin.families.editform.editFieldKey', { display_name: editKeyField!.display_name })"
    >
      <div class="flex flex-col gap-4">
        <span class="flex items-center gap-2">
          <label for="display_name_add">{{ $t('cmp.admin.families.editform.fieldTitle') }}:</label>
          <InputText
            id="display_name_add"
            v-model="editKeyField!.display_name"
            :invalid="!editKeyField!.display_name"
            :placeholder="$t('cmp.admin.families.editform.fieldTitlePlaceholder')"
          />
        </span>
        <span class="flex items-center gap-2 ml-2">
          <label for="key_add">{{ $t('cmp.admin.families.editform.fieldKey') }}:</label>
          <InputText
            id="key_add"
            v-model="editKeyKey"
            :invalid="!editKeyKey"
            :placeholder="$t('cmp.admin.families.editform.fieldKeyPlaceholder')"
          />
          <Badge
            v-tooltip.bottom="$t('cmp.admin.families.editform.keyWarning', { kindName: props.kindName })"
            value="!"
          />
        </span>
        <div class="flex justify-end gap-2">
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.cancel')"
            severity="secondary"
            @click="editKeyVisible = false"
          />
          <Button
            :disabled="!editKeyKey || !editKeyField?.display_name"
            type="button"
            :label="$t('cmp.admin.families.editform.confirm')"
            @click="() => onKeyChange(editKeyField!, editKeyKey)"
          />
        </div>
      </div>
    </Dialog>

    <Dialog
      v-model:visible="addFieldVisible"
      modal
      dismissable-mask
      :closable="false"
      :header="$t('cmp.admin.families.editform.addNewField')"
    >
      <div class="flex flex-col gap-4">
        <span class="flex items-center gap-2">
          <label for="display_name_add">{{ $t('cmp.admin.families.editform.fieldTitle') }}:</label>
          <InputText
            id="display_name_add"
            v-model="addFieldTitle"
            :invalid="!addFieldTitle"
            :placeholder="$t('cmp.admin.families.editform.fieldTitlePlaceholder')"
          />
        </span>
        <span class="flex items-center gap-2 ml-2">
          <label for="key_add">{{ $t('cmp.admin.families.editform.fieldKey') }}:</label>
          <InputText
            id="key_add"
            v-model="addFieldKey"
            :invalid="!addFieldKey"
            :placeholder="$t('cmp.admin.families.editform.fieldKeyPlaceholder')"
          />
          <Badge
            v-tooltip.bottom="$t('cmp.admin.families.editform.keyWarning', { kindName: props.kindName })"
            value="!"
          />
        </span>
        <div class="flex justify-end gap-2">
          <Button
            type="button"
            :label="$t('cmp.admin.families.editform.cancel')"
            severity="secondary"
            @click="addFieldVisible = false"
          />
          <Button
            :disabled="!addFieldKey || !addFieldTitle"
            type="button"
            :label="$t('cmp.admin.families.editform.confirm')"
            @click="() => onFieldAdd(addFieldKey, addFieldTitle, addFieldFormPage)"
          />
        </div>
      </div>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'
import type { FormField, StringFieldTypeMetadata, OptionsFieldTypeMetadata, EventsFieldTypeMetadata, FieldTypeMetadataEnum, Category } from '~/lib'

const props = defineProps<{
  kind: 'entity' | 'comment'
  kindName: string
  categories: Category[]
  originalFormFields: FormField[]
  onSaveCallback: (editedFormFields: FormField[]) => Promise<{ error: Error | undefined }>
}>()

const edited_form_fields: Ref<FormField[]> = ref([])

const page_count = ref(0)
const display_indexes: Ref<Record<string, 'notDisplayed' | number>> = ref({})
const max_display_index = ref(0)

function initOrResetComponent() {
  edited_form_fields.value = (JSON.parse(JSON.stringify(props.originalFormFields))) // deep copy
  edited_form_fields.value.sort((field_a, field_b) => field_a.form_weight - field_b.form_weight)

  page_count.value = (1 + Math.max(0, ...edited_form_fields.value.map(field => field.form_page)))
  display_indexes.value = {}
  max_display_index.value = 0

  // Initialize display_indexes and max_display_index
  edited_form_fields.value.forEach((field) => {
    if (field.user_facing) {
      display_indexes.value[field.key] = field.display_weight
      max_display_index.value = Math.max(max_display_index.value, field.display_weight)
    }
    else {
      display_indexes.value[field.key] = 'notDisplayed'
    }
  })
}

initOrResetComponent()

watch(() => props.originalFormFields, () => {
  initOrResetComponent()
})

const processingRequest = ref(false)
const toast = useToast()
const confirm = useConfirm()
const { t } = useI18n()

interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}

const anyFieldTitleOrKeyEmpty = computed(() => {
  return edited_form_fields.value.some(field => !field.display_name || !field.key)
})

const addFieldVisible = ref(false)
const addFieldTitle = ref('')
const addFieldKey = ref('')
const addFieldFormPage = ref(0)

const editKeyVisible = ref(false)
const editKeyField: Ref<FormField | null> = ref(null)
const editKeyKey = ref('')

const editOptionsVisible = ref(false)
const editOptionsField: Ref<FormField | null> = ref(null)
const editOptionsOptions = ref<OptionsFieldTypeMetadata['options']>([])

const editEventsVisible = ref(false)
const editEventsField: Ref<FormField | null> = ref(null)
const editEventsEvents = ref<EventsFieldTypeMetadata['event_types']>([])

const searchableTypes = ['SingleLineText', 'MultiLineText', 'RichText']
const filterableTypes = ['EnumSingleOption', 'EnumMultiOption']
const indexableTypes = [...searchableTypes, ...filterableTypes]

function hasFieldAttributeBeenEdited(index: number, attribute: keyof FormField) {
  return index < props.originalFormFields.length
    && edited_form_fields.value[index][attribute] !== props.originalFormFields[index][attribute]
}

function getFieldsForPage(pageNumber: number) {
  return edited_form_fields.value.filter(field => field.form_page === pageNumber)
}

function updatePageCount() {
  // Remove empty pages (except for the very last page)
  const deletedPages: number[] = []
  for (let i = 1; i < page_count.value; i++) {
    if (!getFieldsForPage(i).length) {
      page_count.value -= 1
      deletedPages.push(i)
    }
  }
  edited_form_fields.value.forEach(
    (field) => {
      let page_displacement_count = 0
      deletedPages.forEach(
        (deleted_page) => {
          if (field.form_page >= deleted_page) page_displacement_count += 1
        },
      )
      field.form_page -= page_displacement_count
    },
  )
  // Add an empty page at the end if not already present
  if (getFieldsForPage(page_count.value).length) {
    page_count.value += 1
  }
}

function onDragStart(event: DragEvent, field: FormField) {
  event.dataTransfer?.setData('application/json', JSON.stringify({ field }))
  event.dataTransfer!.effectAllowed = 'move'
}

function onDropField(event: DragEvent, index: number, page: number) {
  const droppedField: FormField = JSON.parse(event.dataTransfer!.getData('application/json')).field!
  const fields = getFieldsForPage(page)
  const draggedOverField = fields[index]
  if (droppedField !== draggedOverField) {
    const draggedIndex = edited_form_fields.value.findIndex(f => f.key === droppedField!.key)
    edited_form_fields.value.splice(draggedIndex, 1)
    edited_form_fields.value.splice(index, 0, droppedField)
  }
}

function onDropPage(event: DragEvent, page: number) {
  const droppedField: FormField = JSON.parse(event.dataTransfer!.getData('application/json')).field!
  const fieldIndex = edited_form_fields.value.findIndex(f => f.key === droppedField.key)
  if (fieldIndex !== -1) {
    edited_form_fields.value[fieldIndex].form_page = page
  }
  updatePageCount()
}

function onKeyChange(field_to_modify: FormField, new_key: string) {
  const old_key = field_to_modify.key
  if (old_key == new_key) {
    editKeyVisible.value = false
    return
  }
  for (const field of edited_form_fields.value) {
    if (field.key == new_key) {
      toast.add({ severity: 'error', summary: t('cmp.admin.families.editform.error'), detail: t('cmp.admin.families.editform.keyAlreadyInUse'), life: 3000 })
      return
    }
  }
  editKeyVisible.value = false
  field_to_modify.key = new_key
  const display_index = display_indexes.value[old_key]
  // risk of deleting hidden key of the record, such as __data, but this issue is present as soon as such a key is set
  // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
  delete display_indexes.value[old_key]
  display_indexes.value[new_key] = display_index
}

function onDisplayIndexChange(field: FormField, new_display_index: 'notDisplayed' | number | null) {
  const old_display_index = display_indexes.value[field.key]
  if (old_display_index === new_display_index) return
  if (old_display_index === 'notDisplayed') {
    max_display_index.value += 1
  }
  else {
    // Decrement indexes greater than the old display index
    Object.entries(display_indexes.value).forEach(([key_i, display_index_i]) => {
      if (display_index_i !== 'notDisplayed' && display_index_i > old_display_index) {
        display_indexes.value[key_i] = display_index_i - 1
      }
    })
  }
  if (new_display_index === 'notDisplayed' || new_display_index === null) {
    if (filterableTypes.includes(field.field_type))
      field.privately_indexed = true
    else
      field.indexed = false
    max_display_index.value -= 1
  }
  else {
    // Increment indexes greater than or equal to the new display index
    Object.entries(display_indexes.value).forEach(([key_i, display_index_i]) => {
      if (display_index_i !== 'notDisplayed' && display_index_i >= new_display_index) {
        display_indexes.value[key_i] = display_index_i + 1
      }
    })
  }
  if (new_display_index === null)
  // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
    delete display_indexes.value[field.key]
  else
    display_indexes.value[field.key] = new_display_index
}

function onFieldTypeChange(field: FormField) {
  switch (field.field_type) {
    case 'SingleLineText':
      if (!field.field_type_metadata || !('format' in (field as Extract<FieldTypeMetadataEnum, { field_type: 'SingleLineText' }>))) {
        field.field_type_metadata = { format: 'none' }
      }
      break
    case 'Boolean':
      field.mandatory = true
      break
    case 'EnumSingleOption':
    case 'EnumMultiOption':
      if (!field.field_type_metadata || !('options' in ((field as Extract<FieldTypeMetadataEnum, { field_type: 'EnumSingleOption' | 'EnumMultiOption' }>)).field_type_metadata)) {
        field.field_type_metadata = { options: [] }
      }
      break
    case 'EventList':
      if (!field.field_type_metadata || !('event_types' in ((field as Extract<FieldTypeMetadataEnum, { field_type: 'EventList' }>)).field_type_metadata)) {
        field.field_type_metadata = { event_types: [] }
      }
      break
    default:
      field.field_type_metadata = null
      break
  }
  if (!indexableTypes.includes(field.field_type)) {
    field.indexed = false
    field.privately_indexed = false
  }
  else if (!filterableTypes.includes(field.field_type)) {
    field.privately_indexed = false
    if (display_indexes.value[field.key] == 'notDisplayed')
      field.indexed = false
  }
  else {
    if (display_indexes.value[field.key] == 'notDisplayed')
      field.privately_indexed = field.indexed
  }
}

function onIndexableChange(field: FormField) {
  if (filterableTypes.includes(field.field_type))
    if (display_indexes.value[field.key] == 'notDisplayed')
      field.privately_indexed = field.indexed
    else
      field.privately_indexed = field.privately_indexed && field.indexed
}

function onFieldDelete(key: string) {
  const index = edited_form_fields.value.findIndex(field => field.key === key)
  onDisplayIndexChange(edited_form_fields.value[index], 'notDisplayed')
  edited_form_fields.value.splice(index, 1)
  updatePageCount()
}

function onFieldAdd(key: string, title: string, form_page: number) {
  edited_form_fields.value.forEach((field) => {
    if (field.key == key) {
      toast.add({ severity: 'error', summary: t('cmp.admin.families.editform.error'), detail: t('cmp.admin.families.editform.keyAlreadyInUse'), life: 3000 })
      return
    }
  })
  const new_field: FormField = {
    display_name: title,
    key: key,
    field_type: 'SingleLineText',
    field_type_metadata: { format: 'none' },
    form_page: form_page,
    display_weight: max_display_index.value + 1,
    form_weight: 0,
    user_facing: true,
    indexed: false,
    privately_indexed: false,
    mandatory: false,
  }
  edited_form_fields.value.push(new_field)
  max_display_index.value += 1
  display_indexes.value[key] = max_display_index.value
  updatePageCount()
  addFieldVisible.value = false
}

async function onSave() {
  processingRequest.value = true
  // Set field attributes not set automatically while editing (form_weight, user_facing and display_weight)
  let form_weight_incr = 1
  for (const page of Array.from({ length: page_count.value }, (_, i) => i + 1)) {
    for (const field of getFieldsForPage(page)) {
      field.form_weight = form_weight_incr
      form_weight_incr++
      const pseudo_display_index = display_indexes.value[field.key]
      if (pseudo_display_index === 'notDisplayed') {
        field.user_facing = false
        field.display_weight = 0
      }
      else {
        field.user_facing = true
        field.display_weight = pseudo_display_index
      }
    }
  }
  // Send the request to the server and process the response
  try {
    const { error } = await props.onSaveCallback(edited_form_fields.value)
    if (error) {
      throw error
    }
    navigateTo('/admin/families')
    toast.add({ severity: 'success', summary: t('cmp.admin.families.editform.success'), detail: t('cmp.admin.families.editform.familyUpdatedSuccess'), life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: t('cmp.admin.families.editform.error'), detail: t('cmp.admin.families.editform.familyUpdateError'), life: 3000 })
  }
  processingRequest.value = false
}

const fieldTypeOptions = [
  { label: t('cmp.admin.families.editform.fieldTypeSingleLineText'), value: 'SingleLineText' },
  { label: t('cmp.admin.families.editform.fieldTypeMultiLineText'), value: 'MultiLineText' },
  { label: t('cmp.admin.families.editform.fieldTypeRichText'), value: 'RichText' },
  { label: t('cmp.admin.families.editform.fieldTypeBoolean'), value: 'Boolean' },
  { label: t('cmp.admin.families.editform.fieldTypeNumber'), value: 'Number' },
  { label: t('cmp.admin.families.editform.fieldTypeDiscreteScore'), value: 'DiscreteScore' },
  { label: t('cmp.admin.families.editform.fieldTypeDate'), value: 'Date' },
  { label: t('cmp.admin.families.editform.fieldTypeSingleOption'), value: 'EnumSingleOption' },
  { label: t('cmp.admin.families.editform.fieldTypeMultiOption'), value: 'EnumMultiOption' },
  { label: t('cmp.admin.families.editform.fieldTypeEventList'), value: 'EventList' },
]

const fieldStringTypeOptions = [
  { label: t('cmp.admin.families.editform.stringTypeNone'), value: 'none' },
  { label: t('cmp.admin.families.editform.stringTypeUrl'), value: 'url' },
  { label: t('cmp.admin.families.editform.stringTypePhone'), value: 'phone-number' },
  { label: t('cmp.admin.families.editform.stringTypeEmail'), value: 'e-mail' },
]
</script>

<style scoped>
.draggable-item {
  cursor: grab;
}

.draggable-item:active {
  cursor: grabbing;
}

html::not(.sh-dark) .page-fieldset {
  background-color: #FAF8FF;
}

html::not(.sh-dark) .field-fieldset {
  background-color: #FAEFFF;
}

html.sh-dark .page-fieldset {
  background-color: #453741;
}

html.sh-dark .field-fieldset {
  background-color: #241f22;
}
</style>
