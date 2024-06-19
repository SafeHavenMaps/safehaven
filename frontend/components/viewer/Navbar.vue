<template>
  <Toolbar>
    <template #start>
      <div class="flex items-center">
        <img
          height="40"
          width="40"
          alt="icon"
          :src="state.logo ?? defaultLogo"
        >
        <div class="pl-4">
          <div class="my-0 text-lg font-extrabold">
            {{ state.title }}
          </div>
          <div class="text-xs italic">
            {{ state.subtitle }}
          </div>
        </div>
      </div>
    </template>

    <template #center>
      <div class="hidden lg:block">
        <ViewerFamilySwitcher
          v-if="props.showFamilySwitcher"
        />
      </div>
    </template>

    <template #end>
      <div class="items-center">
        <div class="block lg:hidden">
          <Button
            outlined
            severity="primary"
            small
            @click="openOverflowPanel"
          >
            <template #default>
              <AppIcon
                icon-name="menu"
                size="24px"
              />
            </template>
          </Button>
        </div>

        <Popover ref="overflowPanel">
          <div class="flex flex-col gap-4">
            <ViewerFamilySwitcher
              v-if="props.showFamilySwitcher"
            />

            <div class="flex flex-col gap-2">
              <Button
                label="Info"
                outlined
                severity="primary"
                @click="showInformation = true"
              >
                <template #icon>
                  <AppIcon icon-name="information" />
                </template>
              </Button>

              <Button
                label="Ajouter"
                severity="info"
                @click="openAddModal()"
              >
                <template #icon>
                  <AppIcon
                    icon-name="addEntity"
                  />
                </template>
              </Button>

              <Button
                v-if="props.showCategorySwitcher"
                label="Filtres"
                severity="primary"
                @click="openFilterPopup()"
              >
                <template #icon>
                  <AppIcon
                    icon-name="filter"
                  />
                </template>
              </Button>

              <Button
                v-if="props.showSearch"
                label="Recherche"
                severity="primary"
                @click="openSearchPanel"
              >
                <template #icon>
                  <AppIcon icon-name="mapSearch" />
                </template>
              </Button>
            </div>
          </div>
        </Popover>

        <div
          class="hidden lg:flex justify-end items-center gap-2"
        >
          <Button
            label="Informations"
            class="p-button-text"
            @click="showInformation = true"
          >
            <template #icon>
              <AppIcon icon-name="information" />
            </template>
          </Button>

          <Button
            label="Ajouter"
            severity="info"
            @click="openAddModal()"
          >
            <template #icon>
              <AppIcon
                icon-name="addEntity"
              />
            </template>
          </Button>

          <Button
            v-if="props.showCategorySwitcher"
            label="Filtres"
            severity="primary"
            @click="openFilterPanel"
          >
            <template #icon>
              <AppIcon
                icon-name="filter"
              />
            </template>
          </Button>

          <Button
            v-if="props.showSearch"
            severity="primary"
            @click="openSearchPanel"
          >
            <template #icon>
              <AppIcon icon-name="mapSearch" />
            </template>
          </Button>
        </div>
      </div>
    </template>
  </Toolbar>

  <Popover ref="filterOp">
    <ViewerFilterConfig
      v-model:filteringTags="state.filteringTags"
      v-model:filteringCategories="state.filteringCategories"
      class="w-[25rem]"
      @filters-changed="filtersChanged"
    />
  </Popover>

  <Dialog
    v-model:visible="filterPopupVisible"
    header="Filtres"
    modal
  >
    <ViewerFilterConfig
      v-model:filteringTags="state.filteringTags"
      v-model:filteringCategories="state.filteringCategories"
      @filters-changed="filtersChanged"
    />
  </Dialog>
  <Popover ref="searchOp">
    <div class="flex flex-col gap-4 w-[25rem]">
      <Tabs value="0">
        <TabList>
          <Tab value="0">
            Chercher un point
          </Tab>
          <Tab value="1">
            Chercher un lieu
          </Tab>
        </TabList>
        <TabPanels>
          <TabPanel value="0">
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

          <TabPanel value="1">
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

            <div v-if="oneSearchMade && currentLocationsResults.length > 0">
              <Divider type="dotted" />

              <div style="max-height: 500px; overflow-y: auto;">
                <div
                  v-for="result in currentLocationsResults"
                  :key="result.id"
                  class="result mb-2 p-2"
                  @click="locationChosen(result)"
                >
                  <span>{{ result.title }}</span><br>
                  <span class="text-xs text-surface-800">{{ result.subtitle }}</span>
                </div>
              </div>
            </div>
            <div
              v-else-if="oneSearchMade && currentLocationsResults.length == 0"
              class="my-2"
            >
              <Message severity="error">
                Aucun résultat trouvé
              </Message>
            </div>

            <Divider type="dotted" />

            <div class="text-xs text-surface-800">
              Recherche avec Nominatim © OpenStreetMap Contributor
            </div>
          </TabPanel>
        </TabPanels>
      </Tabs>
    </div>
  </Popover>

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

  <ViewerEntityAddForm
    ref="entityAddForm"
    :family="state.activeFamily"
    :categories="state.categories.filter(category => category.family_id == state.activeFamily.id)"
  />
</template>

<script setup lang="ts">
import type Popover from 'primevue/popover'
import type { Coordinate } from 'ol/coordinate'
import state from '~/lib/viewer-state'
import defaultLogo from '~/assets/logo_square.svg'
import type { Result as NominatimResult } from '~/lib/nominatim'
import { freeFormSearch } from '~/lib/nominatim'
import type { ViewerCachedEntity, ViewerPaginatedCachedEntitiesWithLocation } from '~/lib'
import type { ViewerEntityAddForm } from '#build/components'

const oneSearchMade = ref(false)

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

const entityAddForm = ref<typeof ViewerEntityAddForm>()

const emit = defineEmits<{
  filtersChanged: []
  locationChosen: [Coordinate]
  entityChosen: [ViewerCachedEntity]
}>()

const filterOp = ref<typeof Popover>()
const searchOp = ref<typeof Popover>()
const overflowPanel = ref<typeof Popover>()

const placeSearch: Ref<string> = ref('')
const entitySearch: Ref<string> = ref('')
const showInformation = ref(false)

const filterPopupVisible = ref(false)

const currentLocationsResults: Ref<NominatimResult[]> = ref([])
const currentEntitiesResults: Ref<ViewerPaginatedCachedEntitiesWithLocation | null> = ref(null)

function currentSearchEntities() {
  return currentEntitiesResults.value?.entities ?? []
}

function openFilterPanel(event: Event) {
  filterOp!.value!.toggle(event)
}

function openSearchPanel(event: Event) {
  searchOp!.value!.toggle(event)
}

function openOverflowPanel(event: Event) {
  overflowPanel!.value!.toggle(event)
}

async function searchLocation() {
  currentLocationsResults.value = await freeFormSearch(placeSearch.value)
  oneSearchMade.value = true
}

async function searchEntity() {
  currentEntitiesResults.value = await state.searchEntitiesWithLocations(entitySearch.value)
}

function openFilterPopup() {
  filterPopupVisible.value = true
  overflowPanel!.value!.hide()
}

function openAddModal() {
  entityAddForm.value!.open()
  overflowPanel.value!.hide()
}

function locationChosen(result: NominatimResult) {
  const gpsCoordinates: Coordinate = [result.lon, result.lat]
  emit('locationChosen', gpsCoordinates)
}

function entityChosen(result: ViewerCachedEntity) {
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
