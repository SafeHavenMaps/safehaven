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
    class="w-full max-w-md"
    :header="props.family.comment_form.title"
    :content-props="{ onClick: (event: Event) => { event.stopPropagation() } }"
  >
    <form
      v-if="curr_page == 0"
      class="flex grow flex-col gap-4"
      @submit.prevent="curr_page+=1"
    >
      <AdminInputTextField
        id="author"
        v-model="editedComment!.author"
        label="Auteur"
      />

      <div class="flex flex-col gap-2">
        <label for="comment_text">Texte du commentaire<RequiredIndicator /></label>
        <ViewerRichTextEditor
          id="comment_text"
          v-model="editedComment!.text"
          label="Texte du commentaire"
        />
      </div>

      <span class="flex gap-1 justify-end">
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
      class="flex grow flex-col gap-4 w-[30rem]"
      @submit.prevent="() => page == page_count ? onSave() : curr_page+=1"
    >
      <div
        v-if="!showCaptcha && curr_page == page"
        class="flex grow flex-col gap-4 max-w-[30rem]"
      >
        <FormDynamicField
          v-for="field in props.family.comment_form.fields.toSorted((field_a, field_b) => field_a.form_weight - field_b.form_weight)"
          :key="field.key"
          v-model:fieldContent="(editedComment!.data as EntityOrCommentData)[field.key]"
          :form-field="(field as FormField)"
        />

        <span
          class="flex gap-1 justify-end"
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
      <div
        v-if="showCaptcha"
        class="flex flex-col justify-center items-center "
      >
        <div class="text-center font-bold">
          Une petite seconde, on doit vérifier que vous n'êtes pas un robot...
        </div>

        <div class="m-3">
          <vue-hcaptcha
            :sitekey="state.hCaptchaSiteKey"
            @verify="hCaptchaVerify"
            @expired="hCaptchaExpired"
            @error="hCaptchaError"
          />
        </div>
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
const showCaptcha = ref(false)
const toast = useToast()

function hCaptchaVerify(token: string) {
  realOnSave(token)
}

function hCaptchaExpired() {
  toast.add({
    severity: 'error',
    summary: 'Erreur',
    detail: 'Le captcha a expiré',
    life: 3000,
  })
}

function hCaptchaError() {
  toast.add({
    severity: 'error',
    summary: 'Erreur',
    detail: 'Erreur de validation du captcha',
    life: 3000,
  })
}

async function onSave() {
  if (state.hasSafeMode) {
    showCaptcha.value = true
  }
  else {
    await realOnSave(null)
  }
}

async function realOnSave(token: string | null) {
  processingRequest.value = true
  try {
    await state.client.createComment({
      comment: editedComment.value!,
      hcaptcha_token: token,
    })
    formVisible.value = false
    toast.add({
      severity: 'success',
      summary: 'Succès',
      detail: 'Commentaire modifié avec succès',
      life: 3000,
    })
    reset_refs(props.entity.id)
  }
  catch {
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: 'Erreur de modification du commentaire',
      life: 3000,
    })
  }
  processingRequest.value = false
}
</script>
