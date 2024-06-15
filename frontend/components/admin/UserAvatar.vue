<!-- eslint-disable vue/no-v-html -->
<template>
  <Avatar
    shape="circle"
    :size="props.size"
  >
    <template #icon>
      <div
        class="flex"
        v-html="icon"
      />
    </template>
  </Avatar>
</template>

<script setup lang="ts">
import { toSvg } from 'jdenticon'

const props = defineProps<{
  username: string
  size: 'normal' | 'large' | 'xlarge' | undefined
}>()

const icon = toSvg(props.username, 40)
</script>
