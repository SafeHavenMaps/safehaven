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
        v-if="selectedColumns.includes('Affichage par défaut')"
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
            model-name="la catégorie"
            :name="slotProps.data.title"
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
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { Category } from '~/lib'
import state from '~/lib/admin-state'

const familyId = useRoute().params.familyId as string

// Initialize the ref with an empty array, then fetch to update categories asynchronously
const categories: Ref<Category[]> = ref([])
async function refreshTable() {
  categories.value = await state.client.listCategories()
  categories.value = categories.value.filter(category => category.family_id == familyId)
}
refreshTable()

const optionalColumns = ref(['Affichage par défaut'])
const selectedColumns = ref(['Affichage par défaut'])

const filters = ref({
  global: { value: null, matchMode: FilterMatchMode.CONTAINS },
})

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
      url: `/admin/categories/new`,
    },
  ],
  [
    { label: 'Catégories', url: '/admin/categories' },
  ],
)

async function onDelete(category_id: string) {
  await state.client.deleteCategory(category_id)
  refreshTable()
}
</script>
