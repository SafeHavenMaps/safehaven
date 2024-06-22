<template>
  <img
    :class="{ highlighted: !!props.highlighted }"
    :width="props.width"
    :height="props.height"
    :src="url"
    style="cursor: pointer"
    class="map-marker"
    @click="handleClick"
  >
</template>

<script setup lang="ts" generic="T">
const props = defineProps<{
  width: number
  height: number
  iconHash?: string | null | undefined
  borderColor?: string | null | undefined
  fillColor?: string | null | undefined
  highlighted?: boolean
  callbackItem: T
}>()

const url = computed(() => {
  let url = `/api/icons/pin?h=${props.height}&w=${props.width}`

  if (props.iconHash) {
    url += `&ih=${props.iconHash}`
  }
  if (props.borderColor) {
    url += `&bc=${props.borderColor.replace('#', '')}`
  }
  if (props.fillColor) {
    url += `&fc=${props.fillColor.replace('#', '')}`
  }

  return url
})

const emit = defineEmits<{
  click: [item: T]
}>()

const handleClick = () => {
  emit('click', props.callbackItem)
}
</script>

<style scoped>
.map-marker {
  transform: translateX(-50%) translateY(-100%);
}

.map-marker.highlighted {
  transform: translateX(-50%) translateY(-100%) scale(1.25);
  animation: bounce 0.75s infinite alternate;
}

@keyframes bounce {
  0% {
    transform: translateY(-100%) scale(1.25) translateX(-50%);
  }
  100% {
    transform: translateY(-150%) scale(1.25) translateX(-50%);
  }
}
</style>
