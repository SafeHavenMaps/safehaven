<template>
  <form
    class="flex flex-col gap-4 max-w-[30rem] mx-6"
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

    <AdminInputSwitchField
      v-if="editedTag.is_filter"
      id="is_filter"
      v-model="editedTag.is_primary_filter"
      label="Mettre en avant le filtre"
      helper-text="(si coché, le filtre sera présent dans le panneau de filtrage en dessous du choix des catégories)"
    />

    <AdminInputTextField
      v-if="editedTag.is_filter && editedTag.is_primary_filter"
      id="filter_description"
      v-model="editedTag.filter_description"
      label="Description du filtre"
      :variant="hasBeenEdited('filter_description')"
      helper-text="(description exposée aux utilisateurices lorsqu'il est mis en avant)"
    />

    <AdminInputColorField
      id="border_color"
      v-model="editedTag.border_color"
      label="Couleur de bordure"
      :variant="hasBeenEdited('border_color')"
    />

    <AdminInputColorField
      id="fill_color"
      v-model="editedTag.fill_color"
      label="Couleur de remplissage"
      :variant="hasBeenEdited('fill_color')"
    />

    <span class="flex gap-1 justify-end">
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
        :disabled="isDisabled()"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateTag } from '~/lib'
import state from '~/lib/admin-state'
import { isValidHexColor, isValidText } from '~/lib/validation'

definePageMeta({
  layout: 'admin-ui',
})

const tagId = useRoute().params.id as string

const isNew = (tagId === 'new')
const fetchedTag = isNew ? null : await state.fetchTag(tagId)
const editedTag: Ref<NewOrUpdateTag> = ref(isNew
  ? {
      title: '',
      is_filter: true,
      default_filter_status: true,
      filter_description: '',
      border_color: '#deb9c9',
      fill_color: '#824261',
    }
  : JSON.parse(JSON.stringify(fetchedTag)),
)

const processingRequest = ref(false)
const toast = useToast()

function isDisabled() {
  return processingRequest.value
    || !isValidText(editedTag.value.title)
    || (editedTag.value.is_filter && !isValidText(editedTag.value.filter_description))
    || !isValidHexColor(editedTag.value.border_color)
    || !isValidHexColor(editedTag.value.fill_color)
}

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  isNew ? `Création d'un nouveau tag` : `Édition du jeton ${fetchedTag!.title}`,
  'tag',
  [],
  isNew
    ? [
        { label: 'Tags', url: '/admin/tags' },
        { label: `Création d'un nouveau tag`, url: `/admin/tags/new` },
      ]
    : [
        { label: 'Jetons d\'accès', url: '/admin/tags' },
        { label: `Édition du jeton ${fetchedTag!.title}`, url: `/admin/tags/${tagId}` },
      ],
)

function hasBeenEdited(field: keyof NewOrUpdateTag) {
  return isNew
    ? false
    : editedTag.value[field] !== fetchedTag![field]
}

async function onSave() {
  try {
    processingRequest.value = true
    if (isNew) {
      await state.createTag(editedTag.value)
      toast.add({
        severity: 'success',
        summary: 'Succès',
        detail: 'Tag créé avec succès',
        life: 3000,
      })
    }
    else {
      await state.updateTag(tagId, editedTag.value)
      toast.add({
        severity: 'success',
        summary: 'Succès',
        detail: 'Tag modifié avec succès',
        life: 3000,
      })
    }
    navigateTo('/admin/tags')
  }
  catch (error) {
    console.error(error)
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: (isNew ? 'Erreur de création du tag' : 'Erreur de modification du tag'),
      life: 3000,
    })
  }

  processingRequest.value = false
}
</script>
