<template>
  <Tag
    :style="{
      backgroundColor: 'hotpink',
      borderColor: 'white',
      color: 'white',
    }"
  >
    <!-- todo : add dynamic colors to tags -->
    {{ props.tag.title }}
  </Tag>
</template>

<script setup lang="ts">
import type { Tag } from '~/lib'

const props = defineProps<{
  tag: Tag
}>()
</script>
