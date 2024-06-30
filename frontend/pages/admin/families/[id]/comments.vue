<template>
  <div>
    <p class="text-muted-color">
      Une page additionelle est insérée automatiquement au début du formulaire,
      contenant le nom de l'auteur.ice ainsi qu'un champ libre.
    </p>

    <AdminFamiliesEditForm
      :original-form-fields="fetchedFamily.comment_form.fields"
      :on-save-callback="onSave"
      kind-name="commentaire"
      kind="comment"
    />
  </div>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { FormField } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const id = useRoute().params.id as string

const fetchedFamily = await state.client.getFamily(id)
const editedFamily = JSON.parse(JSON.stringify(fetchedFamily))

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
    `Édition du formulaire d'ajout de commentaires de la famille ${fetchedFamily.title}`,
    'family',
    [],
    [
      { label: 'Familles', url: '/admin/families' },
      { label: `Édition du formulaire d'ajout de commentaires de la famille ${fetchedFamily.title}`, url: `/admin/families/${id}/comments` },
    ],
)

async function onSave(newFormFields: FormField[]): Promise<{ error: Error | undefined }> {
  try {
    editedFamily.comment_form.fields = newFormFields
    await state.client.updateFamily(id, editedFamily)
    navigateTo('/admin/families')
    return { error: undefined }
  }
  catch (error) {
    return { error: error as Error }
  }
}
</script>
