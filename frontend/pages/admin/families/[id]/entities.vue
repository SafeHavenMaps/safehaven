<template>
  <Tabs value="0">
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
          class="-ml-4"
          :original-form-fields="fetchedFamily.entity_form.fields"
          :on-save-callback="onSave"
          kind-name="entité"
          kind="entity"
        />
      </TabPanel>
      <TabPanel value="1">
        <p class="text-muted-color">
          Édition directe du formulaire d'ajout en json, utile pour l'import/export.
          Les changements ne sont pas synchonisés avec ceux de l'édition visuelle en absence de sauvegarde.
        </p>
      <!-- <AdminFamiliesEditFormJson /> -->
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

const fetchedFamily = await state.client.getFamily(id)
const editedFamily = JSON.parse(JSON.stringify(fetchedFamily))

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
    editedFamily.entity_form.fields = newFormFields
    await state.client.updateFamily(id, editedFamily)
    navigateTo('/admin/families')
    return { error: undefined }
  }
  catch (error) {
    return { error: error as Error }
  }
}
</script>
