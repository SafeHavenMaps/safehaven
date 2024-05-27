<template>
  <div>
    <DataTable
      :value="state.accessTokens"
      striped-rows
      :table-style="{ 'min-width': '42rem' }"
      :rows="10"
      :rows-per-page-options="[10, 20, 50]"
      removable-sort
    >
      <Column
        field="token"
        header="Jeton"
      />
      <Column
        header="Actif"
        field="active"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.active ? 'Oui' : 'Non'"
            :severity="slotProps.data.active ? 'success' : 'warning'"
          />
        </template>
      </Column>

      <Column
        header="Actif"
        field="active"
        sortable
      >
        <template #body="slotProps">
          <EditDeleteButtons
            :id="slotProps.data.id"
            model-name="le jeton d'accès"
            :name="slotProps.data.id"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/access-tokens/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Jeton d\'accès',
  'accessToken',
  [
    {
      icon: 'add',
      label: 'Nouveau jeton d\'accès',
      severity: 'success',
      url: `/admin/access-tokens/new`,
    },
  ],
  [
    { label: 'Jeton d\'accès', url: '/admin/access-tokens' },
  ],
)

await state.fetchAccessTokens()

async function onDelete(access_token_id: string) {
  await state.deleteAccessToken(access_token_id)
}
</script>
