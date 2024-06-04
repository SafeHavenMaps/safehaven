<template>
  <form
    class="flex flex-column gap-3 mx-4"
    style="width:64rem;"
    @submit.prevent="onSave"
  >
    <Fieldset
      v-for="i in [1, 2]"
      :key="`Page ${i}`"
      :legend="`Page ${i}`"
      :toggleable="true"
    >
      <Fieldset
        v-for="(field, index) in editedFamily.entity_form.fields"
        :key="field.key"
        :toggleable="true"
        :draggable="true"
        :legend="field.display_name"
      >
        <div class="flex gap-5 ">
          <div class="flex flex-column gap-3 w-28rem">
            <AdminInputTextField
              :id="'display_name_' + index"
              v-model="field.display_name"
              label="Titre"
              :variant="hasFieldBeenEdited(index, 'display_name')"
            />

            <span class="flex align-items-center gap-2">
              <Dropdown
                :id="'field_type' + index"
                v-model="field.field_type_metadata"
              />
              <label :for="'field_type' + index"> Type </label>
            </span>

            <span class="flex align-items-center gap-2">
              <Dropdown
                :id="'field_type_metadata_' + index"
                v-model="field.field_type_metadata"
              />
              <label :for="'field_type_metadata_' + index"> Options </label>
            </span>

            <AdminInputTextField
              :id="'help_' + index"
              v-model="field.help"
              label="Help Text"
              :variant="hasFieldBeenEdited(index, 'help')"
            />
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
                :options="['Non-affiché publiquement', ...Array.from({ length: 5 }, (v, k) => k+1)]"
              />
            </span>
            <AdminInputSwitchField
              :id="'mandatory_' + index"
              v-model="field.mandatory"
              label="Requis"
              :variant="hasFieldBeenEdited(index, 'mandatory')"
            />

            <AdminInputSwitchField
              :id="'user_facing_' + index"
              v-model="field.user_facing"
              label="Affiché publiquement"
              :variant="hasFieldBeenEdited(index, 'user_facing')"
            />

            <AdminInputSwitchField
              :id="'indexed_' + index"
              v-model="field.indexed"
              label="Recherchable"
              :variant="hasFieldBeenEdited(index, 'indexed')"
            />
          </div>
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

function hasFieldBeenEdited(index: number, field: keyof NewOrUpdateFamily['entity_form']['fields'][number]) {
  return editedFamily.value.entity_form.fields[index][field] !== fetchedFamily.entity_form.fields[index][field]
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
