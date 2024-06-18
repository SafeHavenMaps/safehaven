<template>
  <form
    class="flex flex-col gap-4 max-w-[30rem] mx-6"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="username"
      v-model="user.name"
      label="Nom d'utilisateur⋅ice"
    />

    <AdminInputSwitchField
      id="userIsAdmin"
      v-model="user.is_admin"
      label="Droits d'administration"
      :disabled="state.username == user.name"
    />

    <AdminInputSwitchField
      v-if="!isNew"
      id="editPassword"
      v-model="editPassword"
      label="Écraser l'ancien mot de passe"
    />

    <div
      :hidden="!editPassword && !isNew"
      class="flex-col gap-4"
      :class="{ flex: isNew || editPassword }"
    >
      <label for="password">
        Nouveau mot de passe :
      </label>
      <Password
        id="password"
        v-model="newPassword"
        :disabled="!isNew && !editPassword"
        toggle-mask
        class="-mt-2"
        input-class="w-full"
        :invalid="editPassword && (newPassword!=newPasswordConfirm || !newPassword)"
      />
      <label for="password">
        Confirmer le nouveau mot de passe :
      </label>
      <Password
        id="passwordConfirm"
        v-model="newPasswordConfirm"
        :disabled="!isNew && !editPassword"
        toggle-mask
        class="-mt-2"
        input-class="w-full"
        :invalid="editPassword && (newPassword!=newPasswordConfirm || !newPassword)"
      />
    </div>

    <span class="flex gap-1 justify-end   ">
      <NuxtLink
        to="/admin/users"
      >
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
import type { NewOrUpdatedUser } from '~/lib'
import state from '~/lib/admin-state'

const userId = useRoute().params.id as string

const isNew = (userId === 'new')

const user = ref(isNew
  ? { name: '', is_admin: false }
  : await state.client.getUser(userId),
)

const editPassword = ref(false)
const newPassword = ref('')
const newPasswordConfirm = ref('')

const processingRequest = ref(false)
const toast = useToast()

definePageMeta({
  layout: 'admin-ui',
})

function isDisabled() {
  return processingRequest.value
    || (editPassword.value && (newPassword.value != newPasswordConfirm.value || !newPassword.value))
    || !user.value.name
}

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  isNew
    ? 'Nouvel⋅le utilisateur⋅ice'
    : `Édition de l'utilisateur⋅ice ${user.value.name}`,
  'user',
  [],
  [
    { label: 'Utilisateur⋅ices', url: '/admin/users' },
    (isNew
      ? { label: 'Nouvel⋅le utilisateur⋅ice', url: '/admin/users/new' }
      : { label: `Édition de l'utilisateur⋅ice ${user.value.name}`, url: `/admin/users/${userId}` }
    ),
  ],
)

async function onSave() {
  processingRequest.value = true
  try {
    const newUser: NewOrUpdatedUser = {
      is_admin: user.value.is_admin, name: user.value.name }

    if (editPassword.value || isNew) {
      if ((newPassword.value != newPasswordConfirm.value || !newPassword.value))
        throw new Error('Empty or non-matching password')

      newUser['password'] = newPassword.value
    }

    if (isNew) {
      state.client.createUser(newUser)
      navigateTo('/admin/users')
      toast.add({
        severity: 'success',
        summary: 'Succès',
        detail: 'Utilisateur⋅ice modifié⋅e avec succès',
        life: 3000,
      })
    }
    else {
      await state.client.updateUser(userId, newUser)

      if (state.username == user.value.name) {
        state.logout()
      }

      navigateTo('/admin/users')
      toast.add({
        severity: 'success',
        summary: 'Succès',
        detail: 'Utilisateur⋅ice modifié⋅e avec succès',
        life: 3000,
      })
    }
  }
  catch {
    toast.add({
      severity: 'error',
      summary: 'Erreur',
      detail: 'Erreur de modification de l\'utilisateur⋅ice',
      life: 3000,
    })
  }
  processingRequest.value = false
}
</script>
