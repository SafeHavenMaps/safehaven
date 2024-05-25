<template>
  <Tag
    :style="{
      backgroundColor: props.category.fill_color,
      borderColor: props.category.border_color,
      color: 'white',
    }"
  >
    <img
      height="16"
      class="mr-1"
      :src="imageSrc()"
    >
    {{ props.category.title }}
  </Tag>
</template>

<script setup lang="ts">
import type { Category } from '~/lib'

const props = defineProps<{
  category: Category
}>()

function imageSrc(): string {
  return `/api/icons/categories/${props.category.icon_hash}`
}
</script>
