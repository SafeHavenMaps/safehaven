<template>
  <form
    class="flex flex-col gap-4 max-w-[30rem] mx-6"
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
      :variant="editedFamily.entity_form.title !== fetchedFamily.entity_form.title"
    />

    <AdminInputTextField
      id="entity_form_title"
      v-model="editedFamily.entity_form.help"
      label="Texte d'aide du formulaire d'ajout d'entité"
      :variant="editedFamily.entity_form.help !== fetchedFamily.entity_form.help"
      text-length="long"
    />

    <AdminInputTextField
      id="comment_form_title"
      v-model="editedFamily.comment_form.title"
      label="Titre du formulaire d'ajout de commentaire"
      :variant="editedFamily.comment_form.title !== fetchedFamily.comment_form.title"
    />

    <AdminInputTextField
      id="comment_form_help"
      v-model="editedFamily.comment_form.help"
      label="Texte d'aide du formulaire d'ajout de commentaire"
      :variant="editedFamily.comment_form.help !== fetchedFamily.comment_form.help"
      text-length="long"
    />

    <span class="flex items-center gap-2">
      <Select
        id="sort_order"
        v-model="editedFamily.sort_order"
        :options="Array.from({ length: state.families.length }, (_, i) => i + 1)"
        class="w-full md:w-56"
      />
      <label for="sort_order">Ordre d'affichage parmi les familles</label>
    </span>

    <AdminInputIconUpload
      :object-id="id"
      object-type="families"
    />
    <span class="flex gap-1 justify-end">
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
    { label: `Édition de l'affichage de la famille ${fetchedFamily.title}`, url: `/admin/families/${id}/general` },
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
