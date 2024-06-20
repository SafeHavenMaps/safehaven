<template>
  <Dialog
    v-model:visible="formVisible"
    modal
    closable
    class="w-full  max-w-[30rem]"
    :header="props.family.entity_form.title"
    :content-props="{ onClick: (event: Event) => { event.stopPropagation() } }"
  >
    <!-- Entity Form Pages -->
    <form
      v-for="page in Array.from({ length: entityPageCount+1 }, (_, i) => i)"
      :key="`EntityPage${page}`"
      class="flex grow flex-col gap-4"
      @submit.prevent="curr_page += 1"
    >
      <div
        v-if="curr_page == page"
        class="flex grow flex-col gap-4"
      >
        <template v-if="page == 0">
          <AdminInputTextField
            id="display_name"
            v-model="editedEntity.display_name"
            label="Nom d'affichage"
          />

          <FormCategorySelect
            v-model="editedEntity.category_id"
            :categories="props.categories"
          />

          <FormAdresses
            v-model:locations="editedEntity!.locations"
          />
        </template>
        <template v-else>
          <FormDynamicField
            v-for="field in entityFieldsSortedByPage(page)"
            :key="field.key"
            v-model:fieldContent="(editedEntity.data as EntityOrCommentData)[field.key]"
            :form-field="(field as FormField)"
            @is-valid="isValid => entityFieldValid[field.key]= isValid"
          />
        </template>

        <span class="flex gap-1 justify-end">
          <Button
            v-if="page > 0"
            label="Précédent"
            outlined
            @click="curr_page -= 1"
          />
          <Button
            :label="curr_page == entityPageCount ? 'Suivant' : 'Suivant'"
            type="submit"
            outlined
            :disabled="!isEntityPageValid(page)"
          />
        </span>
      </div>
    </form>

    <!-- Comment Form Pages -->
    <form
      v-for="page in Array.from({ length: commentPageCount+1 }, (_, i) => i)"
      :key="`CommentPage${page}`"
      class="flex grow flex-col gap-4 max-w-[30rem]"
      @submit.prevent="curr_page < (entityPageCount + commentPageCount + 1) ? curr_page += 1 : onSave()"
    >
      <div
        v-if="curr_page == entityPageCount + 1 + page"
        class="flex grow flex-col gap-4"
      >
        <template v-if="page == 0">
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
        </template>
        <template v-else>
          <FormDynamicField
            v-for="field in commentFieldsSortedByPage(page)"
            :key="field.key"
            v-model:fieldContent="(editedComment.data as EntityOrCommentData)[field.key]"
            :form-field="(field as FormField)"
            @is-valid="isValid => commentFieldValid[field.key]= isValid"
          />
        </template>

        <span class="flex gap-1 justify-end">
          <Button
            label="Précédent"
            outlined
            @click="curr_page -= 1"
          />
          <Button
            :label="curr_page == (entityPageCount + commentPageCount + 1) ? 'Sauvegarder' : 'Suivant'"
            type="submit"
            :outlined="curr_page != (entityPageCount + commentPageCount + 1)"
            :loading="processingRequest"
            :disabled="processingRequest || !isCommentPageValid(page)"
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
import { ref, defineProps } from 'vue'
import { useToast } from 'primevue/usetoast'
import type { Category, EntityOrCommentData, Family, FormField, PublicNewComment, PublicNewEntity } from '~/lib'
import state from '~/lib/viewer-state'

defineExpose({ open, close, toggle })

function open() {
  formVisible.value = true
}

function close() {
  formVisible.value = false
}

function toggle() {
  formVisible.value = !formVisible.value
}

const formVisible = ref(false)

const props = defineProps<{
  family: Family
  categories: Category[]
}>()

const editedEntity = ref<PublicNewEntity>({
  category_id: '',
  data: {},
  display_name: '',
  locations: [],
})

const editedComment = ref<PublicNewComment>({
  author: '',
  data: {},
  entity_id: '00000000-0000-4000-8000-000000000000',
  text: '',
})

const curr_page = ref(0)
const entityPageCount = ref(0)
const commentPageCount = ref(0)

const entityFieldValid = ref(
  props.family.entity_form.fields
    .reduce((acc, field) => {
      acc[field.key] = !field.mandatory
      return acc
    }, {} as Record<string, boolean>),
)

const commentFieldValid = ref(
  props.family.comment_form.fields
    .reduce((acc, field) => {
      acc[field.key] = !field.mandatory
      return acc
    }, {} as Record<string, boolean>),
)

function reset_refs() {
  editedEntity.value = {
    category_id: '',
    data: {},
    display_name: '',
    locations: [],
  }
  editedComment.value = {
    author: '',
    data: {},
    entity_id: '00000000-0000-4000-8000-000000000000',
    text: '',
  }
  curr_page.value = 0
  entityPageCount.value = Math.max(
    0,
    ...props.family.entity_form.fields.map(field => field.form_page),
  )
  commentPageCount.value = Math.max(
    0,
    ...props.family.comment_form.fields.map(field => field.form_page),
  )
}
reset_refs()

watch(
  () => props.family,
  (__, _) => {
    reset_refs()
  },
)

const processingRequest = ref(false)
const showCaptcha = ref(false)
const toast = useToast()

function entityFieldsSortedByPage(page: number) {
  return props.family.entity_form.fields
    .filter(field => field.form_page === page)
    .sort((field_a, field_b) => field_a.form_weight - field_b.form_weight)
}

function commentFieldsSortedByPage(page: number) {
  return props.family.comment_form.fields
    .filter(field => field.form_page === page)
    .sort((field_a, field_b) => field_a.form_weight - field_b.form_weight)
}

function isEntityPageValid(page: number) {
  if (page === 0) {
    return editedEntity.value.display_name && editedEntity.value.category_id
  }
  return entityFieldsSortedByPage(page).every(field => entityFieldValid.value[field.key])
}

function isCommentPageValid(page: number) {
  if (page === 0) {
    return editedComment.value.author && editedComment.value.text
  }
  return commentFieldsSortedByPage(page).every(field => commentFieldValid.value[field.key])
}

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
    await state.client.createEntity({
      entity: editedEntity.value,
      comment: editedComment.value,
      hcaptcha_token: token,
    })
    formVisible.value = false
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Entité et commentaire ajoutés avec succès', life: 3000 })
    reset_refs()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur lors de l\'ajout de l\'entité ou du commentaire', life: 3000 })
  }
  processingRequest.value = false
}
</script>
