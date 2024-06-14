<template>
  <div class="flex flex-column gap-2">
    <label for="tag_id">Tags</label>
    <div class="flex flex-wrap gap-1">
      <DisplayedTag
        v-for="tag_id in props.modelValue"
        :key="tag_id"
        :tag="tagRecord[tag_id]"
      />
    </div>

    <MultiSelect
      id="tag_id"
      :model-value="props.modelValue"
      :options="tags"
      label="Tag"
      placeholder="Sélectionner des tags"
      option-value="id"
      option-label="title"
      @update:model-value="updateValue"
    >
      <template #option="slotProps">
        <DisplayedTag :tag="slotProps.option" />
      </template>
    </MultiSelect>
    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
import type { Tag } from '~/lib'

const props = defineProps<{
  modelValue: string[] | undefined | null
  tags: Tag[]
  helperText?: string
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(value: undefined | string) {
  emit('update:modelValue', value)
}

type TagRecord = Record<string, Tag>

const tagRecord: TagRecord = props.tags.reduce((tags, tag) => {
  tags[tag.id] = tag
  return tags
}, {} as TagRecord)
</script>
