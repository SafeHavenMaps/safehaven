<template>
  <Tag
    :style="{
      backgroundColor: props.category.fill_color,
      borderColor: props.category.border_color,
      color: 'white',
    }"
  >
    <i
      style="height: 16px;"
      class="mr-1"
      v-html="props.category.icon"
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
