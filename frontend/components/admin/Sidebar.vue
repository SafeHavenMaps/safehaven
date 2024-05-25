<template>
  <div class="poudre-bg">
    <Tree
      v-model:expandedKeys="expandedKeys"
      :value="nodes"
      class="poudre-bg"
      selection-mode="single"
      @node-select="onNodeSelect"
    >
      <template #nodeicon="slotProps">
        <AppIcon
          :icon-name="slotProps.node.icon!"
          :dynamic-type="slotProps.node.icon_dynamic_type"
          class="mr-2 -ml-2"
        />
      </template>
    </Tree>
  </div>
</template>

<script setup lang="ts">
import type { TreeNode } from 'primevue/treenode'
import state from '~/lib/admin-state'

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
  icon: item.icon_hash!,
  icon_dynamic_type: 'families',
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
</style>
