<template>
  <div class="h-full flex flex-col">
    <ViewerNavbar
      :token="token"
      :show-category-switcher="false"
      :show-search="false"
      :show-family-switcher="false"
      :show-map-button="state.permissions?.can_access_entity"
      :show-search-button="false"
    />

    <Card
      class="m-2 p-2"
    >
      <template #header>
        <div class="flex flex-col gap-1">
          <span class="text-2xl font-bold">
            Recherche
          </span>
          <span
            v-if="state.permissions?.can_add_comment && !state.permissions?.can_access_entity"
            class="text-muted-color"
          >
            Pour ajouter un commentaire, veuillez d'abord rechercher l'entité à laquelle vous désirez l'ajouter
          </span>
        </div>
      </template>

      <template #content>
        <form @submit.prevent="submitSearch">
          <div class="hidden sm:block">
            <InputGroup>
              <InputText
                v-model="query"
                :placeholder="'Tapez votre recherche ici' + (state.permissions?.can_list_without_query ? '' : '(4 caractères minimum)')"
              />

              <Button
                type="button"
                severity="secondary"
                :label="state.activeFamily.title"
                @click="showFamilySwitcher"
              >
                <template #icon>
                  <AppIcon
                    :icon-name="state.activeFamily.icon_hash!"
                    dynamic
                    class="mr-1"
                    size="20px"
                  />
                </template>
              </Button>

              <Button
                type="button"
                severity="warn"
                label="Filtres"
                @click="showCriteriasModal()"
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
          </div>
          <div class="sm:hidden flex flex-col justify-end items-end gap-2">
            <InputGroup>
              <InputText
                v-model="query"
                :placeholder="'Tapez votre recherche ici' + (state.permissions?.can_list_without_query ? '' : '(4 caractères minimum)')"
              />
              <Button type="submit">
                <template #icon>
                  <AppIcon icon-name="search" />
                </template>
              </Button>
            </InputGroup>

            <div class="flex gap-2">
              <Button
                type="button"
                severity="secondary"
                :label="state.activeFamily.title"
                @click="showFamilySwitcher"
              >
                <template #icon>
                  <AppIcon
                    :icon-name="state.activeFamily.icon_hash!"
                    dynamic
                    class="mr-1"
                    size="20px"
                  />
                </template>
              </Button>

              <Button
                type="button"
                severity="warn"
                label="Filtres"
                @click="showCriteriasModal()"
              >
                <template #icon>
                  <AppIcon
                    class="mr-1"
                    icon-name="filter"
                  />
                </template>
              </Button>
            </div>
          </div>
        </form>
      </template>
    </Card>

    <Popover ref="familySwitcher">
      <ViewerFamilySwitcher />
    </Popover>

    <Card
      v-if="currentEntitiesResults"
      class="m-2 p-2"
    >
      <template #header>
        <span class="text-2xl font-bold">
          {{ currentEntitiesResults!.total_results }} {{ resultLabel() }}
        </span>
      </template>

      <template #content>
        <DataView
          :value="currentEntitiesResults!.entities"
          :first="firstRow"
          :rows="pageSize"
          :rows-per-page-options="[5, 10, 20, 50]"
          :total-records="currentEntitiesResults!.total_results"
          data-key="id"
          paginator
          lazy
          layout="list"
          @page="onPage"
        >
          <template #list>
            <ViewerFullResult
              v-for="entity in currentEntitiesResults!.entities"
              :key="`${entity.id}-${researchIncrement}`"
              :entity="entity"
              class="p-col-12"
            />
          </template>
        </DataView>
      </template>
    </Card>

    <Dialog
      v-if="state.hasActiveEntity"
      v-model:visible="state.hasActiveEntity"
      maximizable
      :style="{ width: '50rem' }"
      :breakpoints="{ '1199px': '75vw', '575px': '90vw' }"
      modal
      dismissable-mask
    >
      <template #header>
        <div class="flex items-center gap-2">
          <CategoryTag :category="state.activeEntity!.category" />
          <h3 class="grow font-bold text-lg m-0">
            {{ state.activeEntity!.entity.display_name }}
          </h3>
        </div>
      </template>

      <ViewerCommentAddForm
        v-if="state.permissions?.can_add_comment"
        :family="state.activeEntity!.family"
        :entity="state.activeEntity!.entity"
      />

      <ViewerCommonEntityDisplayer
        :entity="state.activeEntity!"
        :categories="state.categories"
        @entity-selected="displayEntityId"
      />
    </Dialog>

    <Dialog
      v-model:visible="showCriterias"
      maximizable
      header="Filtres"
      :style="{ width: '50rem' }"
      :breakpoints="{ '1199px': '75vw', '575px': '90vw' }"
      modal
    >
      <ViewerFilterConfig
        v-model:filteringTags="state.filteringTags"
        v-model:filteringCategories="state.filteringCategories"
        v-model:filteringEnums="state.filteringEnums"
      />
    </Dialog>

    <StartPopup />
  </div>
</template>

<script setup lang="ts">
import type Popover from 'primevue/popover'
import type { PageState } from 'primevue/paginator'
import type { ViewerPaginatedCachedEntities } from '~/lib'
import state from '~/lib/viewer-state'

const toast = useToast()

// Init state with url token
const route = useRoute()
const token = route.params.token as string
try {
  await state.bootstrapWithToken(token)
  if (!state.permissions?.can_list_entities)
    throw 'Unauthorized'
}
catch {
  toast.add({
    severity: 'error',
    summary: 'Erreur',
    detail: 'Impossible de charger les données',
    life: 3000,
  })
  if (state.redirectUrl) {
    window.location.href = state.redirectUrl
  }
  else {
    throw createError({
      statusCode: 404,
      statusMessage: 'Page Not Found',
      fatal: true,
    })
  }
}

const query = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const firstRow = ref(0)

const researchIncrement = ref(0)

const showCriterias = ref(false)

const currentEntitiesResults: Ref<ViewerPaginatedCachedEntities | null> = ref(null)
const familySwitcher = ref<typeof Popover>()

function resultLabel() {
  const result = currentEntitiesResults!.value?.total_results ?? 0
  return result > 1 ? 'résultats' : 'résultat'
}

async function submitSearch() {
  currentPage.value = 1
  pageSize.value = 20
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
    currentEntitiesResults.value = null
    researchIncrement.value++
    currentEntitiesResults.value = await state.searchEntities(
      query.value,
      currentPage.value,
      pageSize.value,
    )
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

async function displayEntityId(entityId: string) {
  try {
    await state.selectEntity(entityId)
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

async function showFamilySwitcher(event: Event) {
  familySwitcher.value!.toggle(event)
}

async function showCriteriasModal() {
  showCriterias.value = true
}
</script>

<style>
html:not(.sh-dark) {
  background-color: #f7f7f7;
}

html.sh-dark {
  background-color: #282828;
}
</style>
