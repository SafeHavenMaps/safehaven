<template>
  <Toolbar>
    <template #start>
      <div class="flex align-items-center">
        <img
          height="40"
          width="40"
          alt="icon"
          :src="state.logo ?? defaultLogo"
        >
        <div class="pl-3">
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
      <ViewerFamilySwitcher v-if="props.showFamilySwitcher" />
    </template>

    <template #end>
      <div class="align-items-center">
        <Button
          label="Informations"
          class="p-button-text mr-2"
          @click="showInformation = true"
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
          v-if="props.showCategorySwitcher"
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
          v-if="props.showSearch"
          class="p-button-warning mr-2"
          @click="openSearchPanel"
        >
          <template #icon>
            <AppIcon icon-name="mapSearch" />
          </template>
        </Button>
      </div>
    </template>
  </Toolbar>

  <OverlayPanel ref="filterOp">
    <ViewerFilterConfig
      class="w-25rem"
      @filters-changed="filtersChanged"
    />
  </OverlayPanel>

  <OverlayPanel ref="searchOp">
    <div class="flex flex-column gap-3 w-25rem">
      <TabView>
        <TabPanel header="Chercher un point">
          <form @submit.prevent="searchEntity">
            <label for="placeSearch">
              Recherche d'un point sur la carte
            </label>

            <InputGroup>
              <InputText
                v-model="entitySearch"
              />
              <Button type="submit">
                <template #icon>
                  <AppIcon icon-name="search" />
                </template>
              </Button>
            </InputGroup>
          </form>

          <div v-if="currentSearchEntities().length > 0">
            <Divider type="dotted" />

            <div style="max-height: 500px; overflow-y: auto;">
              <div
                v-for="result in currentSearchEntities()"
                :key="result.id"
                class="result mb-2 p-2"
                @click="entityChosen(result)"
              >
                <div>{{ result.display_name }}</div>

                <div
                  v-if="result.parent_display_name"
                  class="text-xs"
                >
                  {{ result.parent_display_name }}
                </div>

                <div class="mt-1">
                  <CategoryTag :category="state.getCategory(result.category_id)" />
                </div>
              </div>
            </div>
          </div>
        </TabPanel>

        <TabPanel header="Chercher un lieu">
          <form @submit.prevent="searchLocation">
            <label for="placeSearch">
              Recherche d'une ville, d'un lieu, d'une adresse
            </label>

            <InputGroup>
              <InputText
                v-model="placeSearch"
                placeholder="Tours, France"
              />
              <Button type="submit">
                <template #icon>
                  <AppIcon icon-name="search" />
                </template>
              </Button>
            </InputGroup>
          </form>

          <div v-if="currentLocationsResults.length > 0">
            <Divider type="dotted" />

            <div style="max-height: 500px; overflow-y: auto;">
              <div
                v-for="result in currentLocationsResults"
                :key="result.id"
                class="result mb-2 p-2"
                @click="locationChosen(result)"
              >
                <span>{{ result.title }}</span><br>
                <span class="text-xs text-800">{{ result.subtitle }}</span>
              </div>
            </div>
          </div>

          <Divider type="dotted" />

          <div class="text-xs text-800">
            Recherche avec Nominatim © OpenStreetMap Contributor
          </div>
        </TabPanel>
      </TabView>
    </div>
  </OverlayPanel>

  <Dialog
    v-model:visible="showInformation"
    maximizable
    :style="{ width: '50rem' }"
    :breakpoints="{ '1199px': '75vw', '575px': '90vw' }"
    header="Informations"
    modal
  >
    <ViewerInformation />
  </Dialog>
</template>

<script setup lang="ts">
import type OverlayPanel from 'primevue/overlaypanel'
import type { Coordinate } from 'ol/coordinate'
import state from '~/lib/viewer-state'
import defaultLogo from '~/assets/logo_square.svg'
import type { Result as NominatimResult } from '~/lib/nominatim'
import { freeFormSearch } from '~/lib/nominatim'
import type { CachedEntity, PaginatedCachedEntitiesWithLocation } from '~/lib'

export interface Props {
  showCategorySwitcher?: boolean
  showSearch?: boolean
  showFamilySwitcher?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showCategorySwitcher: true,
  showSearch: true,
  showFamilySwitcher: true,
})

const emit = defineEmits<{
  filtersChanged: []
  locationChosen: [Coordinate]
  entityChosen: [CachedEntity]
}>()

const filterOp = ref<OverlayPanel>()
const searchOp = ref<OverlayPanel>()

const placeSearch: Ref<string> = ref('')
const entitySearch: Ref<string> = ref('')
const showInformation = ref(false)

const currentLocationsResults: Ref<NominatimResult[]> = ref([])
const currentEntitiesResults: Ref<PaginatedCachedEntitiesWithLocation | null> = ref(null)

function currentSearchEntities() {
  return currentEntitiesResults.value?.entities ?? []
}

function openFilterPanel(event: Event) {
  filterOp!.value!.toggle(event)
}

function openSearchPanel(event: Event) {
  searchOp!.value!.toggle(event)
}

async function searchLocation() {
  currentLocationsResults.value = await freeFormSearch(placeSearch.value)
}

async function searchEntity() {
  currentEntitiesResults.value = await state.searchEntitiesWithLocations(entitySearch.value)
}

function locationChosen(result: NominatimResult) {
  const gpsCoordinates: Coordinate = [result.lon, result.lat]
  emit('locationChosen', gpsCoordinates)
}

function entityChosen(result: CachedEntity) {
  emit('entityChosen', result)
  searchOp!.value!.hide()
}

function filtersChanged() {
  emit('filtersChanged')
}
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
</style>
