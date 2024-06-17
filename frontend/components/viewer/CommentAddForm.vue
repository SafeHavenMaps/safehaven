<template>
  <Button
    label="Nouveau commentaire"
    outlined
    @click="formVisible=true"
  />
  <div @click.stop>
    <Dialog
      v-model:visible="formVisible"
      modal
      :closable="false"
      :header="props.family.comment_form.title"
      :content-props="{ onClick: (event) => { event.stopPropagation() } }"
    >
      <template #header>
        <div @click.stop>
          {{ props.family.comment_form.title }}
        </div>
      </template>
      <form
        class="flex flex-wrap gap-5 mx-4"
        @submit.prevent="onSave"
        @click.stop
      >
        <div
          class="flex flex-grow-1 flex-column gap-3 max-w-30rem"
        >
          <AdminInputTextField
            id="author"
            v-model="editedComment.author"
            label="Auteur"
          />

          <AdminInputTextField
            id="text"
            v-model="editedComment.text"
            label="Texte du commentaire"
            text-length="editor"
          />

          <FormDynamicField
            v-for="field in props.family.comment_form.fields.toSorted((field_a, field_b) => field_a.form_weight - field_b.form_weight)"
            :key="field.key"
            v-model:fieldContent="(editedComment.data as EntityOrCommentData)[field.key]"
            :form-field="(field as FormField)"
          />
          <span class="flex gap-1 justify-content-end">
            <Button
              label="Annuler"
              severity="secondary"
              :disabled="processingRequest"
              :loading="processingRequest"
              @click="formVisible = false"
            />
            <Button
              label="Sauvegarder"
              type="submit"
              :loading="processingRequest"
              :disabled="processingRequest || !editedComment.author || !editedComment.text"
            />
          </span>
        </div>
      </form>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { EntityOrCommentData, Family, FormField, PublicEntity, PublicNewComment } from '~/lib'
import state from '~/lib/viewer-state'

const formVisible = ref(false)

const props = defineProps<{
  family: Family
  entity: PublicEntity
}>()

const editedComment: Ref<PublicNewComment> = ref({
  author: '',
  data: {},
  entity_id: props.entity.id,
  text: '',
})

watch(
  () => props.entity,
  (newEntity, _) => {
    editedComment.value = {
      author: '',
      data: {},
      entity_id: newEntity.id,
      text: '',
    }
  },
)
const processingRequest = ref(false)
const toast = useToast()

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.createComment({ comment: editedComment.value })
    formVisible.value = false
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Commentaire modifié avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification du commentaire', life: 3000 })
  }
  processingRequest.value = false
}
</script>
