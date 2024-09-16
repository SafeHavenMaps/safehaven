<template>
  <Dialog
    v-model:visible="formVisible"
    modal
    closable
    class="w-full  max-w-[30rem]"
    :header="props.family.entity_form.title"
  >
    <!-- Entity Form Pages -->
    <form
      v-for="page in Array.from({ length: entityPageCount+1 }, (_, i) => i)"
      :key="`EntityPage${page}`"
      class="flex grow flex-col gap-4"
      @submit.prevent="include_comment || curr_page < lastNonCaptchaPage ? curr_page += 1 : onSave()"
    >
      <div
        v-if="curr_page == page"
        class="flex grow flex-col gap-4"
      >
        <template v-if="page == 0">
          <span v-if="props.family.entity_form.help">{{ props.family.entity_form.help }}</span>
          <AdminInputTextField
            id="display_name"
            v-model="editedEntity.display_name"
            :label="$t('cmp.viewer.entityAddForm.displayName')"
          />

          <FormCategorySelect
            v-model="editedEntity.category_id"
            :categories="props.categories"
            @update:model-value="(new_category_id: string) => editedComment.entity_category_id = new_category_id"
          />

          <FormAdresses
            v-model:locations="editedEntity!.locations"
          />

          <div
            v-if="state.permissions?.can_add_comment"
            class="flex flex-col gap-2"
          >
            <span class="flex items-center gap-2">
              <ToggleSwitch
                class="shrink-0"
                :model-value="include_comment"
                input-id="include_comment"
                @change="(event: Event) => onIncludeCommentToggle(event)"
              />
              <label for="include_comment">{{ $t('cmp.viewer.entityAddForm.includeComment') }}</label>
            </span>
          </div>
        </template>
        <template v-else>
          <FormDynamicField
            v-for="field in entityFieldsSortedByPage(page)
              .filter(field => field.categories == null || field.categories.includes(editedEntity.category_id))"
            :key="field.key"
            v-model:field-content="(editedEntity.data as EntityOrCommentData)[field.key]"
            :form-field="(field as FormField)"
            @is-valid="isValid => entityFieldValid[field.key]= isValid"
          />
        </template>

        <span class="flex gap-1 justify-end">
          <Button
            v-if="page > 0"
            :label="$t('cmp.viewer.entityAddForm.previous')"
            outlined
            @click="curr_page -= 1"
          />
          <Button
            :label="include_comment || curr_page < entityPageCount ? $t('cmp.viewer.entityAddForm.next') : $t('cmp.viewer.entityAddForm.save')"
            type="submit"
            :outlined="include_comment || curr_page < entityPageCount"
            :disabled="!isEntityPageValid(page)"
          />
        </span>
      </div>
    </form>

    <!-- Comment Form Pages -->
    <form
      v-for="page in Array.from({ length: commentPageCount + 2 }, (_, i) => i)"
      :key="`CommentPage${page}`"
      class="flex grow flex-col gap-4 max-w-[30rem]"
      @submit.prevent="curr_page < lastNonCaptchaPage ? curr_page += 1 : onSave()"
    >
      <div
        v-if="curr_page == entityPageCount + 1 + page"
        class="flex grow flex-col gap-4"
      >
        <template v-if="page == 0">
          <span v-if="props.family.comment_form.help">{{ props.family.comment_form.help }}</span>
          <AdminInputTextField
            id="author"
            v-model="editedComment.author"
            :label="$t('cmp.viewer.entityAddForm.author')"
          />

          <div class="flex flex-col gap-2">
            <label for="comment_text">{{ $t('cmp.viewer.entityAddForm.commentText') }}<RequiredIndicator /></label>
            <ViewerRichTextEditor
              id="comment_text"
              v-model="editedComment!.text"
              label="Texte du commentaire"
            />
          </div>
        </template>
        <template v-else-if="page < (commentPageCount + 1)">
          <FormDynamicField
            v-for="field in commentFieldsSortedByPage(page)
              .filter(field => field.categories == null || field.categories.includes(editedEntity.category_id))"
            :key="field.key"
            v-model:field-content="(editedComment.data as EntityOrCommentData)[field.key]"
            :form-field="(field as FormField)"
            @is-valid="isValid => commentFieldValid[field.key]= isValid"
          />
        </template>

        <span
          v-if="page < (commentPageCount + 1)"
          class="flex gap-1 justify-end"
        >
          <Button
            :label="$t('cmp.viewer.entityAddForm.previous')"
            outlined
            @click="curr_page -= 1"
          />
          <Button
            :label="curr_page == lastNonCaptchaPage ? $t('cmp.viewer.entityAddForm.save') : $t('cmp.viewer.entityAddForm.next')"
            type="submit"
            :outlined="curr_page != lastNonCaptchaPage"
            :loading="processingRequest"
            :disabled="processingRequest || !isCommentPageValid(page)"
          />
        </span>
        <div
          v-if="page == (commentPageCount + 1)"
          class="flex flex-col justify-center items-center "
        >
          <div class="text-center font-bold">
            {{ $t('cmp.viewer.entityAddForm.captchaCheck') }}
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
      </div>
    </form>
  </Dialog>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'
import type { __String } from 'typescript'
import type { Category, EntityOrCommentData, Family, FormField, PublicNewComment, PublicNewEntity } from '~/lib'
import state from '~/lib/viewer-state'
import { isValidRichText, isValidText } from '~/lib/validation'

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
  entity_category_id: '',
})

const curr_page = ref(0)
const entityPageCount = ref(0)
const commentPageCount = ref(0)

const include_comment = ref(!!state.permissions?.can_add_comment)
const lastNonCaptchaPage = computed(() => include_comment.value ? entityPageCount.value + commentPageCount.value + 1 : entityPageCount.value)
const captchaPage = computed(() => entityPageCount.value + commentPageCount.value + 2)

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
    entity_category_id: '',
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

watch(
  () => formVisible.value,
  (__, _) => {
    curr_page.value = Math.min(curr_page.value, lastNonCaptchaPage.value)
  },
)

const processingRequest = ref(false)
const toast = useToast()
const { t } = useI18n()

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
    return isValidText(editedEntity.value.display_name) && isValidText(editedEntity.value.category_id)
  }
  return entityFieldsSortedByPage(page).every(field => entityFieldValid.value[field.key])
}

function isCommentPageValid(page: number) {
  if (page === 0) {
    return isValidText(editedComment.value.author) && isValidRichText(editedComment.value.text)
  }
  return commentFieldsSortedByPage(page).every(field => commentFieldValid.value[field.key])
}

function hCaptchaVerify(token: string) {
  realOnSave(token)
}

function hCaptchaExpired() {
  toast.add({
    severity: 'error',
    summary: t('cmp.viewer.entityAddForm.error'),
    detail: t('cmp.viewer.entityAddForm.captchaExpired'),
    life: 3000,
  })
}

function hCaptchaError() {
  toast.add({
    severity: 'error',
    summary: 'Erreur',
    detail: t('cmp.viewer.entityAddForm.captchaValidationError'),
    life: 3000,
  })
}

async function onSave() {
  if (state.hasSafeModeEnabled) {
    curr_page.value = captchaPage.value
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
      comment: include_comment.value ? editedComment.value : null,
      hcaptcha_token: token,
    })
    formVisible.value = false
    toast.add({ severity: 'success', summary: t('cmp.viewer.entityAddForm.success'),
      detail: include_comment.value ? t('cmp.viewer.entityAddForm.entityAndCommentSuccess') : t('cmp.viewer.entityAddForm.entitySuccess'), life: 3000 })
    reset_refs()
  }
  catch {
    toast.add({ severity: 'error', summary: t('cmp.viewer.entityAddForm.error'),
      detail: include_comment.value ? t('cmp.viewer.entityAddForm.entityAndCommentError') : t('cmp.viewer.entityAddForm.entityError'), life: 3000 })
  }
  processingRequest.value = false
}

const confirm = useConfirm()
function onIncludeCommentToggle(event: Event) {
  if (!include_comment.value) {
    include_comment.value = true
  }
  else {
    const options: ConfirmationOptions = {
      target: event.currentTarget as HTMLElement,
      group: 'confirm_dialog',
      message: `Ne pas inclure de commentaire ne vous laissera préciser que des informations générales. 
      Si vous avez des retours ou une expérience personelle à partager, 
      il est recommandé d'inclure un commentaire.`,
      header: 'Confirmation',
      icon: 'warning',
      rejectClass: 'p-button-secondary p-button-outlined p-button-sm',
      acceptClass: 'p-button-sm',
      rejectLabel: 'Ne pas inclure de commentaire',
      acceptLabel: 'Annuler',
      reject: () => {
        include_comment.value = false
      },
      accept: () => {},
    }
    confirm.require(options)
  }
}
</script>
