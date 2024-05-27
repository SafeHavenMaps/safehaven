<template>
  <div>
    <DataTable
      :value="state.users"
      striped-rows
      :table-style="{ 'min-width': '42rem' }"
      :rows="10"
      :rows-per-page-options="[10, 20, 50]"
      removable-sort
    >
      <Column
        field="name"
        header="Nom"
        sortable
      />
      <Column
        header="Droits"
        field="is_admin"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.is_admin ? 'Administrateur⋅ice' : 'Modérateur⋅ice'"
            :severity="slotProps.data.is_admin ? 'success' : 'warning'"
          />
        </template>
      </Column>

      <Column
        field="last_login"
        header="Dernière connexion"
      >
        <template #body="slotProps">
          {{ new Date(slotProps.data.last_login).toLocaleString() }}
        </template>
      </Column>

      <Column>
        <template #body="slotProps">
          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            model-name="l'utilisateur⋅ice"
            :name="slotProps.data.name"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/users/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Utilisateur⋅ices',
  'user',
  [
    {
      icon: 'add',
      label: 'Ajouter un⋅e utilisateur⋅ice',
      severity: 'success',
      url: `/admin/users/new`,
    },
  ],
  [
    { label: 'Utilisateur⋅ices', url: '/admin/users' },
  ],
)

await state.fetchUsers()

async function onDelete(user_id: string) {
  await state.deleteUser(user_id)
}
</script>
