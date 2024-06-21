<template>
  <div>
    <span class="flex gap-4 flex-wrap">
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
        class="w-full md:w-80"
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
      <Column v-if="state.is_admin">
        <template #body="slotProps">
          <div class="flex gap-2 justify-center">
            <Button
              outlined
              rounded
              severity="warn"
              @click="navigateTo(`/admin/families/${slotProps.data.id}/general`)"
            >
              <template #icon>
                <AppIcon icon-name="familyGeneralEdit" />
              </template>
            </Button>
            <Button
              outlined
              rounded
              severity="warn"
              @click="navigateTo(`/admin/families/${slotProps.data.id}/entities`)"
            >
              <template #icon>
                <AppIcon icon-name="entityFormEdit" />
              </template>
            </Button>
            <Button
              outlined
              rounded
              severity="warn"
              @click="navigateTo(`/admin/families/${slotProps.data.id}/comments`)"
            >
              <template #icon>
                <AppIcon icon-name="commentFormEdit" />
              </template>
            </Button>
            <AdminEditDeleteButtons
              :id="slotProps.data.id"
              model-name="de la famille"
              :name="slotProps.data.title"
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
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { Family } from '~/lib'
import state from '~/lib/admin-state'

const optionalColumns = ref(['Entités'])

const table_key = `dt-state-families`
const isSmallScreen = useMediaQuery('(max-width: 768px)')
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = isSmallScreen.value ? [] : ['Entités']
}
if (!(table_key in state.tablesFilters)) {
  state.tablesFilters[table_key] = {
    global: { value: null, matchMode: 'contains' },
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
  await state.fetchFamilies()
  await state.getEntitiesCommentsCounts()
  families.value = state.families
  families.value.forEach((family) => {
    const counts = state.countsByFamily[family.id]
    family.entity_count = counts ? counts[0] : 0 // As family not in count list if no entities attached
  })
}
refreshTable()

const toast = useToast()

async function onDelete(family_id: string, family_name: string, onDeleteDone: () => void) {
  try {
    await state.client.deleteFamily(family_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Famille ${family_name} supprimée avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la famille ${family_name}`, life: 3000 })
  }
  onDeleteDone()
}
</script>
