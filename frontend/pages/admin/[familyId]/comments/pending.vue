<template>
  <div>
    <span class="flex gap-4 flex-wrap">
      <IconField icon-position="left">
        <InputIcon>
          <AppIcon
            icon-name="search"
            class="-mt-1"
          />
        </InputIcon>
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
      paginator
      paginator-template="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown CurrentPageReport"
      current-page-report-template="&nbsp&nbsp&nbsp({totalPages} page·s, {totalRecords} commentaire·s)"
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      :value="comments"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['author', 'entity_display_name']"
      class=" "
    >
      <Column
        field="author"
        header="Auteur"
        class="max-w-[25rem]"
        sortable
      />
      <Column
        field="entity_display_name"
        header="Nom de l'entité"
        class="max-w-[25rem]"
        sortable
      />

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Catégorie')"
        field="entity_category_id"
        header="Catégorie"
        sortable
      >
        <template #body="slotProps">
          <CategoryTag :category="state.categoryRecord[slotProps.data.entity_category_id]" />
        </template>
      </Column>

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Créé le')"
        field="created_at"
        header="Créé le"
        sortable
      >
        <template #body="slotProps">
          {{ new Date(slotProps.data.created_at).toLocaleDateString() }}
        </template>
      </Column>
      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Mis à jour le')"
        field="updated_at"
        header="Mis à jour le"
        sortable
      >
        <template #body="slotProps">
          {{ new Date(slotProps.data.updated_at).toLocaleDateString() }}
        </template>
      </Column>
      <Column>
        <template #body="slotProps">
          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            model-name="du commentaire"
            :name="slotProps.data.entity_display_name"
            @delete="onDelete"
            @edit="id => navigateTo(`/admin/${familyId}/comments/${id}`)"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminListedComment } from '~/lib'
import state from '~/lib/admin-state'

const familyId = useRoute().params.familyId as string
if (state.families == null)
  await state.fetchFamilies()
if (state.categories == null)
  await state.fetchCategories()

const familyTitle = state.families.filter(family => family.id == familyId)[0].title

// Initialize the ref with an empty array, then fetch to update comments asynchronously
const comments: Ref<AdminListedComment[]> = ref([])
async function refreshTable() {
  comments.value = await state.client.listPendingComments()
  comments.value = comments.value.filter(comment => state.categoryRecord[comment.entity_category_id].family_id === familyId)
  state.getEntitiesCommentsCounts()
}
refreshTable()

const isSmallScreen = useMediaQuery('(max-width: 768px)')
const optionalColumns = ref(['Catégorie', 'Créé le', 'Mis à jour le'])
const table_key = `dt-state-comments-${familyId}`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = isSmallScreen.value ? [] : ['Créé le', 'Catégorie']
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
  'Commentaires en attente de modération',
  'pendingComment',
  [
    {
      icon: 'add',
      label: 'Nouveau commentaire',
      severity: 'success',
      url: `/admin/${familyId}/comments/new`,
    },
  ],
  [
    { label: `${familyTitle}`, url: '/admin/families' },
    { label: 'Commentaires en attente', url: `/admin/${familyId}/comments/pending` },
  ],
)

const toast = useToast()

async function onDelete(comment_id: string, comment_name: string, onDeleteDone: () => void) {
  try {
    await state.client.deleteComment(comment_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Commentaire ${comment_name} supprimé avec succès`, life: 3000 })
    refreshTable()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression du commentaire ${comment_name}`, life: 3000 })
  }
  onDeleteDone()
}
</script>
