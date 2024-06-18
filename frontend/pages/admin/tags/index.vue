<template>
  <div>
    <span class="flex gap-4">
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
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      paginator
      :value="tags"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['title', 'filter_description']"
      class=" "
    >
      <Column
        header="Titre"
        sortable
      >
        <template #body="slotProps">
          <DisplayedTag :tag="slotProps.data" />
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Filtrage')"
        field="is_filter"
        header="Filtrage"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.is_filter ? 'Filtrant' : 'Non-filtrant'"
            :severity="slotProps.data.is_filter ? 'success' : 'warning'"
          />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Valeur de filtre par défaut')"
        header="Valeur de filtre par défaut"
        field="default_filter_status"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.is_filter ? (slotProps.data.default_filter_status ? 'Inclus' : 'Exclus') : 'Non-filtrant'"
            :severity="slotProps.data.is_filter ? (slotProps.data.default_filter_status ? 'success' : 'danger') : 'warning'"
          />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Description de filtre')"
        header="Description de filtre"
        field="filter_description"
        sortable
      />

      <Column>
        <template #body="slotProps">
          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            model-name="du tag"
            :name="slotProps.data.title"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/tags/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { Tag } from '~/lib'
import state from '~/lib/admin-state'

const optionalColumns = ref(['Filtrage', 'Valeur de filtre par défaut', 'Description de filtre'])
const table_key = `dt-state-tags`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = ['Filtrage', 'Valeur de filtre par défaut']
}
if (!(table_key in state.tablesFilters)) {
  state.tablesFilters[table_key] = {
    global: { value: null, matchMode: 'contains' },
  }
}

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Tags',
  'tag',
  [
    {
      icon: 'add',
      label: 'Nouveau tag',
      severity: 'success',
      url: `/admin/tags/new`,
    },
  ],
  [
    { label: 'Tags', url: '/admin/tags' },
  ],
)

// Initialize the ref with an empty array, then fetch to update access tokens asynchronously
const tags: Ref<Tag[]> = ref([])
async function refreshTable() {
  await state.fetchTags()
  tags.value = state.tags
}
refreshTable()

const toast = useToast()

async function onDelete(tag_id: string, tag_name: string, onDeleteDone: () => void) {
  try {
    await state.client.deleteTag(tag_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Tag ${tag_name} supprimé avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression du tag ${tag_name}`, life: 3000 })
  }
  onDeleteDone()
}
</script>
