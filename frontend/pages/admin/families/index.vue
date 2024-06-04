<template>
  <div>
    <span class="flex gap-3">
      <IconField
        icon-position="left"
      >
        <InputIcon><AppIcon
          icon-name="search"
          class="-mt-1"
        /></InputIcon>
        <InputText
          v-model="(state.tablesFilters[table_key]['global'] as DataTableFilterMetaData).value"
          placeholder="Recherche"
        />
      </IconField>
      <MultiSelect
        v-model="state.tablesSelectedColumns[table_key]"
        :options="optionalColumns"
        display="chip"
        placeholder="Sélectionner des colonnes"
        class="w-full md:w-20rem"
      />
    </span>
    <DataTable
      v-model:filters="state.tablesFilters[table_key]"
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      paginator
      :value="families"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['title', 'comment_form', 'entity_form']"
      class=" "
    >
      <Column
        field="title"
        header="Titre"
        sortable
      />
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Entités')"
        field="entity_count"
        header="Entités"
        sortable
      />
      <Column>
        <template #body="slotProps">
          <div class="flex gap-2 justify-content-center">
            <Button
              outlined
              rounded
              severity="warning"
              @click="navigateTo(`/admin/families/${slotProps.data.id}/general`)"
            >
              <template #icon>
                <AppIcon icon-name="familyGeneralEdit" />
              </template>
            </Button>
            <Button
              outlined
              rounded
              severity="warning"
              @click="navigateTo(`/admin/families/${slotProps.data.id}/entities`)"
            >
              <template #icon>
                <AppIcon icon-name="entityFormEdit" />
              </template>
            </Button>
            <Button
              outlined
              rounded
              severity="warning"
              @click="navigateTo(`/admin/families/${slotProps.data.id}/comments`)"
            >
              <template #icon>
                <AppIcon icon-name="commentFormEdit" />
              </template>
            </Button>
            <EditDeleteButtons
              :id="slotProps.data.id"
              model-name="de la famille"
              :name="slotProps.data.title"
              :loading="processingRequest[slotProps.data.id]"
              secure-delete
              :secure-delete-entity-count="slotProps.data.entity_count"
              edit-absent
              @delete="onDelete"
              @edit="id => navigateTo(`/admin/families/${id}`)"
            />
          </div>
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { FilterMatchMode } from 'primevue/api'
import type { DataTableFilterMetaData } from 'primevue/datatable'
import EditDeleteButtons from '~/components/admin/EditDeleteButtons.vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { Family } from '~/lib'
import state from '~/lib/admin-state'

const optionalColumns = ref(['Entités'])

const table_key = `dt-state-families`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = []
}
if (!(table_key in state.tablesFilters)) {
  state.tablesFilters[table_key] = {
    global: { value: null, matchMode: FilterMatchMode.CONTAINS },
  }
}

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Familles',
  'family',
  [
    {
      icon: 'add',
      label: 'Nouvelle famille',
      severity: 'success',
      url: `/admin/families/new`,
    },
  ],
  [
    { label: 'Familles', url: '/admin/families' },
  ],
)

interface FamilyWCount extends Family { entity_count?: number }

const families: Ref<FamilyWCount[]> = ref([])
async function refreshTable() {
  families.value = await state.client.listFamilies()
  await state.getEntitiesCommentsCounts()
  families.value.forEach((family) => {
    const counts = state.countsByFamily[family.id]
    family.entity_count = counts ? counts[0] : 0 // As family not in count list if no entities attached
  })
}
refreshTable()

const processingRequest: Ref<Record<string, boolean>> = ref({})
const toast = useToast()

async function onDelete(family_id: string, family_name: string) {
  try {
    await state.client.deleteFamily(family_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Famille ${family_name} supprimée avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la famille ${family_name}`, life: 3000 })
  }
}
</script>