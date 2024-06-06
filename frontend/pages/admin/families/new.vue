<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedFamily.title"
      label="Titre"
    />

    <AdminInputTextField
      id="entity_form_title"
      v-model="editedFamily.entity_form.title"
      label="Titre du formulaire d'ajout d'entité"
    />

    <AdminInputTextField
      id="comment_form_title"
      v-model="editedFamily.comment_form.title"
      label="Titre du formulaire d'ajout de commentaire"
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

if (state.families == undefined)
  await state.fetchFamilies()

const editedFamily: Ref<NewOrUpdateFamily> = ref({
  comment_form: {
    fields: [],
    title: '',
  },
  entity_form: {
    fields: [],
    title: '',
  },
  sort_order: state.families.length,
  title: '',
})

const processingRequest = ref(false)
const toast = useToast()

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Création d'une nouvelle famille`,
  'family',
  [],
  [
    { label: 'Familles', url: '/admin/families' },
    { label: 'Création d\'une nouvelle famille', url: '/admin/families/new' },
  ],
)

async function onSave() {
  processingRequest.value = true
  try {
    const { id } = await state.client.createFamily(editedFamily.value)
    navigateTo(`/admin/families/new-icon-${id}`)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Famille créée avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de création de la famille', life: 3000 })
  }
  processingRequest.value = false
}
</script>
