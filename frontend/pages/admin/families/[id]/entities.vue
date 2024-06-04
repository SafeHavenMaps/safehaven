<template>
  <form
    class="flex flex-column gap-3 mx-4"
    style="width:64rem;"
    @submit.prevent="onSave"
  >
    <Fieldset
      v-for="i in [0, 1, 2]"
      :key="`Page ${i}`"
      :legend="`Page ${i}`"
      :toggleable="true"
      style="background-color: #F8E8FF;"
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
        :pt="{
          legend: {
            class: 'border-1 surface-border',
          },
        }"
        @dragstart="onDragStart($event, field)"
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
                />
                <Button
                  :id="'field_type_metadata_' + index"
                  v-model="field.field_type_metadata"
                  label="Options"
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
import type { NewOrUpdateFamily } from '~/lib'
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
  `Édition de la famille ${fetchedFamily.title}`,
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
    editedFamily.value.entity_form.fields.splice(draggedIndex, 1).splice(index, 0, droppedField)
  }
}

function onDropPage(event: DragEvent, page: number) {
  const droppedField: FormField = JSON.parse(event.dataTransfer!.getData('application/json')).field!
  const fieldIndex = editedFamily.value.entity_form.fields.findIndex(f => f.key === droppedField.key)
  if (fieldIndex !== -1) {
    editedFamily.value.entity_form.fields[fieldIndex].form_page = page
  }
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
</script>

<style scoped>
.draggable-item {
  cursor: grab;
}
.draggable-item:active {
  cursor: grabbing;
}
</style>
