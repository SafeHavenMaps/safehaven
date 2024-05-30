<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="title"
      v-model="editedAccessToken.title"
      label="Titre"
      :variant="hasBeenEdited('title')"
      :invalid="!editedAccessToken.title"
    />
    <AdminInputTextField
      id="token"
      v-model="editedAccessToken.token"
      label="Jeton"
      :variant="hasBeenEdited('token')"
      :invalid="!editedAccessToken.token"
      helper-text="(utilisé dans l'url d'accès après /map/ ou /search/)"
    />

    <AdminInputSwitchField
      id="active"
      v-model="editedAccessToken.active"
      label="Actif"
    />

    <AdminInputSwitchField
      id="comments"
      v-model="editedAccessToken.permissions.can_access_comments"
      label="Permission d'accès aux commentaires"
    />

    <Divider />

    <AdminInputPolicyPermissionField
      v-model="editedAccessToken.permissions.families_policy"
      :policy-name="'families_policy'"
      :label="'familles'"
      :options="families"
    />

    <Divider />

    <AdminInputPolicyPermissionField
      v-model="editedAccessToken.permissions.categories_policy"
      :policy-name="'categories_policy'"
      :label="'catégories'"
      :options="categories"
    />

    <Divider />

    <AdminInputPolicyPermissionField
      v-model="editedAccessToken.permissions.tags_policy"
      :policy-name="'tags_policy'"
      :label="'tags'"
      :options="tags"
    />

    <span class="flex gap-1 justify-content-end">
      <NuxtLink to="/admin/access-tokens">
        <Button
          label="Annuler"
          severity="secondary"
          :disabled="processingRequest"
        />
      </NuxtLink>
      <Button
        label="Sauvegarder"
        type="submit"
        :disabled="processingRequest || !editedAccessToken.title || !editedAccessToken.token"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateAccessToken, Permissions } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const accessTokenId = useRoute().params.id as string

const fetchedAccessToken = await state.client.getAccessToken(accessTokenId)
const editedAccessToken: Ref<NewOrUpdateAccessToken> = ref(JSON.parse(JSON.stringify(fetchedAccessToken))) // deep copy

const families = state.families
const categories = await state.client.listCategories()
const tags = await state.client.listTags()

const processingRequest = ref(false)

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition du jeton ${fetchedAccessToken.title}`,
  'accessToken',
  [],
  [
    { label: 'Jetons d\'accès', url: '/admin/access-tokens' },
    { label: `Édition du jeton ${fetchedAccessToken.title}`, url: `/admin/access-tokens/${accessTokenId}` },
  ],
)

function hasBeenEdited(field: keyof NewOrUpdateAccessToken) {
  return editedAccessToken.value[field] !== fetchedAccessToken[field]
}

async function onSave() {
  processingRequest.value = true
  updatePolicies(editedAccessToken.value.permissions)
  await state.client.updateAccessToken(accessTokenId, editedAccessToken.value)
  navigateTo('/admin/access-tokens')
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
