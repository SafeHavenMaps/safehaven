<template>
  <div>
    <span class="flex gap-6 items-end mb-4 flex-wrap">
      <form @submit.prevent="refreshTable">
        <InputGroup class="h-10">
          <InputText
            v-model="state.tablesQueryParams[tableKey].search_query"
            placeholder="Tapez votre recherche ici"
          />

          <Button
            type="button"
            severity="warn"
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
        v-model="state.tablesSelectedColumns[tableKey]"
        :options="optionalColumns"
        option-label="label"
        option-value="key"
        display="chip"
        placeholder="Sélectionner des colonnes"
        class="w-full md:w-80 h-10"
      />
    </span>

    <DataTable
      :rows="state.tablesQueryParams[tableKey].pageSize"
      :first="firstRow"
      lazy
      paginator
      paginator-template="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown CurrentPageReport"
      current-page-report-template="&nbsp&nbsp&nbsp({totalPages} page·s, {totalRecords} entité·s)"
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
        class="max-w-[25rem]"
      />
      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('category_id')"
        field="category_id"
        header="Catégorie"
      >
        <template #body="slotProps">
          <CategoryTag :category="state.categoryRecord[slotProps.data.category_id]" />
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[tableKey].includes('tags')"
        field="tags"
        header="Tags"
        class="max-w-72"
      >
        <template #body="slotProps">
          <DisplayedTag
            v-for="tag_id in slotProps.data.tags_ids.slice(0, max_tags_displayed)"
            :key="tag_id"
            :tag="state.tagRecord[tag_id]"
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
        v-if="state.tablesSelectedColumns[tableKey].includes('hidden')"
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
            :id="slotProps.data.entity_id"
            model-name="de l'entité"
            :name="slotProps.data.display_name"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/${familyId}/entities/${id}?entitiesUrl=entities`)"
          />
        </template>
      </Column>
    </DataTable>

    <Popover
      ref="tags_tooltip"
    >
      <DisplayedTag
        v-for="tag_id in tooltip_excess_tags"
        :key="tag_id"
        :tag="state.tagRecord[tag_id]"
        class="m-1"
      />
    </Popover>

    <Popover ref="filters_overlay">
      <ViewerFilterConfig
        v-model:filtering-tags="state.tablesQueryParams[tableKey].tagFilteringList!"
        v-model:filtering-categories="state.tablesQueryParams[tableKey].categoryFilteringList!"
        v-model:filtering-enums="state.tablesQueryParams[tableKey].enumsFilteringList!"
        class="w-[25rem]"
        @filters-changed="refreshTable"
      />
    </Popover>
  </div>
</template>

<script setup lang="ts">
import Popover from 'primevue/popover'
import type { PageState } from 'primevue/paginator'
import DisplayedTag from '~/components/DisplayedTag.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminPaginatedCachedEntities } from '~/lib'
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

const filters_overlay = ref<typeof Popover>()
const tags_tooltip = ref<typeof Popover>()
const tooltip_excess_tags: Ref<undefined | string[]> = ref(undefined)

const firstRow = ref(0)

// 'Catégorie', 'Tags', 'Visibilité'
const optionalColumnsKeys = ['category_id', 'tags', 'hidden']
const optionalColumns = state.toSelectableColumns(optionalColumnsKeys)

const tableKey = `dt-state-entities-${familyId}`
const isSmallScreen = useMediaQuery('(max-width: 768px)')
const selectedColumKeys = isSmallScreen.value ? [] : ['category_id', 'tags', 'hidden']
state.registerTable(tableKey, selectedColumKeys)

if (!(tableKey in state.tablesQueryParams)) {
  state.tablesQueryParams[tableKey] = {
    search_query: '',
    currentPage: 1,
    pageSize: 20,
    categoryFilteringList: state.categories
      .filter(category => category.family_id == familyId)
      .map(category => ({ ...category, active: true })),
    tagFilteringList: state.tags.map(tag => ({ ...tag, active: null })),
    enumsFilteringList: state.familyById(familyId).entity_form.fields
      .filter(f => f.indexed && (f.field_type === 'EnumMultiOption' || f.field_type === 'EnumSingleOption'))
      .map((f) => {
        return {
          key: f.key,
          title: f.display_name,
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          values: (f.field_type_metadata as any).options.map((v: any) => {
            return {
              label: v.label,
              value: v.value,
            }
          }),
          active: [],
        }
      }),
  }
}
else {
  firstRow.value = (state.tablesQueryParams[tableKey].currentPage - 1) * state.tablesQueryParams[tableKey].pageSize
}

let forceFullRefresh = false

watch([
  ...state.tablesQueryParams[tableKey].categoryFilteringList!,
  ...state.tablesQueryParams[tableKey].tagFilteringList!,
  state.tablesQueryParams[tableKey].search_query,
], () => {
  forceFullRefresh = true
})

const currentEntitiesResults: Ref<AdminPaginatedCachedEntities | null> = ref(null)
async function refreshTable() {
  if (forceFullRefresh) {
    state.tablesQueryParams[tableKey].currentPage = 1
    forceFullRefresh = false
  }

  currentEntitiesResults.value = await state.client.searchEntities(
    { page: state.tablesQueryParams[tableKey].currentPage, page_size: state.tablesQueryParams[tableKey].pageSize },
    {
      search: state.tablesQueryParams[tableKey].search_query,
      family: familyId,
      active_categories_ids: state.tablesQueryParams[tableKey].categoryFilteringList!.filter(t => t.active).map(t => t.id),
      required_tags_ids: state.tablesQueryParams[tableKey].tagFilteringList!.filter(t => t.active).map(t => t.id),
      excluded_tags_ids: state.tablesQueryParams[tableKey].tagFilteringList!.filter(t => t.active === false).map(t => t.id),
      enums_constraints: Object
        .fromEntries(
          state.tablesQueryParams[tableKey].enumsFilteringList!
            .filter(f => f.active.length > 0)
            .map(f => [f.key, f.active]),
        ),
    },
  )
}

await refreshTable()

async function onPage(event: PageState) {
  state.tablesQueryParams[tableKey].currentPage = event.page + 1
  state.tablesQueryParams[tableKey].pageSize = event.rows
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
      url: `/admin/${familyId}/entities/new?entitiesUrl=entities`,
    },
  ],
  [
    { label: `${familyTitle}`, url: '/admin/families' },
    { label: 'Entités', url: `/admin/${familyId}/entities` },
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
