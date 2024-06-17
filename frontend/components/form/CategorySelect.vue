<template>
  <div class="flex flex-column gap-2">
    <label for="category_id">Catégorie</label>
    <Dropdown
      id="category_id"
      :model-value="props.modelValue"
      :options="categories"
      label="Catégorie"
      placeholder="Sélectionner une catégorie"
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
    </Dropdown>
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
