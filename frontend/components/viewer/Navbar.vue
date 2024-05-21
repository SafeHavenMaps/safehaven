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
    <div class="flex flex-column gap-3 w-25rem">
      <div>
        <span class="font-medium text-900 block mb-2">Catégories</span>

        <div style="max-height: 500px; overflow-y: auto;">
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
            />
            {{ category.title }}
          </div>
        </div>
      </div>
      <div>
        <span class="font-medium text-900 block mb-2">Filtres</span>
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

  <OverlayPanel ref="searchOp">
    <div class="flex flex-column gap-3 w-25rem">
      <TabView>
        <TabPanel header="Chercher un lieu">
          <label for="placeSearch">
            Recherche d'une ville, d'un lieu, d'une adresse
          </label>

          <InputGroup>
            <InputText
              v-model="placeSearch"
              placeholder="Tours, France"
            />
            <Button @click="searchLocation">
              <template #icon>
                <AppIcon icon-name="search" />
              </template>
            </Button>
          </InputGroup>

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

            <Divider type="dotted" />

            <div class="text-xs text-800">
              Recherche avec Nominatim © OpenStreetMap Contributor
            </div>
          </div>
        </TabPanel>
        <TabPanel header="Chercher un point">
          <span>Entités</span>
        </TabPanel>
      </TabView>
    </div>
  </OverlayPanel>
</template>

<script setup lang="ts">
import type OverlayPanel from 'primevue/overlaypanel'
import type { Coordinate } from 'ol/coordinate'
import state from '~/lib/viewer-state'
import defaultLogo from '~/assets/logo_square.svg'
import type { Result as NominatimResult } from '~/lib/nominatim'
import { freeFormSearch } from '~/lib/nominatim'

const emit = defineEmits<{
  filtersChanged: []
  locationChosen: [Coordinate]
}>()

const filterOp = ref<OverlayPanel>()
const searchOp = ref<OverlayPanel>()

const placeSearch: Ref<string> = ref('')

const currentLocationsResults: Ref<NominatimResult[]> = ref([])

function openFilterPanel(event: Event) {
  filterOp!.value!.toggle(event)
}

function openSearchPanel(event: Event) {
  searchOp!.value!.toggle(event)
}

async function searchLocation() {
  currentLocationsResults.value = await freeFormSearch(placeSearch.value)
}

function locationChosen(result: NominatimResult) {
  const gpsCoordinates: Coordinate = [result.lon, result.lat]
  emit('locationChosen', gpsCoordinates)
}

function filtersChanged() {
  emit('filtersChanged')
}
</script>

<style scoped>
.round {
  width: 1.5rem;
  height: 1.5rem;
  border-radius: 50%;
  border-width: 2px;
  border-style: solid;
  display: inline-block;
}

.result {
  cursor: pointer;
  transition: background-color 0.2s;
}

.result:hover {
  background-color: #f0f0f0;
}
</style>
