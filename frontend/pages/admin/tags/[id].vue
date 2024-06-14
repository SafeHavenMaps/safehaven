<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedTag.title"
      label="Titre"
      :variant="hasBeenEdited('title')"
    />

    <AdminInputSwitchField
      id="is_filter"
      v-model="editedTag.is_filter"
      label="Filtrant"
    />

    <AdminInputSwitchField
      v-if="editedTag.is_filter"
      id="default_filter_status"
      v-model="editedTag.default_filter_status"
      label="Inclus par défaut"
      helper-text="(si décoché, toutes les entités portant ce tag seront exclues des résultats par défaut)"
    />

    <AdminInputTextField
      v-if="editedTag.is_filter"
      id="filter_description"
      v-model="editedTag.filter_description"
      label="Description du filtre"
      :variant="hasBeenEdited('filter_description')"
      helper-text="(description exposée aux utilisateurices)"
    />

    <span class="flex gap-1 justify-content-end">
      <NuxtLink to="/admin/tags">
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
        :disabled="processingRequest || !editedTag.title || (editedTag.is_filter && !editedTag.filter_description)"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateTag } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const tagId = useRoute().params.id as string

const fetchedTag = await state.fetchTag(tagId)
const editedTag: Ref<NewOrUpdateTag> = ref(JSON.parse(JSON.stringify(fetchedTag))) // deep copy

const processingRequest = ref(false)
const toast = useToast()

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition du jeton ${fetchedTag.title}`,
  'tag',
  [],
  [
    { label: 'Jetons d\'accès', url: '/admin/tags' },
    { label: `Édition du jeton ${fetchedTag.title}`, url: `/admin/tags/${tagId}` },
  ],
)

function hasBeenEdited(field: keyof NewOrUpdateTag) {
  return editedTag.value[field] !== fetchedTag[field]
}

async function onSave() {
  try {
    processingRequest.value = true
    await state.updateTag(tagId, editedTag.value)
    navigateTo('/admin/tags')
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Tag modifié avec succès', life: 3000 })
  }
  catch (error) {
    console.error(error)
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification du tag', life: 3000 })
  }
  processingRequest.value = false
}
</script>
