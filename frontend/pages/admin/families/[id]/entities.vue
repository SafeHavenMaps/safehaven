<template>
  <Tabs v-model:value="tabValue">
    <TabList>
      <Tab value="0">
        Éditeur visuel
      </Tab>
      <Tab value="1">
        Éditeur avancé
      </Tab>
    </TabList>
    <TabPanels>
      <TabPanel value="0">
        <p class="text-muted-color">
          Une page additionelle est insérée automatiquement au début du formulaire,
          contenant le titre de l'entité, sa position lontitude/lagitude et son addresse.
        </p>

        <AdminFamiliesEditForm
          :original-form-fields="fetchedFamily.entity_form.fields"
          :categories="categories"
          :on-save-callback="onSave"
          class="-ml-4"
          kind="entity"
          kind-name="entité"
        />
      </TabPanel>
      <TabPanel value="1">
        <AdminFamiliesEditFormJson
          :original-form-fields="fetchedFamily.entity_form.fields"
          :on-sync-callback="onSynchronise"
          kind="entity"
          kind-name="entité"
        />
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { FormField } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

if (!state.is_admin)
  navigateTo('/admin/home')

const id = useRoute().params.id as string
const tabValue = ref('0')

const fetchedFamily = await state.fetchFamily(id)
await state.fetchCategories()
const categories = state.categories.filter(category => category.family_id == fetchedFamily.id)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition du formulaire d'ajout d'entités de la famille ${fetchedFamily.title}`,
  'family',
  [],
  [
    { label: 'Familles', url: '/admin/families' },
    { label: `Édition du formulaire d'ajout d'entités de la famille ${fetchedFamily.title}`, url: `/admin/families/${id}/general` },
  ],
)

async function onSave(newFormFields: FormField[]): Promise<{ error: Error | undefined }> {
  try {
    fetchedFamily.entity_form.fields = newFormFields
    await state.client.updateFamily(id, fetchedFamily)
    navigateTo('/admin/families')
    return { error: undefined }
  }
  catch (error) {
    return { error: error as Error }
  }
}

async function onSynchronise(newFormFields: FormField[]): Promise<{ error: Error | undefined }> {
  try {
    fetchedFamily.entity_form.fields = newFormFields
    tabValue.value = '0' // Switch back to the visual editor tab after synchronization
    return { error: undefined }
  }
  catch (error) {
    return { error: error as Error }
  }
}
</script>
