<template>
  <Accordion :value="sortedComments[0].id">
    <AccordionPanel
      v-for="comment in sortedComments"
      :key="comment.id"
      :value="comment.id"
    >
      <AccordionHeader>
        <span class="flex items-center gap-2 w-full">
          {{ comment.author }} - {{ new Date(comment.created_at).toLocaleDateString() }}
          <Tag
            v-if="!public && !(comment as AdminComment).moderated"
            value="À modérer"
          />
          <AdminEditDeleteButtons
            v-if="!public"
            :id="comment.id"
            model-name="du commentaire"
            :loading="false"
            :name="`de ${comment.author}`"
            class="ml-auto mr-2"
            @edit="id => emit('edit', id)"
            @delete="(id, name, onDeleteDone) => emit('delete', id, name, onDeleteDone)"
          />
        </span>
      </AccordionHeader>

      <!-- eslint-disable vue/no-v-html -->
      <AccordionContent>
        <p
          style="white-space: pre-wrap; word-break: break-word; overflow-wrap: break-word;"
          class="rich-text-content"
          v-html="purify_strict(comment.text)"
        />

        <ViewerCommonFormFields
          :fields="props.commentFormFields
            .filter(field => field.categories == null || field.categories.includes(props.entityCategoryId))"
          :data="comment.data"
        />
      </AccordionContent>
    </AccordionPanel>
  </Accordion>
</template>

<script setup lang="ts">
import { purify_strict } from '~/lib/dompurify'
import type { PublicComment, AdminComment, FormField } from '~/lib'

const props = defineProps<{
  commentFormFields: FormField[]
  comments: PublicComment[]
  public: true
  entityCategoryId: string
} | {
  commentFormFields: FormField[]
  comments: AdminComment[]
  public: false
  entityCategoryId: string
}>()

const emit = defineEmits<{
  (e: 'edit', id: string): void
  (e: 'delete', id: string, name: string, onDeleteDone: () => void): void
}>()

const sortedComments = computed(() => props.comments.slice().sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()))
</script>
