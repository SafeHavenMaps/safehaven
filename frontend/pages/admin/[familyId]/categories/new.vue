<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedCategory.title"
      label="Titre"
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
    />

    <AdminInputColorField
      id="fill_color"
      v-model="editedCategory.fill_color"
      v-model:invalid="color_picker_2_invalid"
      label="Couleur de remplissage"
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
if (state.families == undefined)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title

const editedCategory: Ref<NewOrUpdateCategory> = ref({
  border_color: '#FFFFFF',
  default_status: true,
  family_id: familyId,
  fill_color: '#000000',
  title: '',
})

const processingRequest = ref(false)
const toast = useToast()
const color_picker_1_invalid = ref(false)
const color_picker_2_invalid = ref(false)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
      `Nouvelle catégorie`,
      'category',
      [],
      [
        { label: `${familyTitle}`, url: '/admin/families' },
        { label: 'Catégories', url: `/admin/${familyId}/categories` },
        { label: `Édition d'une nouvelle catégorie`, url: `/admin/${familyId}/categories/new` },
      ],
)

async function onSave() {
  processingRequest.value = true
  try {
    if (editedCategory.value.border_color.length == 6) {
      editedCategory.value.border_color = `#${editedCategory.value.border_color}`
    }
    if (editedCategory.value.fill_color.length == 6) {
      editedCategory.value.fill_color = `#${editedCategory.value.fill_color}`
    }
    const { id } = await state.client.createCategory(editedCategory.value)
    navigateTo(`/admin/${familyId}/categories/new-icon-${id}`)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Catégorie créée avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de création de la catégorie', life: 3000 })
  }
  processingRequest.value = false
}
</script>
