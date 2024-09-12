<template>
  <div>
    <span class="flex gap-4 flex-wrap">
      <IconField
        icon-position="left"
      >
        <InputIcon><AppIcon
          icon-name="search"
          class="-mt-1"
        /></InputIcon>
        <InputText
          v-model="(state.tablesFilters[table_key]['global'] as DataTableFilterMetaData).value"
          placeholder="Recherche"
        />
      </IconField>
      <MultiSelect
        v-model="state.tablesSelectedColumns[table_key]"
        :options="optionalColumns"

        display="chip"
        placeholder="Sélectionner des colonnes"
        class="w-full md:w-80"
      />
    </span>
    <DataTable
      v-model:filters="state.tablesFilters[table_key]"
      paginator
      paginator-template="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown CurrentPageReport"
      current-page-report-template="&nbsp&nbsp&nbsp({totalPages} page·s, {totalRecords} utilisateur·ice·s)"
      :value="users"
      striped-rows
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      :rows="10"
      :rows-per-page-options="[10, 20, 50]"
      removable-sort
      :global-filter-fields="['name', 'id']"
    >
      <Column
        header="Nom"
        sortable
      >
        <template #body="slotProps">
          <div class="flex items-center justify-content-begin gap-2">
            <AdminUserAvatar
              :username="slotProps.data.name"
              size="normal"
            />
            {{ slotProps.data.name }}
          </div>
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Droits')"
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
        v-if="state.tablesSelectedColumns[table_key].includes('Connection')"
        field="last_login"
        header="Dernière connection"
      >
        <template #body="slotProps">
          {{ new Date(slotProps.data.last_login).toLocaleString() }}
        </template>
      </Column>

      <Column>
        <template #body="slotProps">
          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            model-name="de l'utilisateur⋅ice"
            :name="slotProps.data.name"
            :prevent-delete="state.username == slotProps.data.name"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/users/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { User } from '~/lib'
import state from '~/lib/admin-state'

const isSmallScreen = useMediaQuery('(max-width: 768px)')
const optionalColumns = ref(['Droits', 'Connection'])
const table_key = `dt-users`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = isSmallScreen.value ? [] : ['Droits', 'Connection']
}
if (!(table_key in state.tablesFilters)) {
  state.tablesFilters[table_key] = {
    global: { value: null, matchMode: 'contains' },
  }
}

definePageMeta({
  layout: 'admin-ui',
})

if (!state.is_admin)
  navigateTo('/admin/home')

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
const users: Ref<User[]> = ref([])
async function refreshTable() {
  users.value = await state.client.listUsers()
}
refreshTable()

const toast = useToast()

async function onDelete(user_id: string, user_name: string, onDeleteDone: () => void) {
  try {
    await state.client.deleteUser(user_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Utilisateur ${user_name} supprimé avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de l'utilisateur ${user_name}`, life: 3000 })
  }
  onDeleteDone()
}
</script>
