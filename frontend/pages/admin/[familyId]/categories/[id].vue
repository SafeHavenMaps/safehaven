<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedCategory.title"
      label="Titre"
      :variant="hasBeenEdited('title')"
      :invalid="!editedCategory.title"
    />

    <AdminInputColorField
      id="border_color"
      v-model="editedCategory.border_color"
      v-model:invalid="color_picker_1_invalid"
      label="Couleur de bordure"
      :variant="hasBeenEdited('border_color')"
    />

    <AdminInputColorField
      id="fill_color"
      v-model="editedCategory.fill_color"
      v-model:invalid="color_picker_2_invalid"
      label="Couleur de remplissage"
      :variant="hasBeenEdited('fill_color')"
    />

    <!-- <AdminInputTextField
      id="icon_hash"
      v-model="editedCategory.icon_hash"
      label="Hash de l'icône"
    /> -->
    <AdminInputSwitchField
      id="default_status"
      v-model="editedCategory.default_status"
      label="Inclus par défaut"
      helper-text="(si décoché, toutes les entités appartenant à cette catégorie seront exclues des résultats par défaut)"
    />
    <span class="flex gap-1 justify-content-end">
      <NuxtLink :to="`/admin/${familyId}/categories`">
        <Button
          label="Annuler"
          severity="secondary"
          :disabled="processingRequest"
        />
      </NuxtLink>
      <Button
        label="Sauvegarder"
        type="submit"
        :disabled="processingRequest || !editedCategory.title || color_picker_1_invalid || color_picker_2_invalid"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateCategory } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const familyId = useRoute().params.familyId as string
const categoryId = useRoute().params.id as string

const fetchedCategory = await state.client.getCategory(categoryId)
const editedCategory: Ref<NewOrUpdateCategory> = ref(JSON.parse(JSON.stringify(fetchedCategory))) // deep copy

const processingRequest = ref(false)
const color_picker_1_invalid = ref(false)
const color_picker_2_invalid = ref(false)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
    `Édition de la catégorie ${fetchedCategory.title}`,
    'category',
    [],
    [
      { label: 'Catégories', url: '/admin/categories' },
      { label: `Édition de la catégorie ${fetchedCategory.title}`, url: `/admin/categories/${categoryId}` },
    ],
)

function hasBeenEdited(field: keyof NewOrUpdateCategory) {
  if (field == 'icon') return false
  return editedCategory.value[field] !== fetchedCategory[field]
}

async function onSave() {
  processingRequest.value = true
  await state.client.updateCategory(categoryId, editedCategory.value)
  navigateTo(`/admin/${familyId}/categories`)
}
</script>
