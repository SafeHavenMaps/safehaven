<template>
  <TabView>
    <TabPanel header="Général">
      <div>
        <span class="font-medium text-900 block mb-2">Catégories</span>

        <div
          :style="{
            maxHeight: props.maximumHeight,
            overflowY: 'auto',
          }"
        >
          <div
            v-for="category in state.filteringCategories"
            :key="category.id"
            class="flex align-items-center justify-between mb-2"
          >
            <InputSwitch
              v-model="category.active"
              @change="filtersChanged"
            />
            <div
              class="round ml-2 mr-2"
              :style="{
                backgroundColor: category.fill_color,
                borderColor: category.border_color,
              }"
            >
              <img
                height="16"
                :src="`/api/icons/categories/${category.icon_hash}`"
              >
            </div>
            {{ category.title }}
          </div>
        </div>
      </div>
      <div
        class="filter-settings mt-2"
      >
        <span class="font-medium text-900 block mb-2">Filtres</span>
        <div
          v-for="tag in state.filteringTags.filter(t => t.is_filter)"
          :key="tag.id"
          class="mb-2 p-1"
        >
          <div class="text-800 mb-1">
            {{ tag.filter_description }}
          </div>

          <SelectButton
            v-model="tag.active"
            :options="[true, null, false]"
            option-label="title"
            aria-labelledby="custom"
            @change="filtersChanged"
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

    <TabPanel
      header="Avancé"
    >
      <div
        class="filter-settings"
        :style="{
          maxHeight: props.maximumHeight,
          overflowY: 'auto',
        }"
      >
        <InputText
          v-model="tagSearch"
          class="mb-4 mt-2"
          placeholder="Rechercher un tag"
        />
        <span class="font-medium text-900 block mb-2">Tags</span>

        <div
          v-for="tag in shownAdvancedTags()"
          :key="tag.id"
          class="mb-2 p-1"
        >
          <div class="mb-1">
            <Tag
              class="mr-1 mb-1"
            >
              {{ tag.title }}
            </Tag>
          </div>

          <SelectButton
            v-model="tag.active"
            :options="[true, null, false]"
            option-label="title"
            aria-labelledby="custom"
            @change="filtersChanged"
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
  </TabView>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'

export interface Props {
  maximumHeight?: string
}

const props = withDefaults(defineProps<Props>(), {
  maximumHeight: '350px',
})

const emit = defineEmits<{
  filtersChanged: []
}>()

const tagSearch = ref('')

function shownAdvancedTags() {
  const base = state.filteringTags.filter(t => !t.is_filter)

  if (tagSearch.value === '') {
    return base
  }

  return base.filter(tag => tag.title.toLowerCase().includes(tagSearch.value.toLowerCase()))
}

function filtersChanged() {
  emit('filtersChanged')
}
</script>

<style>
.filter-settings .button-content {
  z-index: 9;
}
</style>
