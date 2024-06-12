<template>
  <div>
    <span class="flex gap-4 align-items-end mb-3">
      <form @submit.prevent="refreshTable">
        <InputGroup
          style="height: 36px; "
        >
          <InputText
            v-model="search_query"
            placeholder="Tapez votre recherche ici"
          />

          <Button
            type="button"
            severity="warning"
            label="Filtres"
            @click="(event: Event) => filterOp?.toggle(event)"
          >
            <template #icon>
              <AppIcon
                class="mr-1"
                icon-name="filter"
              />
            </template>
          </Button>

          <Button type="submit">
            <template #icon>
              <AppIcon icon-name="search" />
            </template>
          </Button>
        </InputGroup>
      </form>
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
        class="max-w-25rem"
      />
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Catégorie')"
        field="category_id"
        header="Catégorie"
      >
        <template #body="slotProps">
          <CategoryTag :category="categoryRecord[slotProps.data.category_id]" />
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Tags')"
        field="tags"
        header="Tags"
        class="max-w-18rem"
      >
        <template #body="slotProps">
          <DisplayedTag
            v-for="tag_id in slotProps.data.tags_ids.slice(0, max_tags_displayed)"
            :key="tag_id"
            :tag="tagRecord[tag_id]"
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
        :tag="tagRecord[tag_id]"
        class="m-1"
      />
    </OverlayPanel>

    <OverlayPanel ref="filterOp">
      <ViewerFilterConfig
        v-model:filteringTags="tagFilteringList"
        v-model:filteringCategories="categoryFilteringList"
        class="w-25rem"
        @filters-changed="refreshTable"
      />
    </OverlayPanel>
  </div>
</template>

<script setup lang="ts">
import type OverlayPanel from 'primevue/overlaypanel'
import type { PageState } from 'primevue/paginator'
import DisplayedTag from '~/components/DisplayedTag.vue'
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminPaginatedCachedEntities, Category, Tag } from '~/lib'
import state from '~/lib/admin-state'

const max_tags_displayed = 2

const familyId = useRoute().params.familyId as string
if (state.families == null)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title

const categoryList: Category[] = await state.client.listCategories()
type CategoryRecord = Record<string, Category>
const categoryRecord: CategoryRecord = categoryList.reduce((categories, category) => {
  categories[category.id] = category
  return categories
}, {} as CategoryRecord)
const categoryFilteringList = ref(categoryList.map(tag => ({ ...tag, active: true })))

const tagList: Tag[] = await state.client.listTags()
type TagRecord = Record<string, Tag>
const tagRecord: TagRecord = tagList.reduce((tags, tag) => {
  tags[tag.id] = tag
  return tags
}, {} as TagRecord)
const tagFilteringList = ref(tagList.map(tag => ({ ...tag, active: null })))

const overlay = ref<OverlayPanel>()
const filterOp = ref<OverlayPanel>()
const overlayed_tags: Ref<undefined | string[]> = ref(undefined)

const search_query = ref('')
const currentPage = ref(1)
const pageSize = ref(20)

// Initialize the ref with an empty array, then fetch to update entities asynchronously
const currentEntitiesResults: Ref<AdminPaginatedCachedEntities | null> = ref(null)
async function refreshTable() {
  currentEntitiesResults.value = await state.client.searchEntities(
    { page: currentPage.value, page_size: pageSize.value },
    {
      search: search_query.value,
      family: familyId,
      active_categories_ids: categoryFilteringList.value.filter(t => t.active).map(t => t.id),
      required_tags_ids: tagFilteringList.value.filter(t => t.active).map(t => t.id),
      excluded_tags_ids: tagFilteringList.value.filter(t => t.active === false).map(t => t.id),
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
// TODO : save table search results?

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
</script>
