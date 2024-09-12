<template>
  <form
    class="flex flex-col gap-4 max-w-[30rem] mx-6"
  >
    <AdminInputIconUpload
      :object-id="id"
      object-type="families"
    />
    <span class="flex gap-1 justify-end">
      <NuxtLink :to="`/admin/families`">
        <Button
          label="Retour à la liste"
          severity="secondary"
        />
      </NuxtLink>
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

if (!state.is_admin)
  navigateTo('/admin/home')

const id = useRoute().params.id as string

const fetchedFamily = await state.fetchFamily(id)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition de l'icône de la catégorie ${fetchedFamily.title}`,
  'family',
  [],
  [
    { label: 'Familles', url: '/admin/families' },
    { label: `Édition de l'icône de la famille ${fetchedFamily.title}`, url: `/admin/families/${id}/general` },
  ],
)
</script>
