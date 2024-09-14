<template>
  <form
    class="flex flex-wrap gap-8 mx-6"
    @submit.prevent="onSave"
  >
    <div class="flex grow flex-col gap-4 max-w-[30rem]">
      <AdminInputTextField
        id="author"
        v-model="editedComment.author"
        label="Auteur"
        :variant="hasBeenEdited('author')"
      />

      <div class="flex flex-col gap-2">
        <label for="comment_text">Texte du commentaire <RequiredIndicator /></label>
        <ViewerRichTextEditor
          id="comment_text"
          v-model="editedComment.text"
          label="Texte du commentaire"
        />
      </div>

      <FormDynamicField
        v-for="field in family.comment_form.fields
          .filter(field => field.categories == null || field.categories.includes(parentEntityToDisplay!.category_id))
          .toSorted((field_a, field_b) => field_a.form_weight - field_b.form_weight)"
        :key="field.key"
        v-model:field-content="(editedComment.data as EntityOrCommentData)[field.key]"
        :form-field="(field as FormField)"
      />
    </div>

    <div class="flex flex-col grow gap-4 max-w-[30rem]">
      <span v-if="!isNew">Commentaire créé le
        {{ Intl.DateTimeFormat('fr-FR', {
          dateStyle: 'long',
          timeStyle: 'short',
        }).format(new Date(fetchedComment!.created_at)) }}, mise à jour pour la dernière fois le
        {{ Intl.DateTimeFormat('fr-FR', {
          dateStyle: 'long',
          timeStyle: 'short',
        }).format(new Date(fetchedComment!.updated_at)) }}
      </span>
      <span>
        Rattaché à {{ parentEntityToDisplay?.display_name }} <CategoryTag
          v-if="parentEntityToDisplay?.category_id"
          :category="state.categoryRecord[parentEntityToDisplay!.category_id]"
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
          editedComment.entity_id = entity.entity_id
          parentEntityToDisplay = entity
        }"
      />

      <AdminInputSwitchField
        id="moderated"
        v-model="editedComment.moderated"
        label="Modérée"
        helper-text="Si activé, cette entité quittera la liste des entités en attente et sera rendue publique."
      />
      <span class="flex gap-1 justify-end">
        <NuxtLink :to="returnUrl">
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
const isNew = (commentId === 'new')
const urlEntityId = useRoute().query.urlEntityId
const returnUrl = urlEntityId == null ? `/admin/${familyId}/comments/pending` : `/admin/${familyId}/entities/${urlEntityId}?comments`

const fetchedComment: AdminComment | null = isNew ? null : await state.client.getComment(commentId)
const parentEntityToDisplay = ref<{ category_id: string, display_name: string }>()
if (fetchedComment) {
  parentEntityToDisplay.value = { category_id: fetchedComment.entity_category_id, display_name: fetchedComment.entity_display_name }
}
else if (urlEntityId) {
  const fetchedParent = await state.client.getEntity(urlEntityId as string)
  parentEntityToDisplay.value = { category_id: fetchedParent.category_id, display_name: fetchedParent.display_name }
}
else {
  parentEntityToDisplay.value = { category_id: '', display_name: '' }
}

const editedComment: Ref<AdminNewOrUpdateComment> = isNew
  ? ref({
    author: '',
    data: {},
    entity_id: urlEntityId ?? '',
    entity_category_id: parentEntityToDisplay.value.category_id,
    moderated: false,
    text: '',
    version: 1,
  })
  : ref(JSON.parse(JSON.stringify(fetchedComment))) // deep copy

const processingRequest = ref(false)
const toast = useToast()

const categories = computed(() => state.categories.filter(category => category.family_id == familyId))

const tags = state.tags

const entitySelectVisible = ref(false)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  isNew ? `Nouveau commentaire` : `Édition du commentaire de ${fetchedComment!.author}`,
  'comment',
  [],
  [
    { label: `${family.title}`, url: '/admin/families' },
    { label: urlEntityId ? `Commentaires de l'entité ${parentEntityToDisplay.value.display_name}` : `Commentaires en attente`, url: returnUrl },
    isNew
      ? { label: `Nouveau commentaire`, url: `/admin/${familyId}/comments/new` }
      : { label: `Édition d'un commentaire`, url: `/admin/${familyId}/comments/${commentId}` },
  ],
)

function hasBeenEdited(field: keyof AdminNewOrUpdateComment) {
  return isNew ? false : editedComment.value[field] !== fetchedComment![field]
}

async function onSave() {
  processingRequest.value = true
  try {
    if (isNew) {
      await state.client.createComment(editedComment.value)
    }
    else {
      await state.client.updateComment(commentId, editedComment.value)
    }
    navigateTo(returnUrl)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Commentaire modifié avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification du commentaire', life: 3000 })
  }
  processingRequest.value = false
}
</script>
