<template>
  <div>
    <span class="flex gap-3">
      <IconField
        icon-position="left"
      >
        <InputIcon><AppIcon
          icon-name="search"
          class="-mt-1"
        /></InputIcon>
        <InputText
          v-model="filters['global'].value"
          placeholder="Recherche"
        />
      </IconField>
    </span>
    <DataTable
      v-model:filters="filters"
      paginator
      :value="users"
      striped-rows
      :rows="10"
      :rows-per-page-options="[10, 20, 50]"
      removable-sort
      :global-filter-fields="['name', 'id']"
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
import { FilterMatchMode } from 'primevue/api'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { User } from '~/lib'
import state from '~/lib/admin-state'

const filters = ref({ global: { value: null, matchMode: FilterMatchMode.CONTAINS } })

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

// Initialize the ref with an empty array, then fetch to update access tokens asynchronously
const users: Ref<User[]> = ref([]);
(async () => {
  users.value = await state.client.listUsers()
})()

async function onDelete(user_id: string) {
  await state.client.deleteUser(user_id)
}
</script>