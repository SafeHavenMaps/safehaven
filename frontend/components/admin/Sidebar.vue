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
        <span
          v-else
          style="height: 20rem; width: 20rem;"
          v-html="slotProps.node.icon"
        />
      </template>
    </Tree>
  </div>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'
import viewer_state from '~/lib/viewer-state'

// TODO : add proper @nodeSelect="onNodeSelect" to each node to enable functionality
await state.fetchFamilies()
const family_nodes = state.families.map(item => ({
  label: item.title,
  key: item.id,
  icon: viewer_state.cleanupSvg(item.icon!)!,
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
