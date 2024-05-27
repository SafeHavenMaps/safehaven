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
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Accueil',
  'home',
  [],
  [
    { label: 'Accueil', url: '/admin/home' },
  ],
)
</script>
