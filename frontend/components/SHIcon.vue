<template>
    <MdiIcon :icon="iconDict[props.iconName]"  style="width:1.5em; height:1.5em"/>
</template>

<script setup lang="ts">
import * as Icons from '@mdi/js'
type MdiIconString = keyof typeof Icons

const props = defineProps<{
    iconName: string
}>();

const iconDict: Record<string, MdiIconString>={
    user: "mdiAccount",
    family:"mdiFamilyTree",
    category:"mdiLabel",
    tag:"mdiTagOutline",
    entity: "mdiMapMarkerAccount",
    userGroup: "mdiAccountGroup",
    accessToken: "mdiLock",
    mapSearch: "mdiMapSearch",
    filter: "mdiFilterCog",
    information: "mdiInformationOutline",
    addEntity: "mdiMapMarkerPlus",
    edit: "mdiPencil",
    addComment: "mdiCommentPlus",
    delete: "mdiDelete",
    save: "mdiUpload",
    lightDark: "mdiThemeLightDark"
}
</script>