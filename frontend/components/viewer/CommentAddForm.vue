<template>
  <Button
    :label="$t('cmp.viewer.commentAddForm.newComment')"
    rounded
    outlined
    @click="formVisible=true"
  >
    <template #default>
      <div class="flex items-center">
        <AppIcon
          class="-ml-1 mr-1"
          icon-name="commentAdd"
        />
        {{ $t('cmp.viewer.commentAddForm.addComment') }}
      </div>
    </template>
  </Button>

  <Dialog
    v-model:visible="formVisible"
    modal
    closable
    class="w-full max-w-[30rem]"
    :header="props.family.comment_form.title"
    :content-props="{ onClick: (event: Event) => { event.stopPropagation() } }"
  >
    <form
      v-if="curr_page == 0"
      class="flex grow flex-col gap-4"
      @submit.prevent="curr_page+=1"
    >
      <span v-if="props.family.comment_form.help">{{ props.family.comment_form.help }}</span>
      <AdminInputTextField
        id="author"
        v-model="editedComment!.author"
        :label="$t('cmp.viewer.commentAddForm.author')"
      />

      <div class="flex flex-col gap-2">
        <label for="comment_text">{{ $t('cmp.viewer.commentAddForm.commentText') }}<RequiredIndicator /></label>
        <ViewerRichTextEditor
          id="comment_text"
          v-model="editedComment!.text"
          label="Texte du commentaire"
        />
      </div>

      <span class="flex gap-1 justify-end">
        <Button
          :label="$t('cmp.viewer.commentAddForm.next')"
          type="submit"
          outlined
          :disabled="!isCommentPageValid(0)"
        />
      </span>
    </form>
    <form
      v-for="page in Array.from({ length: page_count+1 }, (_, i) => i+1)"
      :key="`Page ${$t('cmp.viewer.commentAddForm.page')} ${page}`"
      class="flex grow flex-col gap-4 max-w-[30rem]"
      @submit.prevent="() => page == page_count ? onSave() : curr_page+=1"
    >
      <div
        v-if="curr_page == page"
        class="flex grow flex-col gap-4 max-w-[30rem]"
      >
        <template v-if="page < page_count + 1">
          <FormDynamicField
            v-for="field in commentFieldsSortedByPage(page)
              .filter(field => field.categories == null || field.categories.includes(props.entity.category_id))"
            :key="field.key"
            v-model:field-content="(editedComment!.data as EntityOrCommentData)[field.key]"
            :form-field="(field as FormField)"
            @is-valid="isValid => commentFieldValid[field.key]= isValid"
          />

          <span
            class="flex gap-1 justify-end"
          >
            <Button
              :label="$t('cmp.viewer.commentAddForm.previous')"
              outlined
              @click="curr_page -= 1"
            />
            <Button
              :label="page == page_count ? $t('cmp.viewer.commentAddForm.save') : $t('cmp.viewer.commentAddForm.next')"
              type="submit"
              :outlined="page != page_count"
              :loading="processingRequest"
              :disabled="processingRequest || !isCommentPageValid(page)"
            />
          </span>
        </template>
        <div
          v-else
          class="flex flex-col justify-center items-center "
        >
          <div class="text-center font-bold">
            {{ $t('cmp.viewer.commentAddForm.captchaCheck') }}
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
import type { EntityOrCommentData, Family, FormField, PublicEntity, PublicNewComment, ViewerSearchedCachedEntity } from '~/lib'
import { isValidRichText, isValidText } from '~/lib/validation'
import state from '~/lib/viewer-state'

const formVisible = ref(false)

const props = defineProps<{
  family: Family
  entity: PublicEntity | ViewerSearchedCachedEntity
}>()

const processingRequest = ref(false)
const toast = useToast()
const { t } = useI18n()

const editedComment = ref<PublicNewComment>()

const curr_page = ref(0)
const page_count = ref(0)

const commentFieldValid = ref(
  props.family.comment_form.fields
    .reduce((acc, field) => {
      acc[field.key] = !field.mandatory
      return acc
    }, {} as Record<string, boolean>),
)

function reset_refs(new_entity_id: string) {
  editedComment.value = {
    author: '',
    data: {},
    entity_id: new_entity_id,
    text: '',
    entity_category_id: props.entity.category_id,
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

watch(
  () => formVisible.value,
  (__, _) => {
    curr_page.value = Math.min(curr_page.value, page_count.value)
  },
)

function commentFieldsSortedByPage(page: number) {
  return props.family.comment_form.fields
    .filter(field => field.form_page === page)
    .sort((field_a, field_b) => field_a.form_weight - field_b.form_weight)
}

function isCommentPageValid(page: number) {
  if (page === 0) {
    return isValidText(editedComment.value!.author) && isValidRichText(editedComment.value!.text)
  }
  return commentFieldsSortedByPage(page).every(field => commentFieldValid.value[field.key])
}

function hCaptchaVerify(token: string) {
  realOnSave(token)
}

function hCaptchaExpired() {
  toast.add({
    severity: 'error',
    summary: t('cmp.viewer.commentAddForm.error'),
    detail: t('cmp.viewer.commentAddForm.captchaExpired'),
    life: 3000,
  })
}

function hCaptchaError() {
  toast.add({
    severity: 'error',
    summary: t('cmp.viewer.commentAddForm.error'),
    detail: t('cmp.viewer.commentAddForm.captchaValidationError'),
    life: 3000,
  })
}

async function onSave() {
  if (state.hasSafeModeEnabled) {
    curr_page.value += 1
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
      summary: t('cmp.viewer.commentAddForm.success'),
      detail: t('cmp.viewer.commentAddForm.commentSuccess'),
      life: 3000,
    })
    reset_refs(props.entity.id)
  }
  catch {
    toast.add({
      severity: 'error',
      summary: t('cmp.viewer.commentAddForm.error'),
      detail: t('cmp.viewer.commentAddForm.commentError'),
      life: 3000,
    })
  }
  processingRequest.value = false
}
</script>
