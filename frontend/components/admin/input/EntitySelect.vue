<template>
  <Dialog
    v-if="visible"
    :visible="visible"
    position="top"
    modal
    dismissable-mask
    :closable="false"
    :header="props.title"
    class="flex flex-col gap-2 mt-8"
    @update:visible="(value: any) => emit('update:visible', value)"
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
    <div v-if="currentEntitiesResults?.entities?.length">
      <Divider type="dotted" />

      <div style="max-height: 500px; overflow-y: auto;">
        <div
          v-for="result in currentEntitiesResults?.entities"
          :key="result.id"
          class="result mb-2 p-2"
          :class="{ 'selected-result': chosenEntity?.id === result.id }"
          @click="chosenEntity=result"
        >
          <div>{{ result.display_name }}</div>

          <div class="mt-1">
            <CategoryTag :category="state.categoryRecord[result.category_id]" />
          </div>
        </div>
      </div>
    </div>
    <Divider type="dotted" />
    <div class="flex justify-end gap-2">
      <Button
        type="button"
        label="Annuler"
        severity="secondary"
        @click="emit('update:visible', false)"
      />
      <Button
        type="button"
        label="Confirmer"
        :disabled="!Object.hasOwn(chosenEntity, 'display_name')"
        @click="() => {
          emit('update:visible', false)
          saveEntity(chosenEntity as AdminCachedEntity)
        }"
      />
    </div>

    <Popover ref="filters_overlay">
      <ViewerFilterConfig
        v-model:filtering-tags="tagFilteringList"
        v-model:filtering-categories="categoryFilteringList"
        v-model:filtering-enums="enumsFilteringList"
        @filters-changed="refreshSearch"
      />
    </Popover>
  </Dialog>
</template>

<script setup lang="ts">
import type Popover from 'primevue/popover'
import type {
  AdminCachedEntity,
  AdminPaginatedCachedEntities,
  Category,
  Tag,
  EnumFilter,
} from '~/lib'
import state from '~/lib/admin-state'

const props = withDefaults(defineProps<{
  title?: string
  categories: Category[]
  tags: Tag[]
  familyId: string
  visible: boolean
  previousEntityId?: string
}>(), {
  title: 'Choix d\'une entité',
})

const filters_overlay = ref<typeof Popover>()

const search_query = ref('')
const categoryFilteringList = ref<(Category & { active: boolean })[]>([])
const tagFilteringList = ref<(Tag & { active: boolean | null })[]>([])
const enumsFilteringList = ref<EnumFilter[]>([])

const currentEntitiesResults: Ref<AdminPaginatedCachedEntities | null> = ref(null)
const chosenEntity = ref<AdminCachedEntity | { id: string | undefined }>({ id: props.previousEntityId })

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
      enums_constraints: {}, // We do not support enums filtering on Entity selection
    },
  )
}

const emit = defineEmits(['save_entity', 'update:visible'])

function saveEntity(entity_id: AdminCachedEntity) {
  resetSearchParams()
  emit('save_entity', entity_id)
}

watch(
  () => props.previousEntityId,
  (newId) => {
    chosenEntity.value = { id: newId }
  },
)

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
