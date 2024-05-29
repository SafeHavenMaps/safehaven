<template>
  <form
    class="flex flex-column gap-3 max-w-30rem mx-4"
    @submit.prevent="onSave"
  >
    <label for="username" />

    <AdminInputTextField
      id="username"
      v-model="userName"
      label="Nom d'utilisateur⋅ice"
      :variant="userName == name"
      :invalid="!userName"
    />

    <AdminInputSwitchField
      id="userIsAdmin"
      v-model="userIsAdmin"
      label="Droits d'administration"
      :disabled="state.username == name"
    />

    <AdminInputSwitchField
      id="editPassword"
      v-model="editPassword"
      label="Écraser l'ancien mot de passe"
    />

    <div
      :hidden="!editPassword"
      class="flex-column gap-3"
      :class="{ flex: editPassword }"
    >
      <label for="password">
        Nouveau mot de passe :
      </label>
      <Password
        id="password"
        v-model="newPassword"
        :disabled="!editPassword"
        toggle-mask
        class=" -mt-2"
        input-class="w-full"
        :invalid="editPassword && (newPassword!=newPasswordConfirm || !newPassword)"
      />
      <label for="password">
        Confirmer le nouveau mot de passe :
      </label>
      <Password
        id="passwordConfirm"
        v-model="newPasswordConfirm"
        :disabled="!editPassword"
        toggle-mask
        class="-mt-2"
        input-class="w-full"
        :invalid="editPassword && (newPassword!=newPasswordConfirm || !newPassword)"
      />
    </div>

    <span class="flex gap-1 justify-content-end   ">
      <NuxtLink
        to="/admin/users"
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
        :disabled="processingRequest || editPassword && (newPassword!=newPasswordConfirm || !newPassword) || !userName"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdatedUser } from '~/lib'
import state from '~/lib/admin-state'

const userId = useRoute().params.id as string

const { is_admin, name } = await state.client.getUser(userId)
const userIsAdmin = ref(is_admin)
const userName = ref(name)
const editPassword = ref(false)
const newPassword = ref('')
const newPasswordConfirm = ref('')
const processingRequest = ref(false)

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  `Édition de l'utilisateur⋅ice ${name}`,
  'user',
  [],
  [
    { label: 'Utilisateur⋅ices', url: '/admin/users' },
    { label: `Édition de l'utilisateur⋅ice ${name}`, url: `/admin/users/${userId}` },
  ],
)

async function onSave() {
  processingRequest.value = true
  const newUser: NewOrUpdatedUser = { is_admin: userIsAdmin.value, name: userName.value }
  if (editPassword.value) {
    if ((newPassword.value != newPasswordConfirm.value || !newPassword.value))
      throw new Error('Empty or non-matching password')
    newUser['password'] = newPassword.value
  }
  await state.client.updateUser(userId, newUser)
  if (state.username == name) {
    state.logout()
  }
  navigateTo('/admin/users')
}
</script>
