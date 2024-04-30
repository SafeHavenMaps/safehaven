<template>
    <MdiIcon :icon="icon_dict[icon_name]"  style="width:1.5em; height:1.5em"/>
</template>

<script setup lang="ts">
const props = defineProps<{
    icon_name: string
}>();

const icon_dict={
    "user": "mdiAccount",
    "family":"mdiFamilyTree",
    "category":"mdiLabel",
    "tag":"mdiTagOutline",
    "entity": "mdiMapMarkerAccount",
    "userGroup": "mdiAccountGroup",
    "accessToken": "mdiLock",
    "mapSearch": "mdiMapSearch",
    "filter": "mdiFilterCog",
    "information": "mdiInformationOutline",
    "addEntity": "mdiMapMarkerPlus",
    "edit": "mdiPencil",
    "addComment": "mdiCommentPlus",
    "delete": "mdiDelete",
    "save": "mdiSave",
    "lightDark": "mdiThemeLightDark"
}
</script>