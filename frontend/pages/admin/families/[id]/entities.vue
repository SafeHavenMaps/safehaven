<template>
  <form
    class="flex flex-column gap-3 mx-4"
    style="width:70rem;"
    @submit.prevent="onSave"
  >
    <Fieldset
      v-for="i in Array.from({ length: page_count }, (_, i) => i + 1)"
      :key="`Page ${i}`"
      :legend="i==page_count ? 'Nouvelle page' : `Page ${i}`"
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
      @drop="onDropPage($event, i)"
    >
      <Fieldset
        v-for="(field, index) in getFieldsForPage(i)"
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
        @dragstart="onDragStart($event, field as FormField)"
        @dragover.prevent
        @drop="onDropField($event, index, i)"
      >
        <span class="flex gap-5">
          <div class="flex flex-column gap-3 ">
            <span class="flex align-items-center gap-2 flex-grow-1">

              <label :for="'display_name_' + index">Titre :</label>
              <InputText
                :id="'display_name_' + index"
                v-model="field.display_name"
                :variant="hasFieldBeenEdited(index, 'display_name') ? 'filled': 'outlined'"
                :invalid="!field.display_name"
                class="mr-4"
              />
              <label :for="'key_' + index">Clé :</label>
              <InputText
                :id="'key_' + index"
                v-model="field.key"
                :variant="hasFieldBeenEdited(index, 'key') ? 'filled': 'outlined'"
                :invalid="!field.key"
                class="w-12rem"
              />
              <Badge
                v-tooltip.bottom="`Clé unique d'identification du champ.
                  Si modifiée, les anciennes entités ayant l'ancienne clé ne pourront plus afficher ce champ,
                  sauf si un champ du formulaire vient à porter de nouveau l'ancienne clé.`"
                value="!"
                severity="secondary"
                class="mr-2 border-1 border-black-alpha-10"
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
              />
            </span>

            <!-- <AdminInputNumberField
                  :id="'form_page_' + index"
                  v-model="field.form_page"
                  label="Form Page"
                  :variant="hasFieldBeenEdited(index, 'form_page')"
                />

                <AdminInputNumberField
                  :id="'form_weight_' + index"
                  v-model="field.form_weight"
                  label="Form Weight"
                  :variant="hasFieldBeenEdited(index, 'form_weight')"
                /> -->
          </div>

          <div class="flex flex-column gap-3 w-20rem">
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
                :variant="hasFieldBeenEdited(index, 'mandatory')"
              />

              <AdminInputSwitchField
                :id="'indexed_' + index"
                v-model="field.indexed"
                label="Recherchable"
                :variant="hasFieldBeenEdited(index, 'indexed')"
              />
            </span>
          </div>
          <Button
            outlined
            rounded
            severity="primary"
          >
            <template #icon>
              <AppIcon
                icon-name="delete"
              />
            </template>
          </Button>
        </span>

        <span class="flex align-items-center gap-2 mr-3">
          <label :for="'help_' + index">Texte d'aide :</label>
          <InputText
            :id="'help_' + index"
            v-model="field.help"
            class="flex flex-grow-1"
            :variant="hasFieldBeenEdited(index, 'help') ? 'filled': 'outlined'"
          />
        </span>
      </Fieldset>
      <div class="flex justify-content-center">
        <Button
          outlined
          rounded
          severity="primary"
          label="Nouveau champ"
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
          :disabled="processingRequest"
        />
      </NuxtLink>
      <Button
        label="Sauvegarder"
        type="submit"
        :disabled="processingRequest"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateFamily, StringFieldTypeMetadata } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

type FormField = NewOrUpdateFamily['entity_form']['fields'][number]

const id = useRoute().params.id as string

const fetchedFamily = await state.client.getFamily(id)
const editedFamily: Ref<NewOrUpdateFamily> = ref(JSON.parse(JSON.stringify(fetchedFamily))) // deep copy

const processingRequest = ref(false)
const toast = useToast()

const page_count = ref(1 + Math.max(...editedFamily.value.entity_form.fields.map(field => field.form_page)))
const display_indexes: Ref<Record<string, 'notDisplayed' | number>> = ref({})
const max_display_index = ref(0)

// Initialize display_indexes and max_display_index
editedFamily.value.entity_form.fields.forEach((field) => {
  if (field.user_facing) {
    display_indexes.value[field.key] = field.display_weight
    max_display_index.value = Math.max(max_display_index.value, field.display_weight)
  }
  else {
    display_indexes.value[field.key] = 'notDisplayed'
  }
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition du formulaire d'ajout d'entités de la famille ${fetchedFamily.title}`,
  'family',
  [],
  [
    { label: 'Familles', url: '/admin/families' },
    { label: `Édition de la famille ${fetchedFamily.title}`, url: `/admin/families/${id}` },
  ],
)

function hasFieldBeenEdited(index: number, field: keyof FormField) {
  return editedFamily.value.entity_form.fields[index][field] !== fetchedFamily.entity_form.fields[index][field]
}

function getFieldsForPage(pageNumber: number) {
  return editedFamily.value.entity_form.fields.filter(field => field.form_page === pageNumber)
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
  editedFamily.value.entity_form.fields.forEach(
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
    const draggedIndex = editedFamily.value.entity_form.fields.findIndex(f => f.key === droppedField!.key)
    editedFamily.value.entity_form.fields.splice(draggedIndex, 1)
    editedFamily.value.entity_form.fields.splice(index, 0, droppedField)
  }
}

function onDropPage(event: DragEvent, page: number) {
  const droppedField: FormField = JSON.parse(event.dataTransfer!.getData('application/json')).field!
  const fieldIndex = editedFamily.value.entity_form.fields.findIndex(f => f.key === droppedField.key)
  if (fieldIndex !== -1) {
    editedFamily.value.entity_form.fields[fieldIndex].form_page = page
  }
  updatePageCount()
}

function onKeyChange(old_key: string, new_key: string) {
  editedFamily.value.entity_form.fields.forEach((field) => {
    if (field.key == new_key) {
      toast.add({ severity: 'error', summary: 'Erreur', detail: 'Clé déjà actuellement utilisée par un autre champ', life: 3000 })
      return
    }
  })
  editedFamily.value.entity_form.fields.forEach((field) => {
    if (field.key == old_key) {
      field.key = new_key
      toast.add({ severity: 'error', summary: 'Erreur', detail: 'Clé déjà actuellement utilisée par un autre champ', life: 3000 })
      return
    }
  })
  const display_index = display_indexes.value[old_key]
  // risk of deleting hidden key of the record, such as __data, but this issue is present as soon as such a key is set
  // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
  delete display_indexes.value[old_key]
  display_indexes.value[new_key] = display_index
}

function onDisplayIndexChange(key: string, new_display_index: 'notDisplayed' | number) {
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
  if (new_display_index === 'notDisplayed') {
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
  display_indexes.value[key] = new_display_index
}

function onDelete(key: string, index: number) {
  editedFamily.value.entity_form.fields.splice(index, 1)
  if (display_indexes.value[key] != 'notDisplayed')
    max_display_index.value -= 1
  updatePageCount()
}

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.updateFamily(id, editedFamily.value)
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
  { label: 'Éditeur riche', value: 'EditorText' },
  { label: 'Vrai / Faux', value: 'Boolean' },
  { label: 'Nombre', value: 'Number' },
  { label: 'Score', value: 'DiscreteScore' },
  { label: 'Date', value: 'Date' },
  { label: 'Choix Unique', value: 'EnumSingleOption' },
  { label: 'Choix Multiples', value: 'EnumMultiOption' },
  { label: 'Liste d\'événements', value: 'EventList' },
]

const fieldStringTypeOptions = [
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
