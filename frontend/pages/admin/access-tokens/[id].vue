<template>
  <form
    class="flex flex-col gap-4 max-w-[30rem] mx-6"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedAccessToken.title"
      label="Titre"
      :variant="hasBeenEdited('title')"
    />

    <AdminInputTextField
      id="token"
      v-model="editedAccessToken.token"
      label="Jeton"
      :variant="hasBeenEdited('token')"
      helper-text="(utilisé dans l'url d'accès après /map/ ou /search/)"
    />

    <AdminInputSwitchField
      id="active"
      v-model="editedAccessToken.active"
      label="Actif"
    />

    <Divider class="!my-2" />

    <AdminInputSwitchField
      id="list_entities"
      v-model="editedAccessToken.permissions.can_list_entities"
      label="Permission de lister les entités"
      helper-text="Comprend le nom, la catégorie, les tags et la date de création de chaque entité"
    />

    <AdminInputSwitchField
      id="access_entities"
      v-model="editedAccessToken.permissions.can_access_entity"
      label="Permission de consulter les entités"
      :disabled="!editedAccessToken.permissions.can_list_entities"
    />

    <AdminInputSwitchField
      id="add_entity"
      v-model="editedAccessToken.permissions.can_add_entity"
      label="Permission d'ajouter une entité"
    />

    <AdminInputSwitchField
      id="access_comments"
      v-model="editedAccessToken.permissions.can_access_comments"
      label="Permission de lister et consulter les commentaires"
      :disabled="!editedAccessToken.permissions.can_access_entity"
    />

    <AdminInputSwitchField
      id="add_comment"
      v-model="editedAccessToken.permissions.can_add_comment"
      label="Permission d'ajouter un commentaire"
      :disabled="!editedAccessToken.permissions.can_list_entities"
    />

    <Divider class="!my-2" />

    <AdminInputPolicyPermissionField
      v-model="editedAccessToken.permissions.families_policy"
      :policy-name="'families_policy'"
      :label="'familles'"
      :options="families"
    />

    <Divider class="!my-2" />

    <AdminInputPolicyPermissionField
      v-model="editedAccessToken.permissions.categories_policy"
      :policy-name="'categories_policy'"
      :label="'catégories'"
      :options="categories"
    />

    <Divider class="!my-2" />

    <AdminInputPolicyPermissionField
      v-model="editedAccessToken.permissions.tags_policy"
      :policy-name="'tags_policy'"
      :label="'tags'"
      :options="tags"
    />

    <Divider class="!my-2" />

    <AdminInputSwitchField
      id="geographic_restriction"
      v-model="geographicRestrictionsOn"
      label="Restreindre géographiquement"
    />
    <span>
      <Button
        v-if="geographicRestrictionsOn"
        label="Définir les restrictions"
        @click="() => polygonDrawer!.show()"
      />
    </span>
    <PolygonDrawer
      v-if="geographicRestrictionsOn"
      ref="polygonDrawer"
      v-model:polygon-list="editedAccessToken.permissions.geographic_restrictions"
      :max-polygons="3"
    />

    <span class="flex gap-1 justify-end">
      <NuxtLink to="/admin/access-tokens">
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
        :disabled="isDisabled()"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateAccessToken, Permissions } from '~/lib'
import state from '~/lib/admin-state'
import { isValidText } from '~/lib/validation'
import type PolygonDrawerComponent from '~/components/PolygonDrawer.vue'

definePageMeta({
  layout: 'admin-ui',
})

const accessTokenId = useRoute().params.id as string
const isNew = (accessTokenId === 'new')

const fetchedAccessToken = isNew ? null : await state.client.getAccessToken(accessTokenId)
const editedAccessToken: Ref<NewOrUpdateAccessToken> = ref(
  isNew
    ? {
        active: true,
        permissions: {
          can_list_entities: false,
          can_access_entity: false,
          can_add_entity: false,
          can_access_comments: false,
          can_add_comment: false,
          categories_policy: {
            allow_all: true,
            allow_list: [],
            force_exclude: [],
          },
          families_policy: {
            allow_all: true,
            allow_list: [],
            force_exclude: [],
          },
          tags_policy: {
            allow_all: true,
            allow_list: [],
            force_exclude: [],
          },
          geographic_restrictions: null,
        },
        title: '',
        token: '',
      }
    : JSON.parse(JSON.stringify(fetchedAccessToken)),
)

const families = state.families
const categories = await state.client.listCategories()
const tags = await state.client.listTags()

const polygonDrawer = ref<InstanceType<typeof PolygonDrawerComponent>>()
const geographicRestrictionsOn = ref(editedAccessToken.value.permissions.geographic_restrictions != null)
watch(geographicRestrictionsOn, (newValue) => {
  if (newValue)
    editedAccessToken.value.permissions.geographic_restrictions = []
  else
    editedAccessToken.value.permissions.geographic_restrictions = null
})

const processingRequest = ref(false)
const toast = useToast()

watch(
  () => editedAccessToken.value.permissions.can_list_entities,
  (newVal) => {
    if (!newVal) {
      editedAccessToken.value.permissions.can_access_entity = false
      editedAccessToken.value.permissions.can_add_comment = false
    }
  },
)

watch(
  () => editedAccessToken.value.permissions.can_access_entity,
  (newVal) => {
    if (!newVal) {
      editedAccessToken.value.permissions.can_access_comments = false
    }
  },
)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  isNew ? `Édition d'un nouveau jeton` : `Édition du jeton ${fetchedAccessToken!.title}`,
  'accessToken',
  [],
  [
    { label: 'Jetons d\'accès', url: '/admin/access-tokens' },
    (
      isNew
        ? { label: `Édition d'un nouveau jeton`, url: `/admin/access-tokens/new` }
        : { label: `Édition du jeton ${fetchedAccessToken!.title}`, url: `/admin/access-tokens/${accessTokenId}` }
    ),
  ],
)

function isDisabled() {
  return processingRequest.value
    || !isValidText(editedAccessToken.value.title)
    || !isValidText(editedAccessToken.value.token)
    || (
      editedAccessToken.value.permissions.geographic_restrictions != null
      && editedAccessToken.value.permissions.geographic_restrictions.length == 0
    )
}

function hasBeenEdited(field: keyof NewOrUpdateAccessToken) {
  return isNew ? false : editedAccessToken.value[field] !== fetchedAccessToken![field]
}

async function onSave() {
  processingRequest.value = true
  try {
    updatePolicies(editedAccessToken.value.permissions)

    if (isNew) {
      await state.client.createAccessToken(editedAccessToken.value)
      toast.add({ severity: 'success', summary: 'Succès', detail: 'Jeton créé avec succès', life: 3000 })
    }
    else {
      await state.client.updateAccessToken(accessTokenId, editedAccessToken.value)
      toast.add({ severity: 'success', summary: 'Succès', detail: 'Jeton modifié avec succès', life: 3000 })
    }

    navigateTo('/admin/access-tokens')
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification du jeton', life: 3000 })
  }
  processingRequest.value = false
}

function updatePolicies(permissions: Permissions) {
  for (const policy of ['families_policy', 'categories_policy', 'tags_policy'] as ('families_policy' | 'categories_policy' | 'tags_policy')[]) {
    if (permissions[policy].allow_all) {
      permissions[policy].allow_list = []
    }
    else {
      permissions[policy].force_exclude = []
    }
  }
}
</script>
