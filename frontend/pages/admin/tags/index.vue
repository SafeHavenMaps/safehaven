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
      :value="tags"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['title', 'filter_description']"
      class=" "
    >
      <Column
        field="title"
        header="Titre"
        sortable
      />
      <Column
        v-if="selectedColumns.includes('Filtrage')"
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
        v-if="selectedColumns.includes('Valeur de filtre par défaut')"
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
        v-if="selectedColumns.includes('Description de filtre')"
        header="Description de filtre"
        field="filter_description"
        sortable
      />

      <Column>
        <template #body="slotProps">
          <EditDeleteButtons
            :id="slotProps.data.id"
            model-name="le tag"
            :name="slotProps.data.token"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/tags/${id}`)"
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
import type { Tag } from '~/lib'
import state from '~/lib/admin-state'

const optionalColumns = ref(['Filtrage', 'Valeur de filtre par défaut', 'Description de filtre'])
const selectedColumns = ref(['Filtrage', 'Valeur de filtre par défaut'])

const filters = ref({
  global: { value: null, matchMode: FilterMatchMode.CONTAINS },
})

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
const tags: Ref<Tag[]> = ref([]);
(async () => {
  tags.value = await state.client.listTags()
})()

async function onDelete(tag_id: string) {
  await state.client.deleteTag(tag_id)
}
</script>