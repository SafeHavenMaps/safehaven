<template>
  <div>
    <div>
      <div>
        mainpanel
      </div>
      <div>
        filterbox
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'

definePageMeta({
  layout: 'admin-ui',
  cardTitle: 'Accueil',
  cardIcon: 'home',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  [],
  [
    { label: 'Accueil', url: '/admin/home' },
  ],
)
</script>
