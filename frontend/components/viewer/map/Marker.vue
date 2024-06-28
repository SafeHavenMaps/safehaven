<template>
  <div
    class="map-marker-container"
    :style="
      {
        width: `${props.highlighted ? Math.round(1.25*props.width) : props.width}px`,
        height: `${props.highlighted ? Math.round(1.25*props.height) : props.height}px`,
      }
    "
  >
    <img
      :class="{ highlighted: !!props.highlighted }"
      :width="props.highlighted ? Math.round(1.25*props.width) : props.width"
      :height="props.highlighted ? Math.round(1.25*props.height) : props.height"
      :src="props.highlighted ? urls.upscaled_url : urls.url"
      style="cursor: pointer; user-select: none;"
      draggable="false"
      class="map-marker"
      @click="handleClick"
    >
    <img
      hidden
      :src="urls.upscaled_url"
    >
  </div>
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

const urls = computed(() => {
  const url = `/api/icons/pin?h=${props.height}&w=${props.width}`
  const upscaled_url = `/api/icons/pin?h=${Math.round(1.25 * props.height)}&w=${Math.round(1.25 * props.width)}`

  let url_end = ''
  if (props.iconHash) {
    url_end += `&ih=${props.iconHash}`
  }
  if (props.borderColor) {
    url_end += `&bc=${props.borderColor.replace('#', '')}`
  }
  if (props.fillColor) {
    url_end += `&fc=${props.fillColor.replace('#', '')}`
  }

  return { url: url + url_end, upscaled_url: upscaled_url + url_end }
})

const emit = defineEmits<{
  click: [item: T]
}>()

const handleClick = () => {
  emit('click', props.callbackItem)
}
</script>

<style scoped>
.map-marker-container {
  position: relative;
}

.map-marker {
  display: block;
  position: absolute;
  top: -100%;
  left: -50%;
}

.map-marker.highlighted {
  animation: bounce 0.75s infinite alternate;
}

@keyframes bounce {
  0% {
    transform: translateY(0%);
  }
  100% {
    transform: translateY(-40%);
  }
}
</style>
