<template>
  <Tabs value="0">
    <TabList>
      <Tab value="0">
        {{ $t('cmp.viewer.filterConfig.general') }}
      </Tab>
      <Tab
        v-if="hasFilteringTags"
        value="1"
      >
        {{ $t('cmp.viewer.filterConfig.tags') }}
      </Tab>
      <Tab
        v-if="hasFilteringEnums"
        value="2"
      >
        {{ $t('cmp.viewer.filterConfig.constraints') }}
      </Tab>
    </TabList>

    <TabPanels>
      <TabPanel value="0">
        <div>
          <span class="font-medium block mb-2">{{ $t('cmp.viewer.filterConfig.categories') }}</span>
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
            {{ $t('cmp.viewer.filterConfig.invert') }}
          </Button>
          <Button
            outlined
            size="small"
            class="m-1"
            @click="selectAllFilteringCategories"
          >
            {{ $t('cmp.viewer.filterConfig.selectAll') }}
          </Button>
          <Button
            outlined
            size="small"
            class="m-1"
            @click="resetFilteringCategories"
          >
            {{ $t('cmp.viewer.filterConfig.reset') }}
          </Button>
        </div>
        <div
          v-if="hasFilteringPrimaryTags"
          class="filter-settings mt-2"
        >
          <span class="font-medium block mb-2">{{ $t('cmp.viewer.filterConfig.filters') }}</span>
          <div
            v-for="tag in props.filteringTags.filter(t => t.is_primary_filter)"
            :key="tag.id"
            class="mb-2 p-1"
          >
            <div class="mb-1">
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
                    slotProps.option === false ? $t('cmp.viewer.filterConfig.hidden')
                    : slotProps.option === null
                      ? $t('cmp.viewer.filterConfig.displayed') : $t('cmp.viewer.filterConfig.required')
                  }}</span>
                </div>
              </template>
            </SelectButton>
          </div>
        </div>
      </TabPanel>

      <TabPanel
        v-if="hasFilteringTags"
        value="1"
      >
        <span class="font-medium block mb-2">{{ $t('cmp.viewer.filterConfig.tags') }}</span>
        <InputGroup
          class="mb-6 mt-2"
        >
          <InputText
            v-model="tagSearch"
            :placeholder="$t('cmp.viewer.filterConfig.searchTag')"
          />
          <Button
            v-tooltip.bottom="$t('cmp.viewer.filterConfig.clearSearch')"
            severity="warn"
            @click="tagSearch = ''"
          >
            <template #icon>
              <AppIcon icon-name="clear" />
            </template>
          </Button>
          <Button
            v-tooltip.bottom="$t('cmp.viewer.filterConfig.clearFilters')"
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
                    slotProps.option === false ? $t('cmp.viewer.filterConfig.hidden')
                    : slotProps.option === null
                      ? $t('cmp.viewer.filterConfig.displayed') : $t('cmp.viewer.filterConfig.required')
                  }}</span>
                </div>
              </template>
            </SelectButton>
          </div>
        </div>
      </TabPanel>

      <TabPanel
        v-if="hasFilteringEnums"
        value="2"
      >
        <div
          v-for="item in props.filteringEnums"
          :key="item.key"
        >
          <span class="font-medium block mb-2 mt-4">{{ item.title }}</span>
          <MultiSelect
            v-model="item.active"
            class="w-full"
            :options="item.values"
            option-label="label"
            option-value="value"
            @change="enumsFiltersChanged"
          />
        </div>
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<script setup lang="ts">
import type { Category, Tag, EnumFilter } from '~/lib'

export interface Props {
  maximumHeight?: string
  filteringTags: (Tag & { active: boolean | null })[]
  filteringCategories: (Category & { active: boolean })[]
  filteringEnums: EnumFilter[]
}

const props = withDefaults(defineProps<Props>(), {
  maximumHeight: '350px',
})

const hasFilteringTags = computed(() => props.filteringTags.filter(tag => !tag.is_primary_filter).length > 0)
const hasFilteringPrimaryTags = computed(() => props.filteringTags.filter(tag => tag.is_primary_filter).length > 0)
const hasFilteringEnums = computed(() => props.filteringEnums.length > 0)

const emit = defineEmits<{
  (event: 'filtersChanged'): void
}>()

const tagSearch = ref('')

function shownAdvancedTags() {
  const base = props.filteringTags.filter(t => !t.is_primary_filter)

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

function enumsFiltersChanged() {
  emit('filtersChanged')
}

function resetFilters() {
  props.filteringTags.filter(t => !t.is_primary_filter).forEach(t => t.active = null)
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
