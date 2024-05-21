<template>
  <div class="map-marker">
    <svg
      :class="{ highlighted: props.highlighted }"
      :width="props.width"
      :height="props.height"
      version="1.1"
      viewBox="0 0 43.921 66.94"
      xml:space="preserve"
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      style="cursor: pointer"
      @click="handleClick"
    >
      <path
        d="m21.905 1.2688c-11.397-1.86e-5 -20.637 9.5307-20.636
      21.287 0.00476 3.5178 0.85467 6.9796 2.4736 10.076 5.9268 10.527 12.063 21.068 18.111
      31.572 5.8042-10.829 13.224-21.769 18.766-32.581
      1.4143-2.9374 1.9205-5.7872 1.9231-9.0669 6.2e-5 -11.757-9.2392-21.287-20.636-21.287z"
        :fill="props.fill"
        :stroke="props.stroke"
        stroke-width="2.5"
      />
      <foreignObject
        v-if="props.insideIcon && props.insideIcon.length > 0"
        x="9"
        y="9"
        width="26"
        height="26"
        v-html="props.insideIcon"
      /></svg>
  </div>
</template>

<script setup lang="ts" generic="T">
const props = defineProps<{
  width: number
  height: number
  fill: string
  stroke: string
  highlighted: boolean
  insideIcon: string | null | undefined
  callbackItem: T
}>()

const emit = defineEmits<{
  click: [item: T]
}>()

const handleClick = () => {
  emit('click', props.callbackItem)
}
</script>

<style scoped>
.map-marker {
  position: relative;
  width: 100%;
  height: 100%;
}

.map-marker svg {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  transition: transform 0.2s, filter 0.2s;
}

.map-marker svg.highlighted {
  transform: translateX(-50%) scale(1.2);
  animation: bounce 0.75s infinite alternate;
}

@keyframes bounce {
  0% {
    transform: translateX(-50%) translateY(0) scale(1.2);
  }
  100% {
    transform: translateX(-50%) translateY(-10px) scale(1.2);
  }
}
</style>
