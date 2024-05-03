<template>
  <svg
    :style="{ width: props.size ?? '24px', height: props.size ?? '24px' }"
    viewBox="0 0 24 24"
  >
    <path :d="iconDict[props.iconName]" fill="currentColor" />
  </svg>
</template>

<script setup lang="ts">
import * as mdi from '@mdi/js'

const props = defineProps<{
  iconName: keyof typeof iconDict
  size?: string
}>()

const iconDict: Record<string, string> = {
  user: mdi.mdiAccount,
  family: mdi.mdiFamilyTree,
  category: mdi.mdiLabel,
  tag: mdi.mdiTagOutline,
  entity: mdi.mdiTagOutline,
  userGroup: mdi.mdiAccountGroup,
  accessToken: mdi.mdiAccountGroup,
  mapSearch: mdi.mdiMapSearch,
  filter: mdi.mdiFilterCog,
  information: mdi.mdiInformationOutline,
  addEntity: mdi.mdiMapMarkerPlus,
  edit: mdi.mdiPencil,
  addComment: mdi.mdiCommentPlus,
  delete: mdi.mdiDelete,
  save: mdi.mdiUpload,
  lightDark: mdi.mdiThemeLightDark,
}
</script>

<style scoped>
svg path {
  fill: currentColor;
}
</style>
