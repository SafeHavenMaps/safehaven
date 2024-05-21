<template>
  <Toolbar>
    <template #start>
      <div class="flex align-items-center">
        <div class="flex-shrink-0">
          <img
            height="40"
            width="40"
            alt="icon"
            :src="state.logo ?? defaultLogo"
          >
        </div>
        <div class="flex-grow-1 pl-3">
          <h3 class="my-0">
            {{ state.title }}
          </h3>
          <span class="text-xs font-italic">
            {{ state.subtitle }}
          </span>
        </div>
      </div>
    </template>

    <template #center>
      <ViewerFamilySwitcher />
    </template>

    <template #end>
      <div class="align-items-center">
        <Button
          label="Informations"
          class="p-button-text mr-2"
        >
          <template #icon>
            <AppIcon icon-name="information" />
          </template>
        </Button>
        <Button
          label="Ajouter"
          class="p-button-success mr-2"
        >
          <template #icon>
            <AppIcon
              icon-name="addEntity"
              class="-ml-1 mr-1"
            />
          </template>
        </Button>
        <Button
          label="Filtres"
          class="p-button-help mr-2"
          @click="openFilterPanel"
        >
          <template #icon>
            <AppIcon
              icon-name="filter"
              class="-ml-1 mr-1"
            />
          </template>
        </Button>
        <Button
          icon="pi pi-search"
          class="p-button-warning mr-2"
        >
          <template #icon>
            <AppIcon icon-name="mapSearch" />
          </template>
        </Button>
        <Button
          icon="pi pi-cog"
          class="p-button-secondary"
        >
          <template #icon>
            <AppIcon icon-name="lightDark" />
          </template>
        </Button>
      </div>
    </template>
  </Toolbar>

  <OverlayPanel
    ref="filterOp"
  >
    <div class="flex flex-column gap-3 w-25rem">
      <div>
        <span class="font-medium text-900 block mb-2">Catégories</span>
        <ToggleButton
          v-for="category in state.filteringCategories"
          :key="category.id"
          v-model="category.active"
          :on-label="category.title"
          :off-label="category.title"
          class="mr-1 mb-1"
          @change="filtersChanged"
        />
      </div>
      <div>
        <span class="font-medium text-900 block mb-2">Tags</span>
        <div
          v-for="tag in state.filteringTags"
          :key="tag.id"
        >
          <span class="text-800">{{ tag.title }}</span>

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
    </div>
  </OverlayPanel>
</template>

<script setup lang="ts">
import type OverlayPanel from 'primevue/overlaypanel'
import state from '~/lib/viewer-state'
import defaultLogo from '~/assets/logo_square.svg'

const emit = defineEmits<{
  filtersChanged: []
}>()

const filterOp = ref<OverlayPanel>()

function openFilterPanel(event: Event) {
  filterOp!.value!.toggle(event)
}

function filtersChanged() {
  emit('filtersChanged')
}
</script>
