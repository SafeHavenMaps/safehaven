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
      <MultiSelect
        v-model="selectedColumns"
        :options="optionalColumns"

        display="chip"
        placeholder="Sélectionner des colonnes"
        class="w-full md:w-20rem"
      />
    </span>
    <DataTable
      v-model:filters="filters"
      paginator
      :value="accessTokens"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['title', 'token']"
      class=" "
    >
      <Column
        field="title"
        header="Titre"
        sortable
      />
      <Column
        v-if="selectedColumns.includes('Jeton')"
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
        v-if="selectedColumns.includes('Visites')"
        field="last_week_visits"
        header="Visites (7 derniers jours)"
        class="max-w-8rem "
        sortable
      />

      <Column
        v-if="selectedColumns.includes('Familles')"
        header="Familles"
        :field="data => all_included(data.permissions.families_policy).toString()"
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
        v-if="selectedColumns.includes('Catégories')"
        header="Catégories"
        :field="data => all_included(data.permissions.categories_policy).toString()"
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
        v-if="selectedColumns.includes('Tags')"
        header="Tags"
        field="data => all_included(data.permissions.tags_policy).toString()"
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
        v-if="selectedColumns.includes('Commentaires')"
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
          <EditDeleteButtons
            :id="slotProps.data.id"
            model-name="le jeton d'accès"
            :name="slotProps.data.token"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/access-tokens/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { FilterMatchMode } from 'primevue/api'
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AccessToken, PermissionPolicy } from '~/lib'
import state from '~/lib/admin-state'

const optionalColumns = ref(['Jeton', 'Visites', 'Familles', 'Catégories', 'Tags', 'Commentaires'])
const selectedColumns = ref(['Visites'])

const filters = ref({
  global: { value: null, matchMode: FilterMatchMode.CONTAINS },
})

function all_included(permissionPolicy: PermissionPolicy) {
  return permissionPolicy.allow_all && !permissionPolicy.force_exclude.length
}

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
    { label: 'Jetons d\'accès', url: '/admin/access-tokens' },
  ],
)

// Initialize the ref with an empty array, then fetch to update access tokens asynchronously
const accessTokens: Ref<AccessToken[]> = ref([]);
(async () => {
  accessTokens.value = await state.client.listAccessTokens()
})()

async function onDelete(access_token_id: string) {
  await state.client.deleteAccessToken(access_token_id)
}
</script>
