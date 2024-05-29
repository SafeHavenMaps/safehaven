<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedTag.title"
      label="Titre"
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
      helper-text="(si décoché, toutes les entités portant ce tag seront exclus des résultats par défaut)"
    />

    <AdminInputTextField
      v-if="editedTag.is_filter"
      id="filter_description"
      v-model="editedTag.filter_description"
      label="Description du filtre"
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

const editedTag: Ref<NewOrUpdateTag> = ref({
  title: '',
  is_filter: true,
  default_filter_status: true,
  filter_description: '',
})

const processingRequest = ref(false)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Création d'un nouveau tag`,
  'tag',
  [],
  [
    { label: 'Tags', url: '/admin/tags' },
    { label: `Création d'un nouveau tag`, url: `/admin/tags/new` },
  ],
)

async function onSave() {
  processingRequest.value = true
  await state.client.createTag(editedTag.value)
  navigateTo('/admin/tags')
}
</script>
