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
      v-model:rows="pageSize"
      paginator
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      :value="currentEntitiesResults?.entities"
      :total-records="currentEntitiesResults?.total_results"
      striped-rows
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['display_name']"
      class=" "
      @page="onPage"
    >
      <Column
        field="display_name"
        header="Nom d'affichage"
        sortable
      />
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Catégorie')"
        field="category_id"
        header="Catégorie"
        sortable
      >
        <template #body="slotProps">
          <CategoryTag :category="categories[slotProps.data.category_id]" />
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Tags')"
        field="tags"
        header="Tags"
        sortable
      >
        <template #body="slotProps">
          <DisplayedTag
            v-for="tag_id in slotProps.data.tags_ids.slice(0, max_tags_displayed)"
            :key="tag_id"
            :tag="tags[tag_id]"
            class="m-1"
          />
          <Badge
            v-if="slotProps.data.tags_ids.length > max_tags_displayed"
            ref="opener"
            :value="`+${slotProps.data.tags_ids.length - max_tags_displayed}`"
            severity="info"
            @mouseover="(event: Event) => {
              overlayed_tags = slotProps.data.tags_ids.slice(max_tags_displayed)
              overlay!.show(event)
            }"
            @mouseleave="() => overlay!.hide()"
          />
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Visibilité')"
        field="hidden"
        header="Visibilité"
        sortable
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
          <EditDeleteButtons
            :id="slotProps.data.id"
            model-name="de l'entité"
            :name="slotProps.data.display_name"
            :loading="processingRequest[slotProps.data.id]"
            secure-delete
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/${familyId}/entities/${id}`)"
          />
        </template>
      </Column>
    </DataTable>

    <OverlayPanel
      ref="overlay"
    >
      <DisplayedTag
        v-for="tag_id in overlayed_tags"
        :key="tag_id"
        :tag="tags[tag_id]"
        class="m-1"
      />
    </OverlayPanel>
  </div>
</template>

<script setup lang="ts">
import { FilterMatchMode } from 'primevue/api'
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type OverlayPanel from 'primevue/overlaypanel'
import type { PageState } from 'primevue/paginator'
import DisplayedTag from '~/components/DisplayedTag.vue'
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminPaginatedCachedEntities, Category, Tag } from '~/lib'
import state from '~/lib/admin-state'

const max_tags_displayed = 3

const familyId = useRoute().params.familyId as string
if (state.families == null)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title

type CategoryRecord = Record<string, Category>
const categories: CategoryRecord = (await state.client.listCategories()).reduce((categories, category) => {
  categories[category.id] = category
  return categories
}, {} as CategoryRecord)

type TagRecord = Record<string, Tag>
const tags: TagRecord = (await state.client.listTags()).reduce((tags, tag) => {
  tags[tag.id] = tag
  return tags
}, {} as TagRecord)

const overlay: Ref<null | InstanceType<typeof OverlayPanel>> = ref(null)
const overlayed_tags: Ref<null | string[]> = ref(null)

const search_query = ref('')
const currentPage = ref(1)
const pageSize = ref(20)

const activeCategories = ref(Object.keys(categories))
const activeRequiredTags = ref([])
const activeHiddenTags = ref([])

// Initialize the ref with an empty array, then fetch to update entities asynchronously
const currentEntitiesResults: Ref<AdminPaginatedCachedEntities | null> = ref(null)
async function refreshTable() {
  currentEntitiesResults.value = await state.client.searchEntities(
    { page: currentPage.value, page_size: pageSize.value },
    {
      search: search_query.value,
      family: familyId,
      active_categories_ids: activeCategories.value,
      required_tags_ids: activeRequiredTags.value,
      excluded_tags_ids: activeHiddenTags.value,
    },
  )
}
refreshTable()

function onPage(event: PageState) {
  currentPage.value = event.page + 1
  pageSize.value = event.rows
  refreshTable()
}

const optionalColumns = ref(['Catégorie', 'Tags', 'Visibilité'])
const table_key = `dt-state-entities-${familyId}`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = ['Catégorie', 'Visibilité', 'Tags']
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
  'Entités',
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
    { label: 'Entités', url: `/admin/${familyId}/entities` },
  ],
)

const processingRequest: Ref<Record<string, boolean>> = ref({})
const toast = useToast()

async function onDelete(entity_id: string, entity_name: string) {
  try {
    await state.client.deleteEntity(entity_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Entité ${entity_name} supprimée avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de l'entité ${entity_name}`, life: 3000 })
  }
}
</script>import type { PageState } from 'primevue/paginator'
