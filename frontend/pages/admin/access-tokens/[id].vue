<template>
  <div class="mx-4">
    <form
      class="flex flex-column gap-3 max-w-30rem"
      @submit.prevent="onSave"
    >
      <div class="flex flex-column gap-2">
        <label for="title">
          Titre
        </label>
        <InputText
          id="title"
          v-model="editedAccessToken.title"
          :variant="editedAccessToken.title == fetchedAccessToken.title ? 'outlined' : 'filled'"
          :invalid="!editedAccessToken.title"
        />
      </div>

      <div class="flex flex-column gap-2">
        <label for="token">
          Jeton
        </label>
        <InputText
          id="token"
          v-model="editedAccessToken.token"
          :variant="editedAccessToken.token == fetchedAccessToken.token ? 'outlined' : 'filled'"
          :invalid="!editedAccessToken.token"
        />
        <small>(utilisé dans l'url d'accès après /map/ ou /search/)</small>
      </div>

      <span class="flex align-items-center gap-2">
        <InputSwitch
          v-model="editedAccessToken.active"
          input-id="active"
        />
        <label for="active">
          Actif
        </label>
      </span>

      <span class="flex align-items-center gap-2">
        <InputSwitch
          v-model="editedAccessToken.permissions.can_access_comments"
          input-id="comments"
        />
        <label for="comments">
          Permission d'accès aux commentaires
        </label>
      </span>

      <Divider />

      <div class="flex flex-column gap-3">
        <span class="flex align-items-center gap-2">
          <InputSwitch
            v-model="editedAccessToken.permissions.families_policy.allow_all"
            input-id="families_policy.allow_all"
          />
          <label for="families_policy.allow_all">
            Accès à toutes les familles par défaut
          </label>
        </span>
        <div
          v-if="editedAccessToken.permissions.families_policy.allow_all"
          class="flex flex-column gap-2"
        >
          <label for="families_policy.force_exclude">
            Sélectionner les familles à exclure (le cas écheant)
          </label>
          <MultiSelect
            v-model="editedAccessToken.permissions.families_policy.force_exclude"
            :options="families"
            option-label="title"
            option-value="id"
            input-id="families_policy.force_exclude"
          />
        </div>
        <div
          v-if="!editedAccessToken.permissions.families_policy.allow_all"
          class="flex flex-column gap-2"
        >
          <label for="families_policy.allow_list">
            Sélectionner les familles à inclure
          </label>
          <MultiSelect
            v-model="editedAccessToken.permissions.families_policy.allow_list"
            :options="families"
            option-label="title"
            option-value="id"
            input-id="families_policy.allow_list"
          />
        </div>
      </div>

      <Divider />

      <div class="flex flex-column gap-3">
        <span class="flex align-items-center gap-2">
          <InputSwitch
            v-model="editedAccessToken.permissions.categories_policy.allow_all"
            input-id="categories_policy.allow_all"
          />
          <label for="categories_policy.allow_all">
            Accès à toutes les catégories par défaut
          </label>
        </span>
        <div
          v-if="editedAccessToken.permissions.categories_policy.allow_all"
          class="flex flex-column gap-2"
        >
          <label for="categories_policy.force_exclude">
            Sélectionner les catégories à exclure (le cas écheant)
          </label>
          <MultiSelect
            v-model="editedAccessToken.permissions.categories_policy.force_exclude"
            :options="categories"
            option-label="title"
            option-value="id"
            input-id="categories_policy.force_exclude"
          />
        </div>
        <div
          v-if="!editedAccessToken.permissions.categories_policy.allow_all"
          class="flex flex-column gap-2"
        >
          <label for="categories_policy.allow_list">
            Sélectionner les catégories à inclure
          </label>
          <MultiSelect
            v-model="editedAccessToken.permissions.categories_policy.allow_list"
            :options="categories"
            option-label="title"
            option-value="id"
            input-id="categories_policy.allow_list"
          />
        </div>
      </div>

      <Divider />

      <div class="flex flex-column gap-3">
        <span class="flex align-items-center gap-2">
          <InputSwitch
            v-model="editedAccessToken.permissions.tags_policy.allow_all"
            input-id="tags_policy.allow_all"
          />
          <label for="tags_policy.allow_all">
            Accès à toutes les tags par défaut
          </label>
        </span>
        <div
          v-if="editedAccessToken.permissions.tags_policy.allow_all"
          class="flex flex-column gap-2"
        >
          <label for="tags_policy.force_exclude">
            Sélectionner les tags à exclure (le cas écheant)
          </label>
          <MultiSelect
            v-model="editedAccessToken.permissions.tags_policy.force_exclude"
            :options="tags"
            option-label="title"
            option-value="id"
            input-id="tags_policy.force_exclude"
          />
        </div>
        <div
          v-if="!editedAccessToken.permissions.tags_policy.allow_all"
          class="flex flex-column gap-2"
        >
          <label for="tags_policy.allow_list">
            Sélectionner les tags à inclure
          </label>
          <MultiSelect
            v-model="editedAccessToken.permissions.tags_policy.allow_list"
            :options="tags"
            option-label="title"
            option-value="id"
            input-id="tags_policy.allow_list"
          />
        </div>
      </div>

      <span class="flex gap-1 justify-content-end   ">
        <NuxtLink
          to="/admin/access-tokens"
        >
          <Button
            label="Annuler"
            severity="secondary"
            :disabled="processingRequest"
          />
        </NuxtLink>
        <Button
          label="Sauvegarder"
          type="submit"
          :disabled="processingRequest"
        />
      </span>
    </form>
  </div>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdateAccessToken } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const accessTokenId = useRoute().params.id as string

// TODO : Avoid blocking to define the initial values
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

async function onSave() {
  processingRequest.value = true
  const savedAccessToken = editedAccessToken.value
  state.client.updateAccessToken(accessTokenId, savedAccessToken)
  navigateTo('/admin/access-tokens')
}
</script>
