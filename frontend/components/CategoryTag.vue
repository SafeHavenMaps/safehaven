<template>
  <Tag
    :style="{
      backgroundColor: props.category.fill_color,
      borderColor: props.category.border_color,
      color: 'white',
    }"
  >
    <AppIcon
      class="mr-1"
      size="16px"
      dynamic
      :icon-name="props.category.icon_hash!"
    />
    {{ props.category.title }}
  </Tag>
</template>

<script setup lang="ts">
import type { Category } from '~/lib'

const props = defineProps<{
  category: Category
}>()
</script>
