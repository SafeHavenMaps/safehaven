<template>
  <Dialog
    v-if="visible"
    :visible="visible"
    position="top"
    modal
    dismissable-mask
    :closable="false"
    :header="props.title"
    class="flex flex-column gap-2 mt-5"
    @update:visible="value => emit('update:visible', value)"
  >
    <form @submit.prevent="refreshSearch">
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
    <div v-if="currentEntitiesResults?.entities?.length">
      <Divider type="dotted" />

      <div style="max-height: 500px; overflow-y: auto;">
        <div
          v-for="result in currentEntitiesResults?.entities"
          :key="result.id"
          class="result mb-2 p-2"
          :class="{ 'selected-result': chosenEntity === result.id }"
          @click="chosenEntity=result.id"
        >
          <div>{{ result.display_name }}</div>

          <div
            v-if="result.parent_display_name"
            class="text-xs"
          >
            {{ result.parent_display_name }}
          </div>

          <div class="mt-1">
            <CategoryTag :category="categoryRecord[result.category_id]" />
          </div>
        </div>
      </div>
    </div>
    <Divider type="dotted" />
    <div class="flex justify-content-end gap-2">
      <Button
        type="button"
        label="Annuler"
        severity="secondary"
        @click="emit('update:visible', false)"
      />
      <Button
        type="button"
        label="Confirmer"
        :disabled="chosenEntity == null"
        @click="() => {
          emit('update:visible', false)
          saveEntity(chosenEntity!)
        }"
      />
    </div>

    <OverlayPanel ref="filters_overlay">
      <ViewerFilterConfig
        v-model:filteringTags="tagFilteringList!"
        v-model:filteringCategories="categoryFilteringList!"
        @filters-changed="refreshSearch"
      />
    </OverlayPanel>
  </Dialog>
</template>

<script setup lang="ts">
import type OverlayPanel from 'primevue/overlaypanel'
import type { AdminPaginatedCachedEntities, Category, Tag } from '~/lib'
import state from '~/lib/admin-state'

const props = withDefaults(defineProps<{
  title?: string
  categories: Category[]
  tags: Tag[]
  familyId: string
  visible: boolean
}>(), {
  title: 'Choix d\'une entité',
})

const filters_overlay = ref<OverlayPanel>()

const search_query = ref('')
const categoryFilteringList = ref<(Category & { active: boolean })[]>([])
const tagFilteringList = ref<(Tag & { active: boolean | null })[]>([])

const currentEntitiesResults: Ref<AdminPaginatedCachedEntities | null> = ref(null)
const chosenEntity = ref<string | null> (null)

type CategoryRecord = Record<string, Category>
const categoryRecord: CategoryRecord = state.categories.reduce((categories, category) => {
  categories[category.id] = category
  return categories
}, {} as CategoryRecord)

function resetSearchParams() {
  search_query.value = ''
  categoryFilteringList.value = (JSON.parse(JSON.stringify(props.categories))).map((category: Category) => ({ ...category, active: true }))
  tagFilteringList.value = (JSON.parse(JSON.stringify(props.tags))).map((tag: Tag) => ({ ...tag, active: null }))
  currentEntitiesResults.value = null
}

async function refreshSearch() {
  currentEntitiesResults.value = await state.client.searchEntities(
    { page: 1, page_size: 5 },
    {
      search: search_query.value,
      family: props.familyId,
      active_categories_ids: categoryFilteringList!.value.filter(t => t.active).map(t => t.id),
      required_tags_ids: tagFilteringList!.value.filter(t => t.active).map(t => t.id),
      excluded_tags_ids: tagFilteringList!.value.filter(t => t.active === false).map(t => t.id),
    },
  )
}

const emit = defineEmits(['save_entity', 'update:visible'])

function saveEntity(entity_id: string) {
  resetSearchParams()
  emit('save_entity', entity_id)
}

resetSearchParams()
</script>

<style scoped>
.result {
  cursor: pointer;
  transition: background-color 0.2s;
  border-radius: 0.25rem;
}

.result:hover {
  background-color: #f0f0f0;
}

.selected-result {
  background-color: #e0e0e0;
}
</style>
