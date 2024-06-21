<template>
  <div
    class="mx-6"
  >
    <Tabs value="0">
      <TabList>
        <Tab
          value="0"
        >
          Contenu et modération
        </Tab>
        <Tab
          value="1"
        >
          Parenté
        </Tab>
        <Tab
          value="2"
        >
          Commentaires
        </Tab>
      </TabList>

      <TabPanels>
        <TabPanel value="0">
          <form
            class="flex flex-wrap gap-8"
            @submit.prevent="onSave"
          >
            <div class="flex grow flex-col gap-4 max-w-[30rem]">
              <AdminInputTextField
                id="display_name"
                v-model="editedEntity.display_name"
                label="Nom d'affichage"
                :variant="hasBeenEdited('display_name')"
              />
              <FormCategorySelect
                v-model="editedEntity.category_id"
                :categories="categories"
              />

              <FormDynamicField
                v-for="field in family.entity_form.fields.toSorted((field_a, field_b) => field_a.form_weight - field_b.form_weight)"
                :key="field.key"
                v-model:fieldContent="(editedEntity.data as EntityOrCommentData)[field.key]"
                :form-field="(field as FormField)"
              />
            </div>

            <div class="flex flex-col grow gap-4 max-w-[30rem]">
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

              <FormAdresses
                v-model:locations="editedEntity.locations as UnprocessedLocation[]"
              />

              <span class="flex gap-1 justify-end">
                <NuxtLink :to="`/admin/${familyId}/${entitiesUrl}`">
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
        </TabPanel>

        <TabPanel value="1">
          <AdminEntityKinshipTable
            :main-entity="fetchedEntity"
            :categories="categories"
            :tags="tags"
            :family-id="familyId"
            @update-kinship="async () => fetchedEntity = (await state.client.getEntity(entityId))"
          />
        </TabPanel>

        <TabPanel value="2">
          <Message
            v-if="entityComments.length == 0"
            severity="warn"
          >
            Aucun commentaire pour le moment
          </Message>
          <CommentsDisplayer
            v-else
            style="max-width: 60rem;"
            :comment-form-fields="family.comment_form.fields"
            :comments="entityComments"
            :public="false"
            @delete="onCommentDelete"
            @edit="commentId => navigateTo(`/admin/${familyId}/comments/${commentId}?urlEntityId=${entityId}`)"
          />
          <Button
            label="Nouveau commentaire"
            rounded
            outlined
            class="ml-3 mt-3"
            @click="navigateTo(`/admin/${familyId}/comments/new?urlEntityId=${entityId}`)"
          >
            <template #default>
              <div class="flex align-items-center">
                <AppIcon
                  class="-ml-1 mr-1"
                  icon-name="commentAdd"
                />
                Ajouter un commentaire
              </div>
            </template>
          </Button>
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { AdminComment, AdminNewOrUpdateEntity, EntityOrCommentData, FormField, UnprocessedLocation } from '~/lib'
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

const fetchedEntity = ref(await state.client.getEntity(entityId))
const entityComments = ref<AdminComment[]>([])
async function refreshComments() {
  entityComments.value = await state.client.listEntityComments(entityId)
}
refreshComments()

// Deep copy
const editedEntity: Ref<AdminNewOrUpdateEntity> = ref(JSON.parse(JSON.stringify(fetchedEntity.value)))

const processingRequest = ref(false)
const toast = useToast()
const entitiesUrl = useRoute().query.entitiesUrl

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition de l'entité ${fetchedEntity.value.display_name}`,
  'entity',
  [],
  [
    { label: `${family.title}`, url: '/admin/families' },
    { label: 'Entités', url: `/admin/${familyId}/${entitiesUrl}` },
    { label: `Édition de l'entité ${fetchedEntity.value.display_name}`, url: `/admin/${familyId}/entities/${entityId}?=${entitiesUrl}` },
  ],
)

function hasBeenEdited(field: keyof AdminNewOrUpdateEntity) {
  return editedEntity.value[field] !== fetchedEntity.value[field]
}

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.updateEntity(entityId, editedEntity.value)
    navigateTo(`/admin/${familyId}/${entitiesUrl}`)
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Entité modifiée avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification de l\'entité', life: 3000 })
  }
  processingRequest.value = false
  state.getEntitiesCommentsCounts()
}

async function onCommentDelete(comment_id: string, comment_author: string, onDeleteDone: () => void) {
  try {
    await state.client.deleteComment(comment_id)
    toast.add({ severity: 'success', summary: 'Succès', detail: `Commentaire ${comment_author} supprimé avec succès`, life: 3000 })
    refreshComments()
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: `Erreur de suppression du commentaire de ${comment_author}`, life: 3000 })
  }
  onDeleteDone()
}
</script>
