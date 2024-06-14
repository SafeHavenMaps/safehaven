<template>
  <form
    class="flex gap-5 mx-4"
    @submit.prevent="onSave"
  >
    <div class="flex flex-grow-1 flex-column gap-3 max-w-30rem ">
      <AdminInputTextField
        id="display_name"
        v-model="editedEntity.display_name"
        label="Nom d'affichage"
      />
      <FormCategorySelect
        v-model="editedEntity.category_id"
        :categories="categories"
      />

      <FormAdressSelect
        id="locations"
        v-model="editedEntity.locations"
        label="locations"
      />

      <FormDynamicField
        id="data"
        v-model="editedEntity.data"
        label="Données"
      />
    </div>
    <div class="flex flex-column flex-grow-1 gap-3 max-w-30rem ">
      <FormTagSelect
        v-model="editedEntity.tags"
        :tags="tags"
      />

      <AdminInputSwitchField
        id="hidden"
        v-model="editedEntity.hidden"
        label="Cachée"
        helper-text="Si activé, cette entité ne sera pas visible publiquement, même si modérée.
      Utile pour des entités que vous souhaitez cacher à long terme sans les supprimer."
      />
      <AdminInputSwitchField
        id="moderated"
        v-model="editedEntity.moderated"
        label="Modérée"
        helper-text="Si activé, cette entité quittera la liste des entités en attente et sera rendue publique."
      />

      <AdminInputTextField
        id="moderation_notes"
        v-model="editedEntity.moderation_notes"
        label="Notes de modération"
        text-length="long"
        optional
      />

      <span class="flex gap-1 justify-content-end">
        <NuxtLink :to="`/admin/${familyId}/entities`">
          <Button
            label="Annuler"
            severity="secondary"
            :loading="processingRequest"
            :disabled="processingRequest"
          />
        </NuxtLink>
        <Button
          label="Sauvegarder"
          type="submit"
          :loading="processingRequest"
          :disabled="processingRequest || !editedEntity.display_name || !editedEntity.category_id"
        />
      </span>
    </div>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminNewOrUpdateEntity } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const familyId = useRoute().params.familyId as string
if (state.families == undefined)
  await state.fetchFamilies()
const familyTitle = state.families.filter(family => family.id == familyId)[0].title

// Fetch categories and tags if not already fetched
if (!state.categories) {
  await state.fetchCategories()
}
if (!state.tags) {
  await state.fetchTags()
}

const categories = computed(() => state.categories.filter(category => category.family_id == familyId))

const tags = state.tags

// Initialize editedEntity with default values for creation
const editedEntity: Ref<AdminNewOrUpdateEntity> = ref({
  category_id: '',
  data: {},
  display_name: '',
  hidden: false,
  locations: [],
  moderated: false,
  tags: [],
})

const processingRequest = ref(false)
const toast = useToast()

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
        `Nouvelle entité`,
        'entity',
        [],
        [
          { label: `${familyTitle}`, url: '/admin/families' },
          { label: 'Entités', url: `/admin/${familyId}/entities` },
          { label: `Édition d'une nouvelle entité`, url: `/admin/${familyId}/entities/new` },
        ],
)

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.createEntity(editedEntity.value)
    navigateTo(`/admin/${familyId}/entities`)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Entité créée avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de création de l\'entité', life: 3000 })
  }
  processingRequest.value = false
}
</script>
