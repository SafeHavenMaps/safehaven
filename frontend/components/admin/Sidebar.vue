<template>
  <div class="body">
    <Tree
      :value="nodes"
      class="body"
    >
      <template #nodeicon="slotProps">
        <AppIcon
          v-if="slotProps.node.children == null"
          :icon-name="slotProps.node.icon!"
          class="mr-2 -ml-2"
        />
        <img
          v-else
          style="height: 20rem; width: 20rem;"
          :src="imageSrc(slotProps.node.icon_hash)"
        >
      </template>
    </Tree>
  </div>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'

function imageSrc(hash: string): string {
  return `/api/icons/families/${hash}`
}

// TODO : add proper @nodeSelect="onNodeSelect" to each node to enable functionality
await state.fetchFamilies()
const family_nodes = state.families.map(item => ({
  label: item.title,
  key: item.id,
  icon_hash: item.icon_hash,
  children: [
    {
      label: 'Entities',
      icon: 'entity',
    },
    {
      label: 'Comments',
      icon: 'comment',
    },
  ],
}))

const nodes = [
  {
    label: 'Accueil',
    icon: 'stats',
  },
  {
    label: 'General Config',
    icon: 'config',
  },
  {
    label: 'Users',
    icon: 'user',
  },
  {
    label: 'Access Tokens',
    icon: 'accessToken',
  },
  {
    label: 'Families & Forms',
    icon: 'family',
  },
  ...family_nodes,
  {
    label: 'Tags',
    icon: 'tag',
  },
]
</script>

<style>
.p-treenode-content {
  cursor: pointer;
}
.body {
  background-color: rgb(247, 204, 240);
}
</style>
