<template>
  <div class="flex flex-column gap-2">
    <span class="flex gap-2">
      <Button
        v-if="!hasChildren"
        label="Ajouter un parent"
        outlined
        @click="parentSelectVisible=true"
      />
      <Button
        v-if="!hasParents"
        label="Ajouter un enfant"
        outlined
        @click="childSelectVisible=true"
      />
    </span>
    <b v-if="hasChildren">Liste des enfants </b>
    <b v-else-if="hasParents">Liste des parents </b>
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

    <DataTable
      v-if="hasParents || hasChildren"
      paginator
      state-storage="session"
      :state-key="table_key"
      data-key="id"
      :value="hasChildren ? props.mainEntity.children : props.mainEntity.parents"
      striped-rows
      :rows="10"
      :rows-per-page-options="[5, 10, 20, 50]"
      removable-sort
      class=" "
    >
      <Column
        field="display_name"
        header="Nom de l'entité"
        class="max-w-25rem"
        sortable
      />

      <Column
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
            :model-name="hasParents ? 'de la relation avec le parent' : `de la relation avec l'enfant`"
            :name="slotProps.data.display_name"
            @edit="navigateTo(`/admin/${familyId}/entities/${slotProps.data.id}`)"
            @delete="onDelete"
          />
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { AdminEntityWithRelations, Category, Tag } from '~/lib'
import state from '~/lib/admin-state'

const props = defineProps<{
  mainEntity: AdminEntityWithRelations
  categories: Category[]
  tags: Tag[]
  familyId: string
}>()

const emit = defineEmits<{
  updateKinship: []
}>()

const childSelectVisible = ref(false)
const parentSelectVisible = ref(false)

const hasParents = computed(() => !!props.mainEntity.parents.length)
const hasChildren = computed(() => !!props.mainEntity.children.length)

const table_key = `dt-state-entities-kinship-${props.mainEntity.id}`

const toast = useToast()

async function onChildAdd(child_entity: { entity_id: string, display_name: string }) {
  try {
    await state.client.registerEntityParent(props.mainEntity.id, child_entity.entity_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec l'enfant ${child_entity.display_name} ajoutée avec succès`, life: 3000 })
    emit('updateKinship')
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur d'ajout de la relation avec l'enfant ${child_entity.display_name}`, life: 3000 })
  }
}

async function onParentAdd(parent_entity: { entity_id: string, display_name: string }) {
  try {
    await state.client.registerEntityParent(parent_entity.entity_id, props.mainEntity.id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec le parent ${parent_entity.display_name} ajoutée avec succès`, life: 3000 })
    emit('updateKinship')
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur d'ajout de la relation avec le parent ${parent_entity.display_name}`, life: 3000 })
  }
}

async function onDelete(kin_id: string, kin_name: string, onDeleteDone: () => void) {
  if (hasParents.value) {
    try {
      await state.client.removeEntityParent(kin_id, props.mainEntity.id)
      toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec le parent ${kin_name} supprimée avec succès`, life: 3000 })
      emit('updateKinship')
    }
    catch {
      toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la relation avec le parent ${kin_name}`, life: 3000 })
    }
  }
  else {
    try {
      await state.client.removeEntityParent(props.mainEntity.id, kin_id)
      toast.add({ severity: 'success', summary: 'Succès', detail: `Relation avec l'enfant ${kin_name} supprimée avec succès`, life: 3000 })
      emit('updateKinship')
    }
    catch {
      toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression de la relation avec l'enfant ${kin_name}`, life: 3000 })
    }
  }
  onDeleteDone()
}
</script>
