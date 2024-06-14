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
    />

    <AdminInputSwitchField
      id="default_status"
      v-model="editedCategory.default_status"
      label="Inclus par défaut"
      helper-text="(si décoché, toutes les entités appartenant à cette catégorie seront exclues des résultats par défaut)"
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

    <AdminInputIconUpload
      :object-id="categoryId"
      object-type="categories"
    />
    <span class="flex gap-1 justify-content-end">
      <NuxtLink :to="`/admin/${familyId}/categories`">
        <Button
          label="Annuler"
          severity="secondary"
          :disabled="processingRequest"
          :loading="processingRequest"
        />
      </NuxtLink>
      <Button
        label="Sauvegarder"
        type="submit"
        :loading="processingRequest"
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
if (state.families == undefined)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title
const categoryId = useRoute().params.id as string

const fetchedCategory = await state.fetchCategory(categoryId)
const editedCategory: Ref<NewOrUpdateCategory> = ref(JSON.parse(JSON.stringify(fetchedCategory))) // deep copy

const processingRequest = ref(false)
const toast = useToast()
const color_picker_1_invalid = ref(false)
const color_picker_2_invalid = ref(false)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
    `Édition de la catégorie ${fetchedCategory.title}`,
    'category',
    [],
    [
      { label: `${familyTitle}`, url: '/admin/families' },
      { label: 'Catégories', url: `/admin/${familyId}/categories` },
      { label: `Édition de la catégorie ${fetchedCategory.title}`, url: `/admin/${familyId}/categories/${categoryId}` },
    ],
)

function hasBeenEdited(field: keyof NewOrUpdateCategory) {
  return editedCategory.value[field] !== fetchedCategory[field]
}

async function onSave() {
  processingRequest.value = true
  try {
    if (editedCategory.value.border_color.length == 6) {
      editedCategory.value.border_color = `#${editedCategory.value.border_color}`
    }
    if (editedCategory.value.fill_color.length == 6) {
      editedCategory.value.fill_color = `#${editedCategory.value.fill_color}`
    }
    await state.updateCategory(categoryId, editedCategory.value)
    navigateTo(`/admin/${familyId}/categories`)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Catégorie modifiée avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification de la catégorie', life: 3000 })
  }
  processingRequest.value = false
}
</script>
