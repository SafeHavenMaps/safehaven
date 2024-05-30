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
      :invalid="!editedTag.title"
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
      :invalid="!editedTag.filter_description"
      helper-text="(description exposée aux utilisateurices)"
    />

    <span class="flex gap-1 justify-content-end">
      <NuxtLink to="/admin/tags">
        <Button
          label="Annuler"
          severity="secondary"
          :disabled="processingRequest"
        />
      </NuxtLink>
      <Button
        label="Sauvegarder"
        type="submit"
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

const fetchedTag = await state.client.getTag(tagId)
const editedTag: Ref<NewOrUpdateTag> = ref(JSON.parse(JSON.stringify(fetchedTag))) // deep copy

const processingRequest = ref(false)

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
  processingRequest.value = true
  await state.client.updateTag(tagId, editedTag.value)
  navigateTo('/admin/tags')
}
</script>
