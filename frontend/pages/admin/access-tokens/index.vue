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
          v-model="(state.tablesFilters[tableKey]['global'] as DataTableFilterMetaData).value"
          placeholder="Recherche"
        />
      </IconField>
      <MultiSelect
        v-model="state.tablesSelectedColumns[tableKey]"
        :options="optionalColumns"
        option-label="label"
        option-value="key"
        display="chip"
        placeholder="Sélectionner des colonnes"
        class="w-full md:w-80"
      />
    </span>
    <DataTable
      v-model:filters="state.tablesFilters[tableKey]"
      state-storage="session"
      :state-key="tableKey"
      data-key="id"
      paginator
      paginator-template="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown CurrentPageReport"
      current-page-report-template="&nbsp&nbsp&nbsp({totalPages} page·s, {totalRecords} jeton·s)"
      :value="accessTokens"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['title', 'token']"
      filter-match-mode="contains"
    >
      <Column
        field="title"
        header="Titre"
        sortable
      />
      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('token')"
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
            :severity="slotProps.data.active ? 'success' : 'danger'"
          />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('last_week_visits')"
        field="last_week_visits"
        header="Visites (7 derniers jours)"
        class="max-w-32 "
        sortable
      />

      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('permissions_families_policy')"
        header="Familles"
        :field="(data: any) => all_included(data.permissions.families_policy).toString()"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="all_included(slotProps.data.permissions.families_policy) ? 'Toutes' : 'Partiel'"
            :severity="all_included(slotProps.data.permissions.families_policy) ? 'success' : 'warning'"
          />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('permissions_categories_policy')"
        header="Catégories"
        :field="(data: any) => all_included(data.permissions.categories_policy).toString()"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="all_included(slotProps.data.permissions.categories_policy) ? 'Toutes' : 'Partiel'"
            :severity="all_included(slotProps.data.permissions.categories_policy) ? 'success' : 'warning'"
          />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('permissions_tags_policy')"
        header="Tags"
        :field="data => all_included(data.permissions.tags_policy).toString()"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="all_included(slotProps.data.permissions.tags_policy) ? 'Toutes' : 'Partiel'"
            :severity="all_included(slotProps.data.permissions.tags_policy) ? 'success' : 'warning'"
          />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('permissions_can_access_comments')"
        header="Commentaires"
        field="permissions.can_access_comments"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.permissions.can_access_comments ? 'Oui' : 'Non'"
            :severity="slotProps.data.permissions.can_access_comments ? 'success' : 'danger'"
          />
        </template>
      </Column>

      <Column>
        <template #body="slotProps">
          <Button
            outlined
            rounded
            class="mx-2"
            severity="success"
            @click="viewDetails(slotProps.data.id)"
          >
            <template #icon>
              <AppIcon icon-name="stats" />
            </template>
          </Button>

          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            model-name="du jeton d'accès"
            :name="slotProps.data.title"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/access-tokens/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AccessToken, PermissionPolicy } from '~/lib'
import state from '~/lib/admin-state'

// 'Jeton', 'Visites', 'Familles', 'Catégories', 'Tags', 'Commentaires'
const optionalColumnsKeys = ['token', 'last_week_visits', 'permissions_family_policy',
  'permissions_categories_policy', 'permissions_tags_policy', 'permissions_can_access_comments']
const optionalColumns = state.toSelectableColumns(optionalColumnsKeys)

const tableKey = `dt-state-access-tokens`
const isSmallScreen = useMediaQuery('(max-width: 768px)')
const selectedColumKeys = isSmallScreen.value ? [] : ['last_week_visits']
state.registerTable(tableKey, selectedColumKeys)

function all_included(permissionPolicy: PermissionPolicy) {
  return permissionPolicy.allow_all && !permissionPolicy.force_exclude.length
}

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Jetons d\'accès',
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
    { label: 'Jetons d\'accès', url: '/admin/access-tokens' },
  ],
)

// Initialize the ref with an empty array, then fetch to update access tokens asynchronously
const accessTokens: Ref<AccessToken[]> = ref([])
async function refreshTable() {
  accessTokens.value = await state.client.listAccessTokens()
}
refreshTable()

const toast = useToast()

async function onDelete(access_token_id: string, access_token_name: string, onDeleteDone: () => void) {
  try {
    await state.client.deleteAccessToken(access_token_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Jeton ${access_token_name} supprimée avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression du jeton ${access_token_name}`, life: 3000 })
  }
  onDeleteDone()
}

function viewDetails(access_token_id: string) {
  navigateTo(`/admin/access-tokens/details/${access_token_id}`)
}
</script>
