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
        header="Titre"
      />
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
            :severity="slotProps.data.active ? 'success' : 'danger'"
          />
        </template>
      </Column>

      <Column
        header="Familles"
        field="permissions"
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
        header="Catégories"
        field="permissions"
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
        header="Tags"
        field="permissions"
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
        header="Commentaires"
        field="permissions"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.permissions.can_access_comments ? 'Oui' : 'Non'"
            :severity="slotProps.data.permissions.can_access_comments ? 'success' : 'danger'"
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
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { PermissionPolicy } from '~/lib'
import state from '~/lib/admin-state'

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
    { label: 'Jeton d\'accès', url: '/admin/access-tokens' },
  ],
)

await state.fetchAccessTokens()

async function onDelete(access_token_id: string) {
  await state.deleteAccessToken(access_token_id)
}
</script>
