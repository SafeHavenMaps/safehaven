<template>
  <div class="flex flex-col gap-2">
    <label for="tag_id">{{ $t('cmp.form.tagSelect.tags') }}</label>
    <div class="flex flex-wrap gap-1">
      <DisplayedTag
        v-for="tag_id in props.modelValue"
        :key="tag_id"
        :tag="tagRecord[tag_id]"
      />
    </div>

    <MultiSelect
      id="tag_id"
      filter
      :empty-filter-message="$t('cmp.form.tagSelect.emptyFilterMessage')"
      :model-value="props.modelValue"
      :options="tags"
      :placeholder="$t('cmp.form.tagSelect.placeholder')"
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
import type { Tag, TagRecord } from '~/lib'

const props = defineProps<{
  modelValue: string[] | undefined | null
  tags: Tag[]
  helperText?: string
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(value: undefined | string) {
  emit('update:modelValue', value)
}

const tagRecord: TagRecord = props.tags.reduce((tags, tag) => {
  tags[tag.id] = tag
  return tags
}, {} as TagRecord)
</script>
