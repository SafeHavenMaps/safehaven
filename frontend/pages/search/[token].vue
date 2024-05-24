<template>
  <div class="h-full flex flex-column">
    <ViewerNavbar
      :show-category-switcher="false"
      :show-search="false"
    />

    <Card class="m-2 p-2">
      <template #header>
        <span class="text-2xl font-bold">
          Recherche
        </span>
      </template>

      <template #content>
        <form @submit="submitSearch">
          <InputGroup>
            <InputText
              v-model="query"
              placeholder="Tapez votre recherche ici"
            />

            <Button
              type="button"
              severity="warning"
              label="Critères avancés"
            >
              <template #icon>
                <AppIcon icon-name="filter" />
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

    <Card
      v-if="currentEntitiesResults"
      class="m-2 p-2"
    >
      <template #header>
        <span class="text-2xl font-bold">
          Résultats
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
  </div>
</template>

<script setup lang="ts">
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

const currentEntitiesResults: Ref<PaginatedCachedEntities | null> = ref(null)

async function submitSearch(event: Event) {
  event.preventDefault()
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
</script>

<style>
html, body {
  background-color: #f7f7f7;
}
</style>
