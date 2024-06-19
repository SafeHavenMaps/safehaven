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
      :invalid="!editedAccessToken.title"
    />

    <AdminInputTextField
      id="token"
      v-model="editedAccessToken.token"
      label="Jeton"
      :variant="hasBeenEdited('token')"
      helper-text="(utilisé dans l'url d'accès après /map/ ou /search/)"
      :invalid="!editedAccessToken.token"
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
          can_access_comments: false,
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
        },
        title: '',
        token: '',
      }
    : JSON.parse(JSON.stringify(fetchedAccessToken)),
)

const families = state.families
const categories = await state.client.listCategories()
const tags = await state.client.listTags()

const processingRequest = ref(false)
const toast = useToast()

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
    || !editedAccessToken.value.title
    || !editedAccessToken.value.token
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
