<template>
  <form
    class="flex flex-column gap-3 mx-4"
    style="width:64rem;"
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
        }"
        @dragstart="onDragStart($event, field as FormField)"
        @dragover.prevent
        @drop="onDropField($event, index, i)"
      >
        <div class="flex flex-column gap-3 ml-1 mr-5">
          <div class="flex gap-5">
            <div class="flex flex-column gap-3 w-28rem">
              <span class="flex align-items-center gap-2">
                <label :for="'display_name_' + index">Titre :</label>
                <InputText
                  :id="'display_name_' + index"
                  v-model="field.display_name"
                  :variant="hasFieldBeenEdited(index, 'display_name') ? 'filled': 'outlined'"
                  :invalid="!field.display_name"
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

            <div class="flex flex-column gap-3 w-28rem">
              <span class="flex align-items-center gap-2">
                <label :for="'display_weight_' + index"> Ordre d'affichage : </label>
                <Dropdown
                  :id="'display_weight_' + index"
                  v-model="field.display_weight"
                  :options="['Non-affiché publiquement', ...Array.from({ length: 5 }, (v, k) => k + 1)]"
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
          </div>

          <span class="flex align-items-center gap-2">
            <label :for="'help_' + index">Texte d'aide :</label>
            <InputText
              :id="'help_' + index"
              v-model="field.help"
              class="flex flex-grow-1"
              :variant="hasFieldBeenEdited(index, 'help') ? 'filled': 'outlined'"
            />
          </span>
        </div>
      </Fieldset>
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

const page_count = ref(1 + Math.max(...editedFamily.value.entity_form.fields.map(field => field.form_page)))

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
