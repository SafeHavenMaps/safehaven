<template>
  <div class="mx-4">
    <h4>
      Édition de l'utilisateur⋅ice {{ userId }}
    </h4>

    <form
      class="flex flex-column gap-3"
      @submit.prevent="onSave"
    >
      <label for="username">
        Nom d'utilisateur⋅ice
      </label>
      <InputText
        id="username"
        v-model="userName"
        class="-mt-2"
        :variant="userName == name ? 'outlined' : 'filled'"
        :invalid="!userName"
      />

      <span class="flex align-items-center gap-2">

        <InputSwitch
          v-model="userIsAdmin"
          input-id="userIsAdmin"
          :disabled="state.username == name"
        />
        <label for="userIsAdmin">
          Droits d'administration
        </label>
      </span>

      <span class="flex align-items-center gap-2">
        <InputSwitch
          v-model="editPassword"
          input-id="editPassword"
        />
        <label for="editPassword">
          Écraser l'ancien mot de passe
        </label>
      </span>

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
  </div>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { NewOrUpdatedUser } from '~/lib'
import state from '~/lib/admin-state'

const userId = useRoute().params.id as string

const { is_admin, name } = await state.getUser(userId)
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
  `Édition de ${name}`,
  'user',
  [],
  [
    { label: 'Utilisateur⋅ices', url: '/admin/users' },
    { label: `Édition de ${name}`, url: `/admin/users/${userId}` },
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
  await state.updateUser(userId, newUser)
  if (state.username == name) {
    state.logout()
  }
  navigateTo('/admin/users')
}
</script>
