<template>
  <Accordion :active-index="0">
    <AccordionTab
      v-for="comment in sortedComments"
      :key="comment.id"
    >
      <template #header>
        <span class="flex align-items-center gap-2 w-full">

          {{ comment.author }} - {{ new Date(comment.created_at).toLocaleDateString() }}
          <Tag
            v-if="!public"
            value="À modérer"
          />
          <AdminEditDeleteButtons
            v-if="!public"
            :id="`edit_delete_${comment.id}`"
            model-name="le commentaire"
            :loading="false"
            :name="`edit_delete_${comment.id}`"
            class="ml-auto mr-2"
          />
        </span>
      </template>
      <!-- eslint-disable vue/no-v-html -->
      <p
        style="white-space: pre-wrap; word-break: break-word; overflow-wrap: break-word;"
        v-html="purify_strict(comment.text)"
      />

      <ViewerCommonFormFields
        :fields="props.commentFormFields"
        :data="comment.data"
      />
    </AccordionTab>
  </Accordion>
</template>

<script setup lang="ts">
import { purify_strict } from '~/lib/dompurify'
import type { PublicComment, AdminComment, FormField } from '~/lib'

const props = defineProps<{
  commentFormFields: FormField[]
  comments: PublicComment[]
  public: true
} | {
  commentFormFields: FormField[]
  comments: AdminComment[]
  public: false
}>()

const sortedComments = computed(() => props.comments.slice().sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()))
</script>
