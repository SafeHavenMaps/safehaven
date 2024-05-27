<template>
  <div class="mx-4">
    <h4>
      Nouvel⋅le utilisateur⋅ice
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
        :invalid="!userName"
      />

      <span class="flex align-items-center gap-2">

        <InputSwitch
          v-model="userIsAdmin"
          input-id="userIsAdmin"
        />
        <label for="userIsAdmin">
          Droits d'administration
        </label>
      </span>

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
  </div>
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
  `Édition du jeton d'accès`,
  'accessToken',
  [],
  [
    { label: 'Jeton d\'accès', url: '/admin/access-tokens' },
    { label: `Nouveau jeton`, url: '/admin/access-tokens/new' },
  ],
)

const userIsAdmin = ref(false)
const userName = ref('')
const editPassword = ref(true)
const newPassword = ref('')
const newPasswordConfirm = ref('')
const processingRequest = ref(false)

async function onSave() {
  processingRequest.value = true
  const newUser: NewOrUpdatedUser = { is_admin: userIsAdmin.value, name: userName.value, password: newPassword.value }
  state.createUser(newUser)
  navigateTo('/admin/user')
}
</script>
