<template>
  <Button
    label="Nouveau commentaire"
    rounded
    outlined
    @click="formVisible=true"
  >
    <template #default>
      <div class="flex align-items-center">
        <AppIcon
          class="-ml-1 mr-1"
          icon-name="commentAdd"
        />
        Ajouter un commentaire
      </div>
    </template>
  </Button>

  <Dialog
    v-model:visible="formVisible"
    modal
    closable
    :header="props.family.comment_form.title"
    :content-props="{ onClick: (event) => { event.stopPropagation() } }"
  >
    <form
      v-if="curr_page == 0"
      class="flex flex-grow-1 flex-column gap-3 w-30rem"
      @submit.prevent="curr_page+=1"
    >
      <AdminInputTextField
        id="author"
        v-model="editedComment!.author"
        label="Auteur"
      />

      <AdminInputTextField
        id="text"
        v-model="editedComment!.text"
        label="Texte du commentaire"
        text-length="editor"
      />

      <span class="flex gap-1 justify-content-end">
        <Button
          label="Suivant"
          type="submit"
          outlined
          :disabled="!editedComment!.author || !editedComment!.text"
        />
      </span>
    </form>
    <form
      v-for="page in Array.from({ length: page_count }, (_, i) => i+1)"
      :key="`Page ${page}`"
      class="flex flex-grow-1 flex-column gap-3 w-30rem"
      @submit.prevent="() => page == page_count ? onSave() : curr_page+=1"
    >
      <div
        v-if="curr_page == page"
        class="flex flex-grow-1 flex-column gap-3 max-w-30rem"
      >
        <FormDynamicField
          v-for="field in props.family.comment_form.fields.toSorted((field_a, field_b) => field_a.form_weight - field_b.form_weight)"
          :key="field.key"
          v-model:fieldContent="(editedComment!.data as EntityOrCommentData)[field.key]"
          :form-field="(field as FormField)"
        />

        <span
          class="flex gap-1 justify-content-end"
        >
          <Button
            label="Précédent"
            outlined
            @click="curr_page -= 1"
          />
          <Button
            :label="page == page_count ? 'Sauvegarder' : 'Suivant'"
            type="submit"
            :outlined="page != page_count"
            :loading="processingRequest"
            :disabled="processingRequest || !editedComment!.author || !editedComment!.text"
          />
        </span>
      </div>
    </form>
  </Dialog>
</template>

<script setup lang="ts">
import type { EntityOrCommentData, Family, FormField, PublicEntity, PublicNewComment } from '~/lib'
import state from '~/lib/viewer-state'

const formVisible = ref(false)

const props = defineProps<{
  family: Family
  entity: PublicEntity
}>()

const editedComment = ref<PublicNewComment>()

const curr_page = ref(0)
const page_count = ref(0)

function reset_refs(new_entity_id: string) {
  editedComment.value = {
    author: '',
    data: {},
    entity_id: new_entity_id,
    text: '',
  }
  curr_page.value = 0
  page_count.value = Math.max(0, ...props.family.comment_form.fields.map(field => field.form_page))
}
reset_refs(props.entity.id)

watch(
  () => props.entity,
  (newEntity, _) => {
    reset_refs(newEntity.id)
  },
)
const processingRequest = ref(false)
const toast = useToast()

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.createComment({ comment: editedComment.value! })
    formVisible.value = false
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Commentaire modifié avec succès', life: 3000 })
    reset_refs(props.entity.id)
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification du commentaire', life: 3000 })
  }
  processingRequest.value = false
}
</script>
