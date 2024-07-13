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
        <div class="pl-2 xl:pl-4">
          <div class="my-0 text-lg font-extrabold">
            {{ state.title }}
          </div>
          <div class="text-xs italic">
            {{ state.subtitle }}
          </div>
        </div>

        <Button
          v-if="props.showMapButton"
          v-tooltip.bottom="'Passage en mode carte'"
          outlined
          severity="primary"
          small
          class="!p-1 !mr-2 !ml-4"
          @click="() => navigateTo('/map/' + props.token)"
        >
          <template #default>
            <AppIcon
              icon-name="mapPage"
              size="24px"
            />
          </template>
        </Button>

        <Button
          v-if="props.showSearchButton"
          v-tooltip.bottom="'Passage en mode liste'"
          outlined
          severity="primary"
          small
          class="!p-1 !mr-2 !ml-4"
          @click="() => navigateTo('/search/' + props.token)"
        >
          <template #default>
            <AppIcon
              icon-name="searchPage"
              size="24px"
            />
          </template>
        </Button>
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
                v-if="state.permissions?.can_add_entity"
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

              <Button
                severity="secondary"
                outlined
                label="Mode sombre/clair"
                @click="toggleDarkMode()"
              >
                <template #icon>
                  <AppIcon icon-name="lightDark" />
                </template>
              </Button>
            </div>
          </div>
        </Popover>

        <div
          class="hidden lg:flex justify-end items-center gap-2"
        >
          <Button
            label="Info"
            class="p-button-text"
            @click="showInformation = true"
          >
            <template #icon>
              <AppIcon icon-name="information" />
            </template>
          </Button>

          <Button
            v-if="state.permissions?.can_add_entity"
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

          <Button
            severity="secondary"
            outlined
            @click="toggleDarkMode()"
          >
            <template #icon>
              <AppIcon icon-name="lightDark" />
            </template>
          </Button>
        </div>
      </div>
    </template>
  </Toolbar>

  <Popover ref="filterOp">
    <ViewerFilterConfig
      :permissions = "state.permissions!"
      v-model:filteringTags="state.filteringTags"
      v-model:filteringCategories="state.filteringCategories"
      v-model:filteringEnums="state.filteringEnums"
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
      :permissions = "state.permissions!"
      v-model:filteringTags="state.filteringTags"
      v-model:filteringCategories="state.filteringCategories"
      v-model:filteringEnums="state.filteringEnums"
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
                Recherche d'un point sur la carte <i v-if="!state.permissions?.can_list_without_query">(4 caractères minimum)</i>
              </label>
              <InputGroup>
                <InputText
                  v-model="entitySearch"
                  placeholder="Tapez votre recherche ici"
                />
                <Button type="submit">
                  <template #icon>
                    <AppIcon icon-name="search" />
                  </template>
                </Button>
              </InputGroup>
            </form>

            <div v-if="currentSearchEntities().length > 0">
              <Divider
                type="dotted"
                class="!mb-2"
              />

              <div style="max-height: 500px; overflow-y: auto;">
                <DataView
                  :value="currentEntitiesResults!.entities"
                  :first="firstRow"
                  :rows="pageSize"
                  :total-records="currentEntitiesResults!.total_results"
                  paginator
                  :page-link-size="3"
                  data-key="id"
                  lazy
                  layout="list"
                  @page="onPage"
                >
                  <template #list>
                    <div
                      v-for="result in currentSearchEntities()"
                      :key="result.id"
                      class="result mb-2 p-2"
                      @click="entityChosen(result)"
                    >
                      <div>{{ result.display_name }}</div>
                      <div
                        v-if="result.locations.length == 0"
                        class="italic text-sm"
                      >
                        Aucune adresse connue
                      </div>
                      <span
                        v-for="(parent, i) in result.parents"
                        :key="parent.id"
                        class="text-xs"
                      >
                        {{ parent.display_name }} {{ i < result.parents.length - 1 ? ', ' : '' }}
                      </span>

                      <div class="mt-1">
                        <CategoryTag :category="state.getCategory(result.category_id)" />
                      </div>
                    </div>
                  </template>
                </DataView>
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
                  <span class="text-xs">{{ result.subtitle }}</span>
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

            <div class="text-xs">
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
    v-if="state.permissions?.can_add_entity"
    ref="entityAddForm"
    :family="state.activeFamily"
    :categories="
      state.categories
        .filter(category => category.family_id == state.activeFamily.id)
    "
  />
</template>

<script setup lang="ts">
import type Popover from 'primevue/popover'
import type { Coordinate } from 'ol/coordinate'
import type { PageState } from 'primevue/paginator'
import state from '~/lib/viewer-state'
import defaultLogo from '~/assets/logo_square.svg'
import type { Result as NominatimResult } from '~/lib/nominatim'
import { freeFormSearch } from '~/lib/nominatim'
import type { ViewerPaginatedCachedEntities, ViewerSearchedCachedEntity } from '~/lib'
import type { ViewerEntityAddForm } from '#build/components'

const toast = useToast()
const darkMode = useDarkMode()

export interface Props {
  showCategorySwitcher?: boolean
  showSearch?: boolean
  showFamilySwitcher?: boolean
  showMapButton?: boolean
  showSearchButton?: boolean
  token: string
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
  entityChosen: [ViewerSearchedCachedEntity]
}>()

const filterOp = ref<typeof Popover>()
const searchOp = ref<typeof Popover>()
const overflowPanel = ref<typeof Popover>()

const placeSearch: Ref<string> = ref('')
const entitySearch: Ref<string> = ref('')
const showInformation = ref(false)
const oneSearchMade = ref(false)
const filterPopupVisible = ref(false)
const currentLocationsResults: Ref<NominatimResult[]> = ref([])
const currentEntitiesResults: Ref<ViewerPaginatedCachedEntities | null> = ref(null)

const currentPage = ref(1)
const pageSize = ref(5)
const firstRow = ref(0)

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
  try {
    currentLocationsResults.value = await freeFormSearch(placeSearch.value)
    oneSearchMade.value = true
  }
  catch {
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: 'Impossible de charger les résultats',
      life: 3000,
    })
  }
}

async function searchEntity() {
  currentPage.value = 1
  pageSize.value = 5
  firstRow.value = 0
  await refreshResult()
}

function onPage(event: PageState) {
  currentPage.value = event.page + 1
  pageSize.value = event.rows
  firstRow.value = (currentPage.value - 1) * pageSize.value
  refreshResult()
}

async function refreshResult() {
  try {
    currentEntitiesResults.value = await state.searchEntities(entitySearch.value, currentPage.value, pageSize.value)
  }
  catch {
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: 'Impossible de charger les résultats',
      life: 3000,
    })
  }
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
  const gpsCoordinates: Coordinate = [
    result.lon,
    result.lat,
  ]
  emit('locationChosen', gpsCoordinates)
}

function entityChosen(result: ViewerSearchedCachedEntity) {
  emit('entityChosen', result)
  searchOp!.value!.hide()
}

function filtersChanged() {
  emit('filtersChanged')
}

function toggleDarkMode() {
  darkMode.toggle()
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
