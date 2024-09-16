<template>
  <div class="flex flex-col gap-2">
    <label for="category_id">{{ $t('cmp.form.categorySelect.category') }} <RequiredIndicator /></label>
    <Select
      id="category_id"
      :model-value="props.modelValue"
      :options="categories"
      :label="$t('cmp.form.categorySelect.category')"
      :placeholder="$t('cmp.form.categorySelect.selectCategory')"
      option-value="id"
      @update:model-value="updateValue"
    >
      <template #value="slotProps">
        <CategoryTag
          v-if="slotProps.value"
          :category="categoryRecord[slotProps.value]"
        />
        <span v-else>{{ slotProps.placeholder }}</span>
      </template>
      <template #option="slotProps">
        <CategoryTag :category="slotProps.option" />
      </template>
    </Select>
    <small v-if="props.helperText">{{ props.helperText }}</small>
  </div>
</template>

<script setup lang="ts">
import type { Category, CategoryRecord } from '~/lib'

const props = defineProps<{
  modelValue: string | undefined | null
  categories: Category[]
  helperText?: string
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(value: undefined | string) {
  emit('update:modelValue', value)
}

const categoryRecord: CategoryRecord = props.categories.reduce((categories, category) => {
  categories[category.id] = category
  return categories
}, {} as CategoryRecord)
</script>
