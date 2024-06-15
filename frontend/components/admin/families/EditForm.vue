<template>
  <div>
    <form
      class="flex flex-column gap-3 mx-4"
      style="width:68rem;"
      @submit.prevent="onSave"
    >
      <Fieldset
        v-for="page in Array.from({ length: page_count }, (_, i) => i + 1)"
        :key="`Page ${page}`"
        :legend="page==page_count ? 'Nouvelle page' : `Page ${page}`"
        :toggleable="true"
        style="background-color: #FAF8FF;"
        :pt="{
          legend: {
            class: 'border-1 surface-border',
          },
          content: {
            class: 'flex flex-column align-items-stretch',
          },
        }"
        @dragover.prevent
        @drop="onDropPage($event, page)"
      >
        <Fieldset
          v-for="(field, index) in getFieldsForPage(page)"
          :key="field.key"
          :draggable="true"
          class="draggable-item mb-2"
          :legend="field.display_name"
          :toggleable="true"
          style="background-color: #FAEFFF;"
          :pt="{
            legend: {
              class: 'border-1 surface-border',
            },
            content: {
              class: 'flex flex-column gap-3 ml-1 ',
            },
          }"
          @dragstart="onDragStart($event, field)"
          @dragover.prevent
          @drop="onDropField($event, index, page)"
        >
          <template #legend>
            {{ field.display_name }}
            <Button
              v-tooltip.top="`Ce champ ne sera plus visible sur les ${props.kindName}s le comportant déjà
                  tant que la clé n'est pas réutilisée, mais les informations resteront en base de donnée.`"
              text
              class="m-0 p-0 ml-2"
              severity="primary"
              @click=" (event: Event) => confirm.require({
                target: event.currentTarget as HTMLElement,
                group: 'delete',
                message: `Confirmer la suppression du champ`,
                objectId: `${field.display_name}`,
                icon: 'warning',
                rejectClass: 'p-button-secondary p-button-outlined p-button-sm',
                acceptClass: 'p-button-sm',
                rejectLabel: 'Annuler',
                acceptLabel: 'Confirmer',
                reject: () => {},
                accept: () => onFieldDelete(field.key),
              } as ExtendedConfirmationOptions)"
            >
              <template #default>
                <AppIcon
                  icon-name="delete"
                />
              </template>
            </Button>
          </template>
          <span class="flex gap-5">
            <div
              class="flex flex-column gap-3 mr-5"
              style="min-width: 50%;"
            >
              <span class="flex align-items-center gap-2 flex-grow-1">

                <label :for="'display_name_' + index">Titre :</label>
                <InputText
                  :id="'display_name_' + index"
                  v-model="field.display_name"
                  :variant="hasFieldAttributeBeenEdited(index, 'display_name') ? 'filled': 'outlined'"
                  :invalid="!field.display_name"
                  class="mr-4 flex-grow-1"
                />
                <Button
                  label="Modifier la clé"
                  outlined
                  @click="() => {
                    editKeyField = field
                    editKeyKey = field.key
                    editKeyVisible = true
                  }"
                />
              </span>

              <span class="flex align-items-center gap-2">
                <label :for="'field_type_' + index"> Type : </label>
                <Dropdown
                  :id="'field_type_' + index"
                  v-model="field.field_type"
                  :options="fieldTypeOptions"
                  option-label="label"
                  option-value="value"
                  @update:model-value="() => onFieldTypeChange(field)"
                />

                <Dropdown
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
                  label="Options"
                  class="border-1 border-black-alpha-10"
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
                  label="Évènements"
                  class="border-1 border-black-alpha-10"
                  severity="secondary"
                  @click="() => {
                    editEventsField = field
                    editEventsEvents = JSON.parse(JSON.stringify((field.field_type_metadata as EventsFieldTypeMetadata).event_types))
                    editEventsVisible = true
                  }"
                />
                <Badge
                  v-tooltip.bottom="'Modifier le type, le sous-type ou le format du champ peut causer de l\'incohérence parmi les anciennes données et entre les anciennes et nouvelles données'"
                  value="!"
                />
              </span>
            </div>

            <div class="flex flex-column gap-3">
              <span class="flex align-items-center gap-2">
                <label :for="'display_weight_' + index"> Ordre d'affichage : </label>
                <Dropdown
                  :id="'display_weight_' + index"
                  :model-value="display_indexes[field.key]"
                  option-label="label"
                  option-value="value"
                  :options="[{ value: 'notDisplayed',
                               label: 'Non-affiché publiquement' }, ...Array.from({ length: max_display_index }, (_, i) => ({ value: i + 1, label: i + 1 }))]"
                  @update:model-value="(new_index : 'notDisplayed' | number) => onDisplayIndexChange(field.key, new_index) "
                /> <!--  field.user_facing -->
              </span>
              <span class="flex align-items-center gap-5">
                <AdminInputSwitchField
                  :id="'mandatory_' + index"
                  v-model="field.mandatory"
                  label="Requis"
                  :variant="hasFieldAttributeBeenEdited(index, 'mandatory')"
                />

                <AdminInputSwitchField
                  :id="'indexed_' + index"
                  v-model="field.indexed"
                  label="Recherchable"
                  :variant="hasFieldAttributeBeenEdited(index, 'indexed')"
                />
              </span>
            </div>

          </span>

          <span class="flex align-items-center gap-2 mr-3">
            <label :for="'help_' + index">Texte d'aide :</label>
            <InputText
              :id="'help_' + index"
              v-model="field.help"
              class="flex flex-grow-1"
              :variant="hasFieldAttributeBeenEdited(index, 'help') ? 'filled': 'outlined'"
            />
          </span>
        </Fieldset>
        <div class="flex justify-content-center">
          <Button
            outlined
            rounded
            severity="primary"
            label="Nouveau champ"
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

      <span class="flex gap-1 justify-content-end">
        <NuxtLink to="/admin/families">
          <Button
            label="Annuler"
            severity="secondary"
            :loading="processingRequest"
            :disabled="processingRequest"
          />
        </NuxtLink>
        <Button
          label="Sauvegarder"
          type="submit"
          :loading="processingRequest"
          :disabled="processingRequest || anyFieldTitleOrKeyEmpty"
        />
      </span>
    </form>

    <Dialog
      v-if="editOptionsVisible"
      v-model:visible="editOptionsVisible"
      modal
      dismissable-mask
      :closable="false"
      :header="`Édition des options du champ ${editOptionsField!.display_name}`"
    >
      <div class="flex flex-column gap-3">
        <div
          v-for="(option, index) in editOptionsOptions"
          :key="index"
          class="flex align-items-center gap-2"
        >
          <label :for="'option_label_' + index">Label :</label>
          <InputText
            :id="'option_label_' + index"
            v-model="option.label"
            placeholder="nom de l'option"
          />
          <label
            v-tooltip.bottom="`Si activé, ce champ ne sera pas affiché dans les résultats lorsque cette option est sélectionnée`"
            :for="'option_hidden_' + index"
          >Cachée :</label>
          <InputSwitch
            :id="'option_hidden_' + index"
            v-model="option.hidden"
          />
          <label :for="'option_value_' + index">Clé :</label>
          <InputText
            :id="'option_value_' + index"
            v-model="option.value"
            placeholder="clé unique"
          />
          <Badge
            v-tooltip.bottom="`Modifier la clé ou supprimer l'option peut entraîner des incohérences dans les ${props.kindName}s utilisant cette option.`"
            value="!"
          />
          <Button
            v-tooltip.top="`Cette option ne sera plus visible sur les ${props.kindName}s le comportant déjà
                  tant que la clé n'est pas réutilisée, mais les informations resteront en base de donnée.`"
            rounded
            outlined
            class="m-0 p-1 ml-2"
            severity="primary"
            @click=" editOptionsOptions.splice(index, 1)"
          >
            <template #default>
              <AppIcon
                icon-name="delete"
                size="18px"
              />
            </template>
          </Button>
        </div>
        <span class="flex justify-content-center">
          <Button
            type="button"
            label="Ajouter une option"
            @click="editOptionsOptions.push({ label: '', value: '', hidden: false })"
          />
        </span>

        <div class="flex justify-content-end gap-2">
          <Button
            type="button"
            label="Annuler"
            severity="secondary"
            @click="editOptionsVisible = false"
          />
          <Button
            type="button"
            label="Confirmer"
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
      :header="`Édition des évènements du champ ${editEventsField!.display_name}`"
    >
      <div class="flex flex-column gap-3">
        <div
          v-for="(event, index) in editEventsEvents"
          :key="index"
          class="flex align-items-center gap-2"
        >
          <label :for="'event_label_' + index">Label :</label>
          <InputText
            :id="'event_label_' + index"
            v-model="event.label"
            placeholder="nom de l'évènement"
          />
          <label :for="'event_color_' + index">Couleur :</label>
          <InputText
            :id="'event_color_' + index"
            v-model="event.color"
          />
          <ColorPicker
            :id="'event_color_picker_' + index"
            v-model="event.color"
            format="hex"
          />
          <label :for="'event_value_' + index">Clé :</label>
          <InputText
            :id="'event_value_' + index"
            v-model="event.value"
            placeholder="clé unique"
          />
          <Badge
            v-tooltip.bottom="`Modifier la clé ou supprimer l'évènement peut entraîner des incohérences dans les ${props.kindName}s utilisant cette évènement.`"
            value="!"
          />
          <Button
            v-tooltip.top="`Cet évènement ne sera plus visible sur les ${props.kindName}s le comportant déjà
                  tant que la clé n'est pas réutilisée, mais les informations resteront en base de donnée.`"
            rounded
            outlined
            class="m-0 p-1 ml-2"
            severity="primary"
            @click=" editEventsEvents.splice(index, 1)"
          >
            <template #default>
              <AppIcon
                icon-name="delete"
                size="18px"
              />
            </template>
          </Button>
        </div>
        <span class="flex justify-content-center">
          <Button
            type="button"
            label="Ajouter un évènement"
            @click="editEventsEvents.push({ label: '', value: '', color: '#FFFFFF' })"
          />
        </span>

        <div class="flex justify-content-end gap-2">
          <Button
            type="button"
            label="Annuler"
            severity="secondary"
            @click="editEventsVisible = false"
          />
          <Button
            type="button"
            label="Confirmer"
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
      :header="`Édition de la clé du champ ${editKeyField!.display_name}`"
    >
      <div
        class="flex flex-column gap-3"
      >
        <span class="flex align-items-center gap-2">
          <label for="display_name_add">Titre :</label>
          <InputText
            id="display_name_add"
            v-model="editKeyField!.display_name"
            :invalid="!editKeyField!.display_name"
            placeholder="Adresse du coiffeur"
          />
        </span>
        <span class="flex align-items-center gap-2 ml-2">
          <label for="key_add">Clé :</label>
          <InputText
            id="key_add"
            v-model="editKeyKey"
            :invalid="!editKeyKey"
            placeholder="hairdresser_adress"
          />
          <Badge
            v-tooltip.bottom="`Clé unique d'identification du champ.
            Si modifiée, les ${props.kindName}s en base de donnée comportant l'ancienne clé ne pourront plus afficher ce champ,
            sauf si un champ du formulaire avec un type compatible vient à porter de nouveau l'ancienne clé.`"
            value="!"
          />
        </span>
        <div class="flex justify-content-end gap-2">
          <Button
            type="button"
            label="Annuler"
            severity="secondary"
            @click="editKeyVisible = false"
          />
          <Button
            :disabled="!editKeyKey || !editKeyField?.display_name"
            type="button"
            label="Confirmer"
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
      header="Ajout d'un nouveau champ"
    >
      <div
        class="flex flex-column gap-3"
      >
        <span class="flex align-items-center gap-2">
          <label for="display_name_add">Titre :</label>
          <InputText
            id="display_name_add"
            v-model="addFieldTitle"
            :invalid="!addFieldTitle"
            placeholder="Adresse du coiffeur"
          />
        </span>
        <span class="flex align-items-center gap-2 ml-2">
          <label for="key_add">Clé :</label>
          <InputText
            id="key_add"
            v-model="addFieldKey"
            :invalid="!addFieldKey"
            placeholder="hairdresser_adress"
          />
          <Badge
            v-tooltip.bottom="`Clé unique d'identification du champ.
            Si modifiée, les ${props.kindName}s ayant l'ancienne clé ne pourront plus afficher ce champ,
            sauf si un champ du formulaire avec un type compatible vient à porter de nouveau l'ancienne clé.`"
            value="!"
          />
        </span>
        <div class="flex justify-content-end gap-2">
          <Button
            type="button"
            label="Annuler"
            severity="secondary"
            @click="addFieldVisible = false"
          />
          <Button
            :disabled="!addFieldKey || !addFieldTitle"
            type="button"
            label="Confirmer"
            @click="() => onFieldAdd(addFieldKey, addFieldTitle, addFieldFormPage)"
          />
        </div>
      </div>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'
import type { FormField, StringFieldTypeMetadata, OptionsFieldTypeMetadata, EventsFieldTypeMetadata } from '~/lib'

const props = defineProps<{
  kindName: string
  originalFormFields: FormField[]
  onSaveCallback: (editedFormFields: FormField[]) => Promise<{ error: Error | undefined }>
}>()

const edited_form_fields: Ref<FormField[]> = ref(JSON.parse(JSON.stringify(props.originalFormFields))) // deep copy
edited_form_fields.value.sort((field_a, field_b) => field_a.form_weight - field_b.form_weight)

const processingRequest = ref(false)
const toast = useToast()
const confirm = useConfirm()

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

const page_count = ref(1 + Math.max(0, ...edited_form_fields.value.map(field => field.form_page)))
const display_indexes: Ref<Record<string, 'notDisplayed' | number>> = ref({})
const max_display_index = ref(0)

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
      toast.add({ severity: 'error', summary: 'Erreur', detail: 'Clé déjà actuellement utilisée par un autre champ', life: 3000 })
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

function onDisplayIndexChange(key: string, new_display_index: 'notDisplayed' | number | null) {
  const old_display_index = display_indexes.value[key]
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
    delete display_indexes.value[key]
  else
    display_indexes.value[key] = new_display_index
}

function onFieldTypeChange(field: FormField) {
  switch (field.field_type) {
    case 'SingleLineText':
      if (!field.field_type_metadata || !('format' in field.field_type_metadata)) {
        field.field_type_metadata = { format: 'none' }
      }
      break
    case 'EnumSingleOption':
    case 'EnumMultiOption':
      if (!field.field_type_metadata || !('options' in field.field_type_metadata)) {
        field.field_type_metadata = { options: [] }
      }
      break
    case 'EventList':
      if (!field.field_type_metadata || !('event_types' in field.field_type_metadata)) {
        field.field_type_metadata = { event_types: [] }
      }
      break
    default:
      field.field_type_metadata = null
      break
  }
}

function onFieldDelete(key: string) {
  const index = edited_form_fields.value.findIndex(field => field.key === key)
  edited_form_fields.value.splice(index, 1)
  onDisplayIndexChange(key, 'notDisplayed')
  updatePageCount()
}

function onFieldAdd(key: string, title: string, form_page: number) {
  edited_form_fields.value.forEach((field) => {
    if (field.key == key) {
      toast.add({ severity: 'error', summary: 'Erreur', detail: 'Clé déjà actuellement utilisée par un autre champ', life: 3000 })
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
      if (field.field_type == 'EventList') {
        field.field_type_metadata?.event_types.forEach((event_type) => {
          if (event_type.color.length == 6) {
            event_type.color = `#${event_type.color}`
          }
        })
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
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Famille modifiée avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification de la famille', life: 3000 })
  }
  processingRequest.value = false
}

const fieldTypeOptions = [
  { label: 'Texte court', value: 'SingleLineText' },
  { label: 'Texte long', value: 'MultiLineText' },
  { label: 'Éditeur riche', value: 'RichText' },
  { label: 'Vrai / Faux', value: 'Boolean' },
  { label: 'Nombre', value: 'Number' },
  { label: 'Score discret', value: 'DiscreteScore' },
  { label: 'Date', value: 'Date' },
  { label: 'Choix Unique', value: 'EnumSingleOption' },
  { label: 'Choix Multiples', value: 'EnumMultiOption' },
  { label: 'Liste d\'événements', value: 'EventList' },
]

const fieldStringTypeOptions = [
  { label: 'Aucun', value: 'none' },
  { label: 'URL', value: 'url' },
  { label: 'Téléphone', value: 'phone-number' },
  { label: 'E-mail', value: 'e-mail' },
]
</script>

<style scoped>
.draggable-item {
cursor: grab;
}
.draggable-item:active {
cursor: grabbing;
}
</style>
