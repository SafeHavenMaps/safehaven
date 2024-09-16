<template>
  <div class="flex flex-col gap-2">
    <span class="flex gap-2">
      <Button
        v-if="!hasChildren"
        :label="$t('cmp.admin.entityKinshipTable.addParent')"
        outlined
        @click="parentSelectVisible=true"
      />
      <Button
        v-if="!hasParents"
        :label="$t('cmp.admin.entityKinshipTable.addChild')"
        outlined
        @click="childSelectVisible=true"
      />
    </span>
    <b v-if="hasChildren">{{ $t('cmp.admin.entityKinshipTable.childrenList') }}</b>
    <b v-else-if="hasParents">{{ $t('cmp.admin.entityKinshipTable.parentsList') }}</b>
    <span v-else>{{ $t('cmp.admin.entityKinshipTable.noKinship') }}</span>
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
        :header="$t('cmp.admin.entityKinshipTable.entityName')"
        class="max-w-[25rem]"
        sortable
      />

      <Column
        field="category_id"
        :header="$t('cmp.admin.entityKinshipTable.category')"
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
            :model-name="hasParents ? $t('cmp.admin.entityKinshipTable.parentRelationship') : $t('cmp.admin.entityKinshipTable.childRelationship')"
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
const { t } = useI18n()

async function onChildAdd(child_entity: { entity_id: string, display_name: string }) {
  try {
    await state.client.registerEntityParent(props.mainEntity.id, child_entity.entity_id)
    toast.add({ severity: 'success', summary: t('cmp.admin.entityKinshipTable.success'), detail: t('cmp.admin.entityKinshipTable.childAddedSuccess', { name: child_entity.display_name }), life: 3000 })
    emit('updateKinship')
  }
  catch {
    toast.add({ severity: 'error', summary: t('cmp.admin.entityKinshipTable.error'), detail: t('cmp.admin.entityKinshipTable.childAddError', { name: child_entity.display_name }), life: 3000 })
  }
}

async function onParentAdd(parent_entity: { entity_id: string, display_name: string }) {
  try {
    await state.client.registerEntityParent(parent_entity.entity_id, props.mainEntity.id)
    toast.add({ severity: 'success', summary: t('cmp.admin.entityKinshipTable.success'), detail: t('cmp.admin.entityKinshipTable.parentAddedSuccess', { name: parent_entity.display_name }), life: 3000 })
    emit('updateKinship')
  }
  catch {
    toast.add({ severity: 'error', summary: t('cmp.admin.entityKinshipTable.error'), detail: t('cmp.admin.entityKinshipTable.parentAddError', { name: parent_entity.display_name }), life: 3000 })
  }
}

async function onDelete(kin_id: string, kin_name: string, onDeleteDone: () => void) {
  if (hasParents.value) {
    try {
      await state.client.removeEntityParent(kin_id, props.mainEntity.id)
      toast.add({ severity: 'success', summary: t('cmp.admin.entityKinshipTable.success'), detail: t('cmp.admin.entityKinshipTable.parentRemovedSuccess', { name: kin_name }), life: 3000 })
      emit('updateKinship')
    }
    catch {
      toast.add({ severity: 'error', summary: t('cmp.admin.entityKinshipTable.error'), detail: t('cmp.admin.entityKinshipTable.parentRemoveError', { name: kin_name }), life: 3000 })
    }
  }
  else {
    try {
      await state.client.removeEntityParent(props.mainEntity.id, kin_id)
      toast.add({ severity: 'success', summary: t('cmp.admin.entityKinshipTable.success'), detail: t('cmp.admin.entityKinshipTable.childRemovedSuccess', { name: kin_name }), life: 3000 })
      emit('updateKinship')
    }
    catch {
      toast.add({ severity: 'error', summary: t('cmp.admin.entityKinshipTable.error'), detail: t('cmp.admin.entityKinshipTable.childRemoveError', { name: kin_name }), life: 3000 })
    }
  }
  onDeleteDone()
}
</script>
