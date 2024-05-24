<template>
  <div class="body">
    <Tree
      v-model:expandedKeys="expandedKeys"
      :value="nodes"
      class="body"
      selection-mode="single"
      @node-select="onNodeSelect"
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
import type { TreeNode } from 'primevue/treenode'
import state from '~/lib/admin-state'

function imageSrc(hash: string): string {
  return `/api/icons/families/${hash}`
}

await state.fetchFamilies()

const expandedKeys = ref<Record<string, boolean>>({})

function onNodeSelect(node: TreeNode) {
  if (node.children !== undefined) {
    expandedKeys.value[node.key!] = !expandedKeys.value[node.key!]
  }
  else if (node.data !== undefined) {
    navigateTo('/admin/' + node.data)
  }
}

const family_nodes = state.families.map(item => ({
  label: item.title,
  key: item.id,
  icon_hash: item.icon_hash,
  children: [
    {
      label: 'Categories',
      icon: 'category',
      data: item.id + '/categories',
    },
    {
      label: 'Entities',
      icon: 'entity',
      data: item.id + '/entities',
    },
    {
      label: 'Comments',
      icon: 'comment',
      data: item.id + '/comments',
    },
    {
      label: 'Pending Entities',
      icon: 'pendingEntity',
      data: item.id + '/pending-entities',
    },
    {
      label: 'Pending Comments',
      icon: 'pendingComment',
      data: item.id + '/pending-categories',
    },
  ],
}))

const nodes = [
  {
    label: 'Accueil',
    icon: 'home',
    data: '',
  },
  {
    label: 'General Config',
    icon: 'config',
    data: 'config',
  },
  {
    label: 'Users',
    icon: 'user',
    data: 'user',
  },
  {
    label: 'Access Tokens',
    icon: 'accessToken',
    data: 'access-token',
  },
  {
    label: 'Families & Forms',
    icon: 'family',
    data: 'family',
  },
  ...family_nodes,
  {
    label: 'Tags',
    icon: 'tag',
    data: 'tag',
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
