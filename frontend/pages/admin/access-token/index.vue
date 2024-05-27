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
      <template #header>
        <TableHeaders
          model-display-name="Jeton d'accès"
          model-name="access-token"
          model-icon="accessToken"
        />
      </template>
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
            @edit="id => navigateTo(`/admin/access-token/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import TableHeaders from '~/components/admin/TableHeaders.vue'
// import type { AccessToken } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})
await state.fetchAccessTokens()

async function onDelete(access_token_id: string) {
  await state.deleteAccessToken(access_token_id)
}
</script>
