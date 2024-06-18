<template>
  <Tabs value="0">
    <TabList>
      <Tab value="0">
        Général
      </Tab>
      <Tab value="1">
        Avancé
      </Tab>
    </TabList>

    <TabPanels>
      <TabPanel value="0">
        <div>
          <span class="font-medium text-surface-900 block mb-2">Catégories</span>
          <div
            :style="{
              maxHeight: props.maximumHeight,
              overflowY: 'auto',
            }"
          >
            <div
              v-for="category in props.filteringCategories"
              :key="category.id"
              class="flex items-center mb-2"
            >
              <ToggleSwitch
                v-model="category.active"
                @update:model-value="() => categoryFiltersChanged()"
              />
              <div
                class="round ml-2 mr-2"
                :style="{
                  backgroundColor: category.fill_color,
                  borderColor: category.border_color,
                }"
              >
                <AppIcon
                  size="16px"
                  dynamic
                  :icon-name="category.icon_hash!"
                />
              </div>
              {{ category.title }}
            </div>
          </div>
          <Button
            outlined
            size="small"
            class="m-1"
            @click="invertFilteringCategories"
          >
            Inverser
          </Button>
          <Button
            outlined
            size="small"
            class="m-1"
            @click="resetFilteringCategories"
          >
            Réinitialiser
          </Button>
          <Button
            outlined
            size="small"
            class="m-1"
            @click="selectAllFilteringCategories"
          >
            Tout sélectionner
          </Button>
        </div>
        <div
          class="filter-settings mt-2"
        >
          <span class="font-medium text-surface-900 block mb-2">Filtres</span>
          <div
            v-for="tag in props.filteringTags.filter(t => t.is_filter)"
            :key="tag.id"
            class="mb-2 p-1"
          >
            <div class="text-surface-800 mb-1">
              {{ tag.filter_description }}
            </div>

            <SelectButton
              v-model="tag.active"
              :options="[true, null, false]"
              option-label="title"
              aria-labelledby="custom"
              @change="tagFiltersChanged"
            >
              <template #option="slotProps">
                <div class="button-content">
                  <span>{{
                    slotProps.option === false ? 'Caché'
                    : slotProps.option === null
                      ? 'Affiché' : 'Requis'
                  }}</span>
                </div>
              </template>
            </SelectButton>
          </div>
        </div>
      </TabPanel>

      <TabPanel value="1">
        <span class="font-medium text-surface-900 block mb-2">Tags</span>
        <InputGroup
          class="mb-6 mt-2"
        >
          <InputText
            v-model="tagSearch"
            placeholder="Rechercher un tag"
          />
          <Button
            v-tooltip.bottom="'Retirer la recherche'"
            severity="warn"
            @click="tagSearch = ''"
          >
            <template #icon>
              <AppIcon icon-name="clear" />
            </template>
          </Button>
          <Button
            v-tooltip.bottom="'Retirer tous les filtres'"
            severity="danger"
            @click="resetFilters()"
          >
            <template #icon>
              <AppIcon icon-name="filterOff" />
            </template>
          </Button>
        </InputGroup>
        <div
          class="filter-settings"
          :style="{
            maxHeight: props.maximumHeight,
            overflowY: 'auto',
          }"
        >
          <div
            v-for="tag in shownAdvancedTags()"
            :key="tag.id"
            class="mb-2 p-1"
          >
            <div class="mb-1">
              <DisplayedTag :tag="tag" />
            </div>

            <SelectButton
              v-model="tag.active"
              :options="[true, null, false]"
              option-label="title"
              aria-labelledby="custom"
              @change="() => tagFiltersChanged()"
            >
              <template #option="slotProps">
                <div class="button-content">
                  <span>{{
                    slotProps.option === false ? 'Caché'
                    : slotProps.option === null
                      ? 'Affiché' : 'Requis'
                  }}</span>
                </div>
              </template>
            </SelectButton>
          </div>
        </div>
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<script setup lang="ts">
import type { Category, Tag } from '~/lib'

export interface Props {
  maximumHeight?: string
  filteringTags: (Tag & { active: boolean | null })[]
  filteringCategories: (Category & { active: boolean })[]
}

const props = withDefaults(defineProps<Props>(), {
  maximumHeight: '350px',
})

const emit = defineEmits<{
  (event: 'filtersChanged'): void
  // update:modelValue events uneeded as props are not reassigned only mutated through references
  // (event: 'update:modelValue:filteringCategories', categories: (Category & { active: boolean })[]): void
  // (event: 'update:modelValue:filteringTags', tags: (Tag & { active: boolean | null })[]): void
}>()

const tagSearch = ref('')

function shownAdvancedTags() {
  const base = props.filteringTags.filter(t => !t.is_filter)

  if (tagSearch.value === '') {
    return base
  }

  return base.filter(tag => tag.title.toLowerCase().includes(tagSearch.value.toLowerCase()))
}

function invertFilteringCategories() {
  props.filteringCategories.forEach((category) => {
    category.active = !category.active
  })
  categoryFiltersChanged()
}

function resetFilteringCategories() {
  props.filteringCategories.forEach((category) => {
    category.active = category.default_status
  })
  categoryFiltersChanged()
}

function selectAllFilteringCategories() {
  props.filteringCategories.forEach((category) => {
    category.active = true
  })
  categoryFiltersChanged()
}

function tagFiltersChanged() {
  emit('filtersChanged')
}

function categoryFiltersChanged() {
  emit('filtersChanged')
}

function resetFilters() {
  props.filteringTags.filter(t => !t.is_filter).forEach(t => t.active = null)
  tagFiltersChanged()
}
</script>

<style>
.filter-settings .button-content {
  z-index: 9;
}

.round {
  width: 1.5rem;
  height: 1.5rem;
  border-radius: 50%;
  border-width: 2px;
  border-style: solid;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
