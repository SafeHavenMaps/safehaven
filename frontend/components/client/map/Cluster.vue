<template>
  <div class="map-cluster">
    <div
      class="cluster"
      :style="{ backgroundColor: generateColor(props.seed) }"
      @click="handleClick"
      style="cursor: pointer;"
    >
      {{ props.count }}
    </div>
  </div>
</template>

<script setup lang="ts" generic="T">
const props = defineProps<{
  count: number;
  seed: number;
  callbackItem: T;
}>();

function generateColor(seed: number): string {
  const hue = seed % 360;
  const saturation = 50;
  const lightness = 30;

  return `hsl(${hue}, ${saturation}%, ${lightness}%, 0.5)`;
}

const emit = defineEmits<{
  click: [item: T];
}>();

const handleClick = () => {
  emit("click", props.callbackItem);
};
</script>

<style scoped>
.map-cluster {
  position: relative;
  width: 100%;
  height: 100%;
}

.map-cluster .cluster {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  height: 40px;
  width: 40px;
  color: white;
  text-shadow: 0 0 2px black;
  font-weight: bolder;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 3px solid #fff;
  border-radius: 50%;
}
</style>
