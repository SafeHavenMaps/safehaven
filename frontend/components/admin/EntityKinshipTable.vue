<template>
  <div class="flex flex-column gap-2">
    <span  class="flex gap-2">
      <Button
        v-if="!props.mainEntity.children.length"
        label="Ajouter un parent"
        outlined
        @click="parentSelectVisible=true"
      />
      <Button
        v-if="!props.mainEntity.parents.length"
        label="Ajouter un enfant"
        outlined
        @click="childSelectVisible=true"
      />
    </span>
    <b v-if="props.mainEntity.children.length">Liste des enfants </b>
    <b v-else-if="props.mainEntity.parents.length">Liste des parents </b>
    <span v-else> Aucun parent ni enfant pour le moment </span>
    <AdminInputEntitySelect
      v-model:visible="parentSelectVisible"
      :categories="categories"
      :tags="tags"
      :family-id="familyId"
      @save_entity="onParentAdd"
    />
    <AdminInputEntitySelect
      v-model:visible="childSelectVisible"
      :categories="categories"
      :tags="tags"
      :family-id="familyId"
      @save_entity="onChildAdd"
    />

    <!-- <span class="flex gap-3">
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
        class="w-full md:w-20rem"
      />
    </span> -->
    <DataTable
      v-if="props.mainEntity.children.length || props.mainEntity.parents.length"
      v-model:filters="state.tablesFilters[table_key]"
      paginator
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      :value="props.mainEntity.children.length ? props.mainEntity.children : props.mainEntity.parents"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      :global-filter-fields="['author', 'display_name']"
      class=" "
    >
      <Column
        field="display_name"
        header="Nom de l'entité"
        class="max-w-25rem"
        sortable
      />

      <Column
        v-if="state.tablesSelectedColumns[table_key].includes('Catégorie')"
        field="category_id"
        header="Catégorie"
        sortable
      >
        <template #body="slotProps">
          <CategoryTag :category="state.categoryRecord[slotProps.data.category_id]" />
        </template>
      </Column>

      <Column>
        <template #body="slotProps">
          <AdminEditDeleteButtons
            :id="slotProps.data.id"
            :model-name="props.mainEntity.parents.length ? 'de la relation avec le parent' : `de la relation avec l'enfant`"
            edit-absent
            :name="slotProps.data.display_name"
            @delete="onDelete"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { FilterMatchMode } from 'primevue/api'
import type { DataTableFilterMetaData } from 'primevue/datatable'
import type { AdminEntityWithRelations, Category, Tag } from '~/lib'
import state from '~/lib/admin-state'

const props = defineProps<{
  mainEntity: AdminEntityWithRelations
  categories: Category[]
  tags: Tag[]
  familyId: string
}>()

const childSelectVisible = ref(false)
const parentSelectVisible = ref(false)

const optionalColumns = ref(['Catégorie', 'Créé le', 'Mis à jour le'])
const table_key = `dt-state-entities-kinship-${props.mainEntity.id}`
if (!(table_key in state.tablesSelectedColumns)) {
  state.tablesSelectedColumns[table_key] = ['Créé le', 'Catégorie']
}
if (!(table_key in state.tablesFilters)) {
  state.tablesFilters[table_key] = {
    global: { value: null, matchMode: FilterMatchMode.CONTAINS },
  }
}

const toast = useToast()

async function onChildAdd(child_entity: {id: string, display_name:string}){
  try {
    await state.client.registerEntityParent(props.mainEntity.id, child_entity.id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec l'enfant ${child_entity.display_name} ajoutée avec succès`, life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur d'ajout de la relation avec l'enfant ${child_entity.display_name}`, life: 3000 })
  }
}

async function onParentAdd(parent_entity: {id: string, display_name:string}){
  try {
    await state.client.registerEntityParent(parent_entity.id, props.mainEntity.id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec le parent ${parent_entity.display_name} ajoutée avec succès`, life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur d'ajout de la relation avec le parent ${parent_entity.display_name}`, life: 3000 })
  }
}

async function onDelete(kin_id: string, kin_name: string, onDeleteDone: () => void) {
  if (props.mainEntity.parents.length){
    try {
      await state.client.removeEntityParent(kin_id, props.mainEntity.id)
      toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec le parent ${kin_name} supprimée avec succès`, life: 3000 })
    }
    catch {
      toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la relation avec le parent ${kin_name}`, life: 3000 })
    }
  }
  else{
    try {
      await state.client.removeEntityParent(props.mainEntity.id, kin_id)
      toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec l'enfant ${kin_name} supprimée avec succès`, life: 3000 })
    }
    catch {
      toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la relation avec l'enfant ${kin_name}`, life: 3000 })
    }
  }
  onDeleteDone()
}
</script>
