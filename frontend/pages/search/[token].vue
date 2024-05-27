<template>
  <div class="h-full flex flex-column">
    <ViewerNavbar
      :show-category-switcher="false"
      :show-search="false"
      :show-family-switcher="false"
    />

    <Card
      class="m-2 p-2"
    >
      <template #header>
        <span class="text-2xl font-bold">
          Recherche
        </span>
      </template>

      <template #content>
        <form @submit.prevent="submitSearch">
          <InputGroup>
            <InputText
              v-model="query"
              placeholder="Tapez votre recherche ici"
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
                  dynamic-type="families"
                  class="mr-1"
                  size="20px"
                />
              </template>
            </Button>

            <Button
              type="button"
              severity="warning"
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
        </form>
      </template>
    </Card>

    <OverlayPanel ref="familySwitcher">
      <ViewerFamilySwitcher />
    </OverlayPanel>

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
          :paginator="true"
          :rows="pageSize"
          :total-records="currentEntitiesResults!.total_results"
          data-key="id"
          layout="list"
          @page="onPage"
        >
          <template #list>
            <ViewerFullResult
              v-for="entity in currentEntitiesResults!.entities"
              :key="entity.id"
              :entity="entity"
              class="p-col-12"
            />
          </template>
        </DataView>
      </template>
    </Card>

    <Dialog
      v-model:visible="state.hasActiveEntity"
      maximizable
      :style="{ width: '50rem' }"
      :breakpoints="{ '1199px': '75vw', '575px': '90vw' }"
      modal
    >
      <template #header>
        <div
          v-if="
            state.activeEntity"
          class="flex align-items-center gap-2"
        >
          <CategoryTag :category="state.activeEntity!.category" />
          <h3 class="m-0">
            {{ state.activeEntity!.entity.display_name }}
          </h3>
        </div>
      </template>

      <ViewerCommonEntityDisplayer
        v-if="state.activeEntity"
        :entity="state.activeEntity!"
        :categories="state.categories"
        @entity-selected="displayEntityId"
      />
    </Dialog>

    <Dialog
      v-model:visible="showCriterias"
      maximizable
      :style="{ width: '50rem' }"
      :breakpoints="{ '1199px': '75vw', '575px': '90vw' }"
      modal
    >
      <template #header>
        <h3 class="m-0">
          Filtres
        </h3>
      </template>

      <ViewerFilterConfig />
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type OverlayPanel from 'primevue/overlaypanel'
import type { PageState } from 'primevue/paginator'
import type { PaginatedCachedEntities } from '~/lib'
import state from '~/lib/viewer-state'

// Init state with url token
const route = useRoute()
const token = route.params.token as string
await state.bootstrapWithToken(token) // TODO: Redirect to 404 if token is invalid

const query = ref('')
const currentPage = ref(1)
const pageSize = ref(20)

const showCriterias = ref(false)

const currentEntitiesResults: Ref<PaginatedCachedEntities | null> = ref(null)
const familySwitcher = ref<OverlayPanel>()

function resultLabel() {
  const result = currentEntitiesResults!.value?.total_results ?? 0
  return result > 1 ? 'résultats' : 'résultat'
}

async function submitSearch() {
  currentPage.value = 1
  pageSize.value = 20
  await refreshResult()
}

function onPage(event: PageState) {
  currentPage.value = event.page + 1
  pageSize.value = event.rows
  refreshResult()
}

async function refreshResult() {
  currentEntitiesResults.value = await state.searchEntities(
    query.value,
    currentPage.value,
    pageSize.value,
  )
}

async function displayEntityId(entityId: string) {
  await state.selectEntity(entityId)
}

async function showFamilySwitcher(event: Event) {
  familySwitcher.value!.toggle(event)
}

async function showCriteriasModal() {
  showCriterias.value = true
}
</script>

<style>
html, body {
  background-color: #f7f7f7;
}
</style>
