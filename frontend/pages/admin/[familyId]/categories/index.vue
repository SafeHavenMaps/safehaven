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
      :value="categories"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['title']"
      class=" "
    >
      <Column
        field="title"
        header="Titre"
        sortable
      />
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Affichage par défaut')"
        field="default_status"
        header="Affichage par défaut"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.default_status ? 'Inclus' : 'Exclus'"
            :severity="slotProps.data.default_status ? 'success' : 'danger'"
          />
        </template>
      </Column>
      <Column>
        <template #body="slotProps">
          <EditDeleteButtons
            :id="slotProps.data.id"
            model-name="de la catégorie"
            :name="slotProps.data.title"
            :loading="processingRequest[slotProps.data.id]"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/${familyId}/categories/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { FilterMatchMode } from 'primevue/api'
import type { DataTableFilterMetaData } from 'primevue/datatable'
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { Category } from '~/lib'
import state from '~/lib/admin-state'

const familyId = useRoute().params.familyId as string
if (state.families == null)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title

// Initialize the ref with an empty array, then fetch to update categories asynchronously
const categories: Ref<Category[]> = ref([])
async function refreshTable() {
  categories.value = await state.client.listCategories()
  categories.value = categories.value.filter(category => category.family_id == familyId)
}
refreshTable()

const optionalColumns = ref(['Affichage par défaut'])
const table_key = `dt-state-categories-${familyId}`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = ['Affichage par défaut']
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
  'Catégories',
  'category',
  [
    {
      icon: 'add',
      label: 'Nouvelle catégorie',
      severity: 'success',
      url: `/admin/${familyId}/categories/new`,
    },
  ],
  [
    { label: `${familyTitle}`, url: '/admin/families' },
    { label: 'Catégories', url: `/admin/${familyId}/categories` },
  ],
)

const processingRequest: Ref<Record<string, boolean>> = ref({})
const toast = useToast()

async function onDelete(category_id: string, category_name: string) {
  try {
    await state.client.deleteCategory(category_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Catégorie ${category_name} supprimée avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la catégorie ${category_name}`, life: 3000 })
  }
}
</script>
