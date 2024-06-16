<template>
  <div>
    <span class="flex gap-3">
      <IconField icon-position="left">
        <InputIcon>
          <AppIcon
            icon-name="search"
            class="-mt-1"
          />
        </InputIcon>
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
        class="w-full md:w-20rem"
      />
    </span>
    <DataTable
      v-model:filters="state.tablesFilters[table_key]"
      paginator
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      :value="entities"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['display_name']"
      class=" "
    >
      <Column
        field="display_name"
        header="Nom d'affichage"
        class="max-w-25rem"
        sortable
      />

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Catégorie')"
        field="category_id"
        header="Catégorie"
        sortable
      >
        <template #body="slotProps">
          <CategoryTag :category="categoryRecord[slotProps.data.category_id]" />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Créé le')"
        field="created_at"
        header="Créé le"
        sortable
      >
        <template #body="slotProps">
          {{ new Date(slotProps.data.created_at).toLocaleDateString() }}
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Mis à jour le')"
        field="updated_at"
        header="Mis à jour le"
        sortable
      >
        <template #body="slotProps">
          {{ new Date(slotProps.data.updated_at).toLocaleDateString() }}
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Visibilité')"
        field="hidden"
        header="Visibilité"
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.hidden ? 'Caché' : 'Visible'"
            :severity="slotProps.data.hidden ? 'error' : 'success'"
          />
        </template>
      </Column>
      <Column>
        <template #body="slotProps">
          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            model-name="de l'entité"
            :name="slotProps.data.display_name"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/${familyId}/entities/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { FilterMatchMode } from 'primevue/api'
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminListedEntity, Category } from '~/lib'
import state from '~/lib/admin-state'

const familyId = useRoute().params.familyId as string
if (state.families == null)
  await state.fetchFamilies()
if (state.categories == null)
  await state.fetchCategories()

const familyTitle = state.families.filter(family => family.id == familyId)[0].title

type CategoryRecord = Record<string, Category>

const categoryRecord: CategoryRecord = state.categories.reduce((categories, category) => {
  categories[category.id] = category
  return categories
}, {} as CategoryRecord)

// Initialize the ref with an empty array, then fetch to update entities asynchronously
const entities: Ref<AdminListedEntity[]> = ref([])
async function refreshTable() {
  entities.value = await state.client.listPendingEntities()
  entities.value = entities.value.filter(entity => categoryRecord[entity.category_id].family_id == familyId)
  state.getEntitiesCommentsCounts()
}
refreshTable()

const optionalColumns = ref(['Créé le', 'Mis à jour le', 'Visibilité', 'Catégorie'])
const table_key = `dt-state-entities-${familyId}`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = ['Catégorie', 'Créé le']
}
if (!(table_key in state.tablesFilters)) {
  state.tablesFilters[table_key] = {
    global: { value: null, matchMode: FilterMatchMode.CONTAINS },
  }
}

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Entités en attente de modération',
  'entity',
  [
    {
      icon: 'add',
      label: 'Nouvelle entité',
      severity: 'success',
      url: `/admin/${familyId}/entities/new`,
    },
  ],
  [
    { label: `${familyTitle}`, url: '/admin/families' },
    { label: 'Entités en attente', url: `/admin/${familyId}/entities/pending` },
  ],
)

const toast = useToast()

async function onDelete(entity_id: string, entity_name: string, onDeleteDone: () => void) {
  try {
    await state.client.deleteEntity(entity_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Entité ${entity_name} supprimée avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de l'entité ${entity_name}`, life: 3000 })
  }
  onDeleteDone()
}
</script>
