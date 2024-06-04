<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedFamily.title"
      label="Titre"
      :variant="hasBeenEdited('title')"
    />

    <AdminInputTextField
      id="entity_form_title"
      v-model="editedFamily.entity_form.title"
      label="Titre du formulaire d'ajout d'entité"
      :variant="hasBeenEdited('entity_form')"
    />

    <AdminInputTextField
      id="comment_form_title"
      v-model="editedFamily.comment_form.title"
      label="Titre du formulaire d'ajout de commentaire"
      :variant="hasBeenEdited('comment_form')"
    />

    <span class="flex align-items-center gap-2">
      <Dropdown
        id="sort_order"
        v-model="editedFamily.sort_order"
        :options="Array.from({ length: state.families.length }, (_, i) => i + 1)"
        class="w-full md:w-14rem"
      />
      <label for="sort_order">Ordre d'affichage parmi les familles</label>
    </span>

    <AdminInputIconUpload
      :object-id="id"
      object-type="families"
    />
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
        :disabled="processingRequest || !editedFamily.title || !editedFamily.comment_form.title || !editedFamily.entity_form.title"
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

function hasBeenEdited(field: keyof NewOrUpdateFamily) {
  return editedFamily.value[field] !== fetchedFamily[field]
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
