<template>
  <svg
    :width="props.width"
    :height="props.height"
    version="1.1"
    viewBox="0 0 43.921 66.94"
    xml:space="preserve"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink">
  <path
    d="m21.905 1.2688c-11.397-1.86e-5 -20.637 9.5307-20.636 
    21.287 0.00476 3.5178 0.85467 6.9796 2.4736 10.076 5.9268 10.527 12.063 21.068 18.111 
    31.572 5.8042-10.829 13.224-21.769 18.766-32.581 
    1.4143-2.9374 1.9205-5.7872 1.9231-9.0669 6.2e-5 -11.757-9.2392-21.287-20.636-21.287z"
    :fill="props.fill"
    :stroke="props.stroke"
    stroke-width="2.5"/>
  </svg>
</template>

<script setup lang="ts">
const props = defineProps<{
  width: number
  height: number
  fill: string
  stroke: string
}>()
</script>
