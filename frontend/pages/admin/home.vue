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
import type { SetBreadcrumbFunction } from '~/layouts/admin-ui.vue'

definePageMeta({
  layout: 'admin-ui',
  cardTitle: 'Accueil',
  cardIcon: 'home',
})

const setBreadcrumb = inject<SetBreadcrumbFunction>('setBreadcrumb')!
setBreadcrumb(
  { label: 'Accueil', url: '/admin/home' },
)
</script>
