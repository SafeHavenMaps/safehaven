<template>
  <form
    class="mx-4"
    @submit.prevent="onSave"
  >
    <TabView>
      <TabPanel header="Contenu et modération">
        <div class="flex flex-wrap gap-5">
          <div class="flex flex-grow-1 flex-column gap-3 max-w-30rem">
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
              label="Locations"
            />

            <FormDynamicField
              v-for="field in family.entity_form.fields.toSorted((field_a, field_b) => field_a.form_weight - field_b.form_weight)"
              :key="field.key"
              v-model:fieldContent="(editedEntity.data as EntityOrCommentData)[field.key]"
              :form-field="(field as FormField)"
            />
          </div>

          <div class="flex flex-column flex-grow-1 gap-3 max-w-30rem">
            Entitée crée le
            {{ Intl.DateTimeFormat('fr-FR', {
              dateStyle: 'long',
              timeStyle: 'short',
            }).format(new Date(fetchedEntity.created_at)) }}, mise à jour pour la dernière fois le
            {{ Intl.DateTimeFormat('fr-FR', {
              dateStyle: 'long',
              timeStyle: 'short',
            }).format(new Date(fetchedEntity.updated_at)) }}

            <FormTagSelect
              v-model="editedEntity.tags"
              :tags="tags"
            />

            <AdminInputSwitchField
              id="hidden"
              v-model="editedEntity.hidden"
              label="Cachée"
              helper-text="Si activé, cette entité ne sera pas visible publiquement, même si modérée. Utile pour des entités que vous souhaitez cacher à long terme sans les supprimer."
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

            <div class="flex align-items-center gap-2">
              <label>Adresses</label>
              <Button
                size="small"
                outlined
                severity="success"
                label="Ajouter une adresse"
                @click="addNewAddress"
              />
            </div>

            <DataTable :value="editedEntity.locations">
              <Column
                field="plain_text"
                header="Adresse"
                sortable
              />
              <Column>
                <template #body="slotProps">
                  <div class="flex justify-content-end">
                    <Button
                      outlined
                      rounded
                      severity="warning"
                      class="mr-1"
                      @click="() => editAddress(slotProps.index)"
                    >
                      <template #icon>
                        <AppIcon icon-name="edit" />
                      </template>
                    </Button>

                    <Button
                      outlined
                      rounded
                      severity="danger"
                      @click="() => removeAddress(slotProps.index)"
                    >
                      <template #icon>
                        <AppIcon icon-name="delete" />
                      </template>
                    </Button>
                  </div>
                </template>
              </Column>
            </DataTable>

            <Dialog
              v-model:visible="displayNominatimPicker"
              header="Sélectionner une adresse"
              :modal="true"
              :closable="false"
              :style="{ width: '40rem' }"
            >
              <NominatimPicker
                v-model="currentlyEditedLocation"
                @select="handleAddressSelect"
              />
              <template #footer>
                <Button
                  label="Annuler"
                  severity="secondary"
                  @click="cancelAddressChanges"
                />
                <Button
                  label="Sélectionner"
                  @click="applyAddressChanges"
                />
              </template>
            </Dialog>

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
        </div>
      </TabPanel>

      <TabPanel header="Parenté">
        <div class="flex flex-column gap-2 max-w-30rem">
          Ajouter/retirer parents et enfants

          <Button
            label="Ajouter parent/enfant"
            @click="childParentSelectVisible=true"
          />
        </div>

        <AdminInputEntitySelect
          v-model:visible="childParentSelectVisible"
          :categories="categories"
          :tags="tags"
          :family-id="familyId"
        />
      </TabPanel>

      <TabPanel header="Commentaires">
        Liste des commentaires
      </TabPanel>
    </TabView>
  </form>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminNewOrUpdateEntity, EntityOrCommentData, FormField, UnprocessedLocation } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const familyId = useRoute().params.familyId as string
const entityId = useRoute().params.id as string
if (state.families == undefined) {
  await state.fetchFamilies()
}
const family = state.families.filter(family => family.id == familyId)[0]

// Fetch categories and tags if not already fetched
if (!state.categories) {
  await state.fetchCategories()
}
if (!state.tags) {
  await state.fetchTags()
}

const categories = computed(() => state.categories.filter(category => category.family_id == familyId))

const tags = state.tags

const childParentSelectVisible = ref(false)

// Fetch the entity to be edited
const fetchedEntity = await state.client.getEntity(entityId)

// Deep copy of the fetched entity for editing
const editedEntity: Ref<AdminNewOrUpdateEntity> = ref(JSON.parse(JSON.stringify(fetchedEntity)))

const processingRequest = ref(false)
const toast = useToast()

const currentlyEditedLocation = ref<UnprocessedLocation | null>(null)
const displayNominatimPicker = ref(false)

function addNewAddress() {
  currentlyEditedLocation.value = { plain_text: '', lat: 0, long: 0 }
  displayNominatimPicker.value = true
}

function removeAddress(index: number) {
  editedEntity.value.locations.splice(index, 1)
}

function editAddress(index: number) {
  currentlyEditedLocation.value = { ...editedEntity.value.locations[index] }
  displayNominatimPicker.value = true
}

function cancelAddressChanges() {
  currentlyEditedLocation.value = null
  displayNominatimPicker.value = false
}

function applyAddressChanges() {
  if (currentlyEditedLocation.value) {
    const existingIndex = editedEntity.value.locations.findIndex(
      (location: { plain_text: string | undefined }) => location.plain_text === currentlyEditedLocation.value?.plain_text,
    )

    if (existingIndex === -1) {
      editedEntity.value.locations.push({ ...currentlyEditedLocation.value })
    }
    else {
      editedEntity.value.locations[existingIndex] = { ...currentlyEditedLocation.value }
    }

    displayNominatimPicker.value = false
  }
}

function handleAddressSelect(location: UnprocessedLocation) {
  currentlyEditedLocation.value = location
}

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition de l'entité ${fetchedEntity.display_name}`,
  'entity',
  [],
  [
    { label: `${family.title}`, url: '/admin/families' },
    { label: 'Entités', url: `/admin/${familyId}/entities` },
    { label: `Édition de l'entité ${fetchedEntity.display_name}`, url: `/admin/${familyId}/entities/${entityId}` },
  ],
)

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.updateEntity(entityId, editedEntity.value)
    navigateTo(`/admin/${familyId}/entities`)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Entité modifiée avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification de l\'entité', life: 3000 })
  }
  processingRequest.value = false
}
</script>
