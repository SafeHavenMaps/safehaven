<template>
  <Tag
    :style="{
      backgroundColor: props.tag.fill_color,
      borderColor: props.tag.border_color,
      color: getTextColor(props.tag.fill_color),
    }"
  >
    {{ props.tag.title }}
  </Tag>
</template>

<script setup lang="ts">
const props = defineProps<{
  tag: {
    title: string
    border_color: string
    fill_color: string
  }
}>()

function getTextColor(hexColor: string): string {
  const hex = hexColor.replace('#', '')
  const r = parseInt(hex.substring(0, 2), 16)
  const g = parseInt(hex.substring(2, 4), 16)
  const b = parseInt(hex.substring(4, 6), 16)
  const luminance = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255
  return luminance > 0.5 ? 'black' : 'white'
}
</script>
