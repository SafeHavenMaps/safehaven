<template>
  <img
    v-if="props.dynamicType"
    :style="{ width: props.size ?? '24px', height: props.size ?? '24px' }"
    :src="imageSrc(props.dynamicType, props.iconName)"
    :class="{ 'icon-rotate': rotating }"
  >
  <svg
    v-else
    :style="{ width: props.size ?? '24px', height: props.size ?? '24px' }"
    viewBox="0 0 24 24"
    :class="{ 'icon-rotate': rotating }"
  >
    <path
      :d="iconDict[props.iconName]"
      fill="currentColor"
    />
  </svg>
</template>

<script setup lang="ts">
import * as mdi from '@mdi/js'

const props = defineProps<{
  iconName: keyof typeof iconDict
  size?: string
  rotating?: boolean
  dynamicType?: DynamicIconTypes
}>()

export type DynamicIconTypes = 'families' | 'categories'

function imageSrc(dynamic_type: DynamicIconTypes, hash: string): string {
  return `/api/icons/${dynamic_type}/${hash}`
}

const iconDict: Record<string, string> = {
  home: mdi.mdiHome,
  config: mdi.mdiCog,
  user: mdi.mdiAccount,
  login: mdi.mdiAccountLockOpen,
  family: mdi.mdiFamilyTree,
  category: mdi.mdiLabel,
  tag: mdi.mdiTagOutline,
  entity: mdi.mdiMapMarkerAccount,
  pendingEntity: mdi.mdiMapMarkerAlert,
  comment: mdi.mdiCommentOutline,
  pendingComment: mdi.mdiCommentAlertOutline,
  userGroup: mdi.mdiAccountGroup,
  accessToken: mdi.mdiLock,
  mapSearch: mdi.mdiMapSearch,
  search: mdi.mdiMagnify,
  filter: mdi.mdiFilterCog,
  information: mdi.mdiInformationOutline,
  addEntity: mdi.mdiMapMarkerPlus,
  edit: mdi.mdiPencil,
  addComment: mdi.mdiCommentPlus,
  add: mdi.mdiPlus,
  delete: mdi.mdiDelete,
  save: mdi.mdiUpload,
  lightDark: mdi.mdiThemeLightDark,
  loading: mdi.mdiLoading,
  eye: mdi.mdiEye,
  warning: mdi.mdiAlert,
}
</script>

<style scoped>
svg path {
  fill: currentColor;
}

.icon-rotate {
  animation: spin infinite 1s linear;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
