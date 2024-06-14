<template>
  <div>
    <span class="flex gap-4 align-items-end mb-3">
      <form @submit.prevent="refreshTable">
        <InputGroup
          style="height: 36px; "
        >
          <InputText
            v-model="state.tablesQueryParams[table_key].search_query"
            placeholder="Tapez votre recherche ici"
          />

          <Button
            type="button"
            severity="warning"
            label="Filtres"
            @click="(event: Event) => filters_overlay?.toggle(event)"
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
      :rows="state.tablesQueryParams[table_key].pageSize"
      :first="firstRow"
      lazy
      paginator
      data-key="id"
      :value="currentEntitiesResults?.entities"
      :total-records="currentEntitiesResults?.total_results"
      striped-rows
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
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
              tooltip_excess_tags = slotProps.data.tags_ids.slice(max_tags_displayed)
              tags_tooltip!.show(event)
            }"
            @mouseleave="() => tags_tooltip!.hide()"
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
      ref="tags_tooltip"
    >
      <DisplayedTag
        v-for="tag_id in tooltip_excess_tags"
        :key="tag_id"
        :tag="tagRecord[tag_id]"
        class="m-1"
      />
    </OverlayPanel>

    <OverlayPanel ref="filters_overlay">
      <ViewerFilterConfig
        v-model:filteringTags="state.tablesQueryParams[table_key].tagFilteringList!"
        v-model:filteringCategories="state.tablesQueryParams[table_key].categoryFilteringList!"
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
if (state.families == null) {
  await state.fetchFamilies()
}
if (state.categories == null) {
  await state.fetchCategories()
}
if (state.tags == null) {
  await state.fetchTags()
}

const familyTitle = state.families.filter(family => family.id == familyId)[0].title

type CategoryRecord = Record<string, Category>

const categoryRecord: CategoryRecord = state.categories.reduce((categories, category) => {
  categories[category.id] = category
  return categories
}, {} as CategoryRecord)

type TagRecord = Record<string, Tag>
const tagRecord: TagRecord = state.tags.reduce((tags, tag) => {
  tags[tag.id] = tag
  return tags
}, {} as TagRecord)

const filters_overlay = ref<OverlayPanel>()
const tags_tooltip = ref<OverlayPanel>()
const tooltip_excess_tags: Ref<undefined | string[]> = ref(undefined)

const firstRow = ref(0)
const optionalColumns = ref(['Catégorie', 'Tags', 'Visibilité'])
const table_key = `dt-state-entities-${familyId}`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = ['Catégorie', 'Visibilité', 'Tags']
}

if (!(table_key in state.tablesQueryParams)) {
  state.tablesQueryParams[table_key] = {
    search_query: '',
    currentPage: 1,
    pageSize: 20,
    categoryFilteringList: state.categories
      .filter(category => category.family_id == familyId)
      .map(category => ({ ...category, active: true })),
    tagFilteringList: state.tags.map(tag => ({ ...tag, active: null })),
  }
}
else {
  firstRow.value = state.tablesQueryParams[table_key].currentPage * state.tablesQueryParams[table_key].pageSize - 1
}

let forceFullRefresh = false

watch([
  ...state.tablesQueryParams[table_key].categoryFilteringList!,
  ...state.tablesQueryParams[table_key].tagFilteringList!,
  state.tablesQueryParams[table_key].search_query,
], () => {
  forceFullRefresh = true
})

const currentEntitiesResults: Ref<AdminPaginatedCachedEntities | null> = ref(null)
async function refreshTable() {
  if (forceFullRefresh) {
    state.tablesQueryParams[table_key].currentPage = 1
    forceFullRefresh = false
  }

  currentEntitiesResults.value = await state.client.searchEntities(
    { page: state.tablesQueryParams[table_key].currentPage, page_size: state.tablesQueryParams[table_key].pageSize },
    {
      search: state.tablesQueryParams[table_key].search_query,
      family: familyId,
      active_categories_ids: state.tablesQueryParams[table_key].categoryFilteringList!.filter(t => t.active).map(t => t.id),
      required_tags_ids: state.tablesQueryParams[table_key].tagFilteringList!.filter(t => t.active).map(t => t.id),
      excluded_tags_ids: state.tablesQueryParams[table_key].tagFilteringList!.filter(t => t.active === false).map(t => t.id),
    },
  )
}

await refreshTable()

async function onPage(event: PageState) {
  state.tablesQueryParams[table_key].currentPage = event.page + 1
  state.tablesQueryParams[table_key].pageSize = event.rows
  await refreshTable()
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
</script>
