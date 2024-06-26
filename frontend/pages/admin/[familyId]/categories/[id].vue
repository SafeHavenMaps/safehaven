<template>
  <form
    class="flex flex-col gap-4 max-w-[30rem] mx-6"
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
      label="Couleur de bordure"
      :variant="hasBeenEdited('border_color')"
    />

    <AdminInputColorField
      id="fill_color"
      v-model="editedCategory.fill_color"
      label="Couleur de remplissage"
      :variant="hasBeenEdited('fill_color')"
    />

    <AdminInputIconUpload
      :object-id="categoryId"
      object-type="categories"
    />
    <span class="flex gap-1 justify-end">
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
        :disabled="isDisabled()"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateCategory } from '~/lib'
import state from '~/lib/admin-state'
import { isValidHexColor, isValidText } from '~/lib/validation'

definePageMeta({
  layout: 'admin-ui',
})

// Load family
const familyId = useRoute().params.familyId as string
if (state.families == undefined)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title

// Load category
const categoryId = useRoute().params.id as string
const isNew = (categoryId === 'new')

const fetchedCategory = isNew ? null : await state.fetchCategory(categoryId)
const editedCategory: Ref<NewOrUpdateCategory> = ref(isNew
  ? {
      border_color: '#FFFFFF',
      default_status: true,
      family_id: familyId,
      fill_color: '#000000',
      title: '',
    }
  : JSON.parse(JSON.stringify(fetchedCategory!)))

const processingRequest = ref(false)
const toast = useToast()

function isDisabled() {
  return processingRequest.value
    || !isValidText(editedCategory.value.title)
    || !isValidHexColor(editedCategory.value.border_color)
    || !isValidHexColor(editedCategory.value.fill_color)
}

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  isNew ? 'Nouvelle catégorie' : `Édition de la catégorie ${fetchedCategory!.title}`,
  'category',
  [],
  [
    { label: `${familyTitle}`, url: '/admin/families' },
    { label: 'Catégories', url: `/admin/${familyId}/categories` },
    (
      isNew
        ? { label: `Édition d'une nouvelle catégorie`, url: `/admin/${familyId}/categories/new` }
        : { label: `Édition de la catégorie ${fetchedCategory!.title}`, url: `/admin/${familyId}/categories/${categoryId}` }
    ),
  ],
)

function hasBeenEdited(field: keyof NewOrUpdateCategory) {
  return isNew
    ? false
    : editedCategory.value[field] !== fetchedCategory![field]
}

async function onSave() {
  processingRequest.value = true
  try {
    if (isNew) {
      const { id } = await state.createCategory(editedCategory.value)
      navigateTo(`/admin/${familyId}/categories/new-icon-${id}`)
      toast.add({
        severity: 'success',
        summary: 'Succès',
        detail: 'Catégorie créée avec succès',
        life: 3000,
      })
    }
    else {
      await state.updateCategory(categoryId, editedCategory.value)
      navigateTo(`/admin/${familyId}/categories`)
      toast.add({
        severity: 'success',
        summary: 'Succès',
        detail: 'Catégorie modifiée avec succès',
        life: 3000,
      })
    }
  }
  catch {
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: 'Erreur de modification de la catégorie',
      life: 3000,
    })
  }
  processingRequest.value = false
}
</script>
