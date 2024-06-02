<template>
  <form
    class="flex flex-column gap-3 mx-4"
    @submit.prevent="onSave"
  >
    <AdminInputTextField
      id="username"
      v-model="userName"
      label="Nom d'utilisateur⋅ice"
    />

    <AdminInputSwitchField
      id="userIsAdmin"
      v-model="userIsAdmin"
      label="Droits d'administration"
    />

    <div
      class="flex flex-column gap-3"
    >
      <label for="password">
        Nouveau mot de passe :
      </label>
      <Password
        id="password"
        v-model="newPassword"
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
        toggle-mask
        class="-mt-2"
        input-class="w-full"
        :invalid="editPassword && (newPassword!=newPasswordConfirm || !newPassword)"
      />
    </div>

    <span class="flex gap-1 justify-content-end   ">
      <NuxtLink
        to="/admin/user"
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

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Nouvel⋅le utilisateur⋅ice',
  'user',
  [],
  [
    { label: 'Utilisateur⋅ices', url: '/admin/users' },
    { label: 'Nouvel⋅le utilisateur⋅ice', url: '/admin/users/new' },
  ],
)

const userIsAdmin = ref(false)
const userName = ref('')
const editPassword = ref(true)
const newPassword = ref('')
const newPasswordConfirm = ref('')

const processingRequest = ref(false)
const toast = useToast()

async function onSave() {
  processingRequest.value = true
  try {
    const newUser: NewOrUpdatedUser = { is_admin: userIsAdmin.value, name: userName.value, password: newPassword.value }
    state.client.createUser(newUser)
    navigateTo('/admin/user')
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Utilisateur⋅ice modifié⋅e avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification de l\'utilisateur⋅ice', life: 3000 })
  }
  processingRequest.value = false
}
</script>
