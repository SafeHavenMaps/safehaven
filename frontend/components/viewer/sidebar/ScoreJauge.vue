<template>
  <div class="jauge mb-2">
    <div
      class="dot"
      :style="style"
    />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  score: number
}>()

const style = {
  left: `calc(${props.score * 10}% - 10px)`,
}
</script>

<style scoped>
.jauge {
  position: relative;
  height: 20px;
  width: 100%;
  background: rgb(242, 129, 116);
  background: linear-gradient(
    90deg,
    rgba(242, 129, 116, 1) 0%,
    rgba(247, 188, 143, 1) 33%,
    rgba(231, 232, 179, 1) 66%,
    rgba(180, 212, 200, 1) 100%
  );
  border: 1px solid rgba(0, 0, 0, 0.3);
  border-radius: 200px;
}

.dot {
  position: absolute;
  top: -1px;
  border-radius: 50%;
  height: 20px;
  width: 20px;
  background-color: white;
  border: 2px solid rgba(0, 0, 0, 0.3);
}
</style>
