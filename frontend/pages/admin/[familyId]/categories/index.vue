<template>
  <div>
    <span class="flex gap-4 flex-wrap">
      <IconField icon-position="left">
        <InputIcon>
          <AppIcon
            icon-name="search"
            class="-mt-1"
          />
        </InputIcon>
        <InputText
          v-model="(state.tablesFilters[tableKey]['global'] as DataTableFilterMetaData).value"
          :placeholder="$t('page.admin.familyId.categories.index.search')"
        />
      </IconField>
      <MultiSelect
        v-model="state.tablesSelectedColumns[tableKey]"
        :options="optionalColumns"
        option-label="label"
        option-value="key"
        display="chip"
        :placeholder="$t('page.admin.familyId.categories.index.selectColumns')"
        class="w-full md:w-80"
      />
    </span>
    <DataTable
      v-model:filters="state.tablesFilters[tableKey]"
      paginator
      paginator-template="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown CurrentPageReport"
      :current-page-report-template="$t('page.admin.familyId.categories.index.currentPageReport')"
      state-storage="session"
      :state-key="tableKey"
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
        :header="$t('page.admin.familyId.categories.index.title')"
        sortable
      />
      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('default_status')"
        field="default_status"
        :header="$t('page.admin.familyId.categories.index.defaultDisplay')"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.default_status ? $t('page.admin.familyId.categories.index.included') : $t('page.admin.familyId.categories.index.excluded')"
            :severity="slotProps.data.default_status ? 'success' : 'danger'"
          />
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('entity_count')"
        field="entity_count"
        :header="$t('page.admin.familyId.categories.index.entities')"
        sortable
      />
      <Column>
        <template #body="slotProps">
          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            model-name="de la catégorie"
            :name="slotProps.data.title"
            secure-delete
            :secure-delete-entity-count="slotProps.data.entity_count"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/${familyId}/categories/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { Category } from '~/lib'
import state from '~/lib/admin-state'

// const { t } = useI18n()
const toast = useToast()

const familyId = useRoute().params.familyId as string
if (state.families == null)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title

interface CategoryWCount extends Category { entity_count?: number }

// Initialize the ref with an empty array, then fetch to update categories asynchronously
const categories: Ref<CategoryWCount[]> = ref([])
async function refreshTable() {
  await state.fetchCategories()
  categories.value = state.categories
  categories.value = categories.value.filter(category => category.family_id == familyId)
  await state.getEntitiesCommentsCounts()
  categories.value.forEach((category) => {
    const counts = state.countsByCategory[category.id]
    category.entity_count = counts ? counts[0] : 0 // Set to 0 if cat not in count list
  })
}
refreshTable()

// const optionalColumns = ref(['Affichage par défaut', 'Entités'])
const optionalColumnsKeys = ['default_status', 'entity_count']
const optionalColumns = state.toSelectableColumns(optionalColumnsKeys)

const tableKey = `dt-state-categories-${familyId}`
const isSmallScreen = useMediaQuery('(max-width: 768px)')
const selectedColumKeys = isSmallScreen.value ? [] : ['default_status']
state.registerTable(tableKey, selectedColumKeys)

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

async function onDelete(category_id: string, category_name: string, onDeleteDone: () => void) {
  try {
    await state.deleteCategory(category_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Catégorie ${category_name} supprimée avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la catégorie ${category_name}`, life: 3000 })
  }
  onDeleteDone()
}
</script>
