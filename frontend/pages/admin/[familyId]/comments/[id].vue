<template>
  <form
    class="flex flex-wrap gap-5 mx-4"
    @submit.prevent="onSave"
  >
    <div class="flex flex-grow-1 flex-column gap-3 max-w-30rem">
      <AdminInputTextField
        id="author"
        v-model="editedComment.author"
        label="Auteur"
        :variant="hasBeenEdited('author')"
      />

      <AdminInputTextField
        id="text"
        v-model="editedComment.text"
        label="Texte du commentaire"
        :variant="hasBeenEdited('text')"
        text-length="editor"
      />

      <FormDynamicField
        v-for="field in family.comment_form.fields.toSorted((field_a, field_b) => field_a.form_weight - field_b.form_weight)"
        :key="field.key"
        v-model:fieldContent="(editedComment.data as EntityOrCommentData)[field.key]"
        :form-field="(field as FormField)"
      />
    </div>

    <div class="flex flex-column flex-grow-1 gap-3 max-w-30rem">
      Commentaire créé le
      {{ Intl.DateTimeFormat('fr-FR', {
        dateStyle: 'long',
        timeStyle: 'short',
      }).format(new Date(fetchedComment.created_at)) }}, mise à jour pour la dernière fois le
      {{ Intl.DateTimeFormat('fr-FR', {
        dateStyle: 'long',
        timeStyle: 'short',
      }).format(new Date(fetchedComment.updated_at)) }}

      <span>
        Rattaché à {{ parentEntityToDisplay.display_name }} <CategoryTag
          :category="state.categoryRecord[parentEntityToDisplay.category_id]"
        />

      </span>
      <Button
        label="Modifier l'entité de rattachement"
        outlined
        class="-mt-1 mb-1"
        @click="entitySelectVisible=true"
      />
      <AdminInputEntitySelect
        v-model:visible="entitySelectVisible"
        title="Choix de l'entité de rattachement du commentaire"
        :categories="categories"
        :tags="tags"
        :family-id="familyId"
        :previous-entity-id="editedComment.entity_id"
        @save_entity="entity => {
          editedComment.entity_id = entity.id
          parentEntityToDisplay = entity
        }"
      />

      <AdminInputSwitchField
        id="moderated"
        v-model="editedComment.moderated"
        label="Modérée"
        helper-text="Si activé, cette entité quittera la liste des entités en attente et sera rendue publique."
      />
      <span class="flex gap-1 justify-content-end">
        <NuxtLink :to="`/admin/${familyId}/comments`">
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
          :disabled="processingRequest || !editedComment.author || !editedComment.text"
        />
      </span>
    </div>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminNewOrUpdateComment, AdminComment, EntityOrCommentData, FormField } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const familyId = useRoute().params.familyId as string
if (state.families == undefined)
  await state.fetchFamilies()
if (state.categories == undefined)
  await state.fetchCategories()
if (state.tags == undefined)
  await state.fetchTags()

const family = state.families.filter(family => family.id == familyId)[0]
const commentId = useRoute().params.id as string

const fetchedComment: AdminComment = await state.client.getComment(commentId)
const parentEntityToDisplay = ref<{ category_id: string, display_name: string }>
({ category_id: fetchedComment.category_id, display_name: fetchedComment.entity_display_name })
const editedComment: Ref<AdminNewOrUpdateComment> = ref(JSON.parse(JSON.stringify(fetchedComment))) // deep copy

const processingRequest = ref(false)
const toast = useToast()

const categories = computed(() => state.categories.filter(category => category.family_id == familyId))

const tags = state.tags

const entitySelectVisible = ref(false)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
      `Édition du commentaire de ${fetchedComment.author}`,
      'comment',
      [],
      [
        { label: `${family.title}`, url: '/admin/families' },
        { label: 'Commentaires', url: `/admin/${familyId}/comments` },
        { label: `Édition du commentaire`, url: `/admin/${familyId}/comments/${commentId}` },
      ],
)

function hasBeenEdited(field: keyof AdminNewOrUpdateComment) {
  return editedComment.value[field] !== fetchedComment[field]
}

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.updateComment(commentId, editedComment.value)
    navigateTo(`/admin/${familyId}/comments`)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Commentaire modifié avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification du commentaire', life: 3000 })
  }
  processingRequest.value = false
}
</script>