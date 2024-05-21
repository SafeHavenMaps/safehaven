<template>
  <div>
    <Tree
      :value="nodes"
      class="w-full md:w-30rem"
    />
  </div>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'

// TODO : add proper @nodeSelect="onNodeSelect" to each node to enable functionality
await state.fetchFamilies()
const family_nodes = state.families.map(item => ({
  label: item.title,
  key: item.id,
  children: [
    {
      label: 'Entities',
    },
    {
      label: 'Comments',
    },
  ],
}))

const nodes = [
  {
    label: 'Accueil',
  },
  {
    label: 'General Config',
  },
  {
    label: 'Users',
  },
  {
    label: 'Access Tokens',
  },
  {
    label: 'Families & Forms',
  },
  ...family_nodes,
  {
    label: 'Tags',
  },
]
</script>
