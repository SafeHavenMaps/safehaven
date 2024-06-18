<template>
  <form
    class="flex flex-col gap-4 max-w-[30rem] mx-6"
    @submit.prevent="onSave"
  >
    <span> Nom d'utilisateur⋅ice: {{ state.username }} </span>

    <span> Statut: {{ state.is_admin ? 'Administrateur⋅ice' : 'Modérateur⋅ice' }} </span>

    <AdminInputSwitchField
      id="editPassword"
      v-model="editPassword"
      label="Écraser l'ancien mot de passe"
    />

    <div
      :hidden="!editPassword"
      class="flex-col gap-4"
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

    <span class="flex gap-1 justify-end   ">
      <NuxtLink
        to="/admin/"
      >
        <Button
          label="Revenir à l'accueil"
          severity="secondary"
          :loading="processingRequest"
          :disabled="processingRequest"
        />
      </NuxtLink>
      <Button
        v-if="editPassword"
        label="Sauvegarder"
        type="submit"
        :loading="processingRequest"
        :disabled="processingRequest || (newPassword!=newPasswordConfirm || !newPassword)"
      />
    </span>
  </form>
</template>

<script setup lang="ts">
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import state from '~/lib/admin-state'

const editPassword = ref(false)
const newPassword = ref('')
const newPasswordConfirm = ref('')

const processingRequest = ref(false)
const toast = useToast()

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
    `Profil`,
    'user',
    [],
    [
      { label: `Profil`, url: `/admin/users/self` },
    ],
)

async function onSave() {
  processingRequest.value = true
  try {
    await state.client.changeSelfPassword({ is_admin: state.is_admin!, name: state.username!, password: newPassword.value })
    editPassword.value = false
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Mot de passe mis à jour avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de modification de mise à jour du mot de passe ', life: 3000 })
  }
  processingRequest.value = false
}
</script>
