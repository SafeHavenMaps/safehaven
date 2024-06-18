<template>
  <TabView>
    <TabPanel header="Général">
      <form
        class="flex flex-column gap-3 max-w-30rem mx-4"
        @submit.prevent="onSave('general', editedConfig.general)"
      >
        <AdminInputTextField
          id="title"
          v-model="editedConfig.general.title"
          label="Titre"
          :variant="hasBeenEdited('general', 'title')"
        />

        <AdminInputTextField
          id="subtitle"
          v-model="editedConfig.general.subtitle"
          label="Sous-titre"
          :variant="hasBeenEdited('general', 'subtitle')"
        />

        <AdminInputTextField
          id="information"
          v-model="editedConfig.general.information"
          label="Information"
          :variant="hasBeenEdited('general', 'information')"
          text-length="editor"
        />

        <AdminInputTextField
          id="logo_url"
          v-model="editedConfig.general.logo_url"
          label="URL du Logo"
          :variant="hasBeenEdited('general', 'logo_url')"
          :invalid="!editedConfig.general.logo_url || !validator.isURL(editedConfig.general.logo_url)"
        />

        <AdminInputSwitchField
          id="popup_enabled"
          v-model="popupEnabled"
          label="Afficher un popup au lancement de l'application"
        />

        <AdminInputTextField
          v-if="popupEnabled"
          id="popup"
          v-model="editedConfig.general.popup"
          label="Popup d'Accueil"
          :variant="hasBeenEdited('general', 'popup')"
          text-length="editor"
        />

        <AdminInputSwitchField
          v-if="popupEnabled"
          id="popup_check_enabled"
          v-model="popupCheckboxEnabled"
          label="Mettre une case à cocher dans le popup d'accueil"
        />

        <AdminInputTextField
          v-if="popupEnabled && popupCheckboxEnabled"
          id="popup"
          v-model="editedConfig.general.popup_check_text"
          label="Case à Cocher de la Popup d'Accueil"
          :variant="hasBeenEdited('general', 'popup_check_text')"
        />

        <AdminInputSwitchField
          id="redirect_url_enabled"
          v-model="customRedirection"
          label="Rediriger les tokens invalides vers une URL personnalisée"
        />

        <AdminInputTextField
          v-if="customRedirection"
          id="redirect_url"
          v-model="editedConfig.general.redirect_url"
          label="URL de Redirection"
          :variant="hasBeenEdited('general', 'redirect_url')"
          :invalid="!editedConfig.general.redirect_url || !validator.isURL(editedConfig.general.redirect_url)"
        />

        <span class="flex gap-1 justify-content-end">
          <Button
            label="Annuler"
            severity="secondary"
            :disabled="processingRequest"
            @click="onCancel('general')"
          />
          <Button
            v-if="state.is_admin"
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('general')"
          />
          <Button
            v-if="state.is_admin"
            label="Sauvegarder"
            type="submit"
            :disabled="processingRequest"
          />
        </span>
      </form>
    </TabPanel>

    <TabPanel header="Cartographie - Initialisation">
      <form
        class="flex flex-column gap-3 max-w-30rem mx-4"
        @submit.prevent="onSave('cartography_init', editedConfig.cartography_init)"
      >
        <AdminInputNumberField
          id="center_lat"
          v-model="editedConfig.cartography_init.center_lat"
          label="Latitude du Centre"
          :variant="hasBeenEdited('cartography_init', 'center_lat')"
        />

        <AdminInputNumberField
          id="center_lng"
          v-model="editedConfig.cartography_init.center_lng"
          label="Longitude du Centre"
          :variant="hasBeenEdited('cartography_init', 'center_lng')"
        />

        <AdminInputNumberField
          id="zoom"
          v-model="editedConfig.cartography_init.zoom"
          label="Niveau de Zoom"
          :variant="hasBeenEdited('cartography_init', 'zoom')"
        />

        <span class="flex gap-1 justify-content-end">
          <Button
            label="Annuler"
            severity="secondary"
            :disabled="processingRequest"
            @click="onCancel('cartography_init')"
          />
          <Button
            v-if="state.is_admin"
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('cartography_init')"
          />
          <Button
            v-if="state.is_admin"
            label="Sauvegarder"
            type="submit"
            :disabled="processingRequest"
          />
        </span>
      </form>
    </TabPanel>
    <TabPanel header="Cartographie - Cluster">
      <form
        class="flex flex-column gap-3 max-w-30rem mx-4"
        @submit.prevent="onSave('cartography_cluster', editedConfig.cartography_cluster)"
      >
        <AdminInputNumberField
          id="characteristic_distance"
          v-model="editedConfig.cartography_cluster.characteristic_distance"
          label="Distance Caractéristique"
          :variant="hasBeenEdited('cartography_cluster', 'characteristic_distance')"
        />

        <AdminInputNumberField
          id="declustering_speed"
          v-model="editedConfig.cartography_cluster.declustering_speed"
          label="Vitesse de Dégroupement"
          :variant="hasBeenEdited('cartography_cluster', 'declustering_speed')"
        />

        <AdminInputNumberField
          id="minimal_cluster_size"
          v-model="editedConfig.cartography_cluster.minimal_cluster_size"
          label="Taille Minimale du Cluster"
          :variant="hasBeenEdited('cartography_cluster', 'minimal_cluster_size')"
        />

        <span class="flex gap-1 justify-content-end">
          <Button
            label="Annuler"
            severity="secondary"
            :disabled="processingRequest"
            @click="onCancel('cartography_cluster')"
          />
          <Button
            v-if="state.is_admin"
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('cartography_cluster')"
          />
          <Button
            v-if="state.is_admin"
            label="Sauvegarder"
            type="submit"
            :disabled="processingRequest"
          />
        </span>
      </form>
    </TabPanel>
    <TabPanel
      v-if="false"
      header="Mode Sécurisé"
    >
      <form
        class="flex flex-column gap-3 max-w-30rem mx-4"
        @submit.prevent="onSave('safe_mode', editedConfig.safe_mode)"
      >
        <AdminInputTextField
          id="popup_message"
          v-model="editedConfig.safe_mode.popup_message"
          label="Message Pop-up"
          :variant="hasBeenEdited('safe_mode', 'popup_message')"
        />

        <AdminInputTextField
          id="popup_title"
          v-model="editedConfig.safe_mode.popup_title"
          label="Titre Pop-up"
          :variant="hasBeenEdited('safe_mode', 'popup_title')"
        />

        <AdminInputTextField
          id="recaptcha_v3_sitekey"
          v-model="editedConfig.safe_mode.recaptcha_v3_sitekey"
          label="Clé de Site Recaptcha v3"
          :variant="hasBeenEdited('safe_mode', 'recaptcha_v3_sitekey')"
        />

        <span class="flex gap-1 justify-content-end">
          <Button
            label="Annuler"
            severity="secondary"
            :disabled="processingRequest"
            @click="onCancel('safe_mode')"
          />
          <Button
            v-if="state.is_admin"
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('safe_mode')"
          />
          <Button
            v-if="state.is_admin"
            label="Sauvegarder"
            type="submit"
            :disabled="processingRequest"
          />
        </span>
      </form>
    </TabPanel>
  </TabView>
</template>

<script setup lang="ts">
import validator from 'validator'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { ConfigurationOption, SafeHavenOptions } from '~/lib'
import state from '~/lib/admin-state'

await state.fetchConfig()
let fetchedConfig = state.options
const editedConfig: Ref<SafeHavenOptions> = ref(JSON.parse(JSON.stringify(fetchedConfig))) // deep copy

const processingRequest = ref(false)
const toast = useToast()

const popupEnabled = ref(!!editedConfig.value.general.popup)
const popupCheckboxEnabled = ref(!!editedConfig.value.general.popup_check_text)
const customRedirection = ref(!!editedConfig.value.general.redirect_url)

watch(popupEnabled, (value) => {
  if (!value) {
    editedConfig.value.general.popup = null
    popupCheckboxEnabled.value = false
  }
})

watch(popupCheckboxEnabled, (value) => {
  if (!value) {
    editedConfig.value.general.popup_check_text = null
  }
})

watch(customRedirection, (value) => {
  if (!value) {
    editedConfig.value.general.redirect_url = null
  }
})

function hasBeenEdited<T extends keyof SafeHavenOptions>(field: T, subField?: keyof SafeHavenOptions[T]) {
  if (subField) {
    return editedConfig.value[field][subField] !== fetchedConfig[field][subField]
  }
  return editedConfig.value[field] !== fetchedConfig[field]
}

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Édition de la configuration',
  'user',
  [],
  [
    { label: 'Configuration', url: '/admin/configuration' },
  ],
)

async function onSave(name: keyof SafeHavenOptions, config: ConfigurationOption) {
  processingRequest.value = true
  try {
    await state.client.updateConfig(name, config)
    fetchedConfig = state.options
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Groupe d\'options sauvegardé avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de sauvegarde du groupe d\'options', life: 3000 })
  }
  processingRequest.value = false
}

async function onDelete(name: keyof SafeHavenOptions) {
  processingRequest.value = true
  try {
    await state.client.deleteConfig(name)
    fetchedConfig = state.options
    editedConfig.value[name] = JSON.parse(JSON.stringify(fetchedConfig))[name]
    toast.add({ severity: 'success', summary: 'Succès', detail: 'Groupe d\'options réinitialisé avec succès', life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: 'Erreur', detail: 'Erreur de réinitialisation du groupe d\'options', life: 3000 })
  }
  processingRequest.value = false
}

async function onCancel(name: keyof SafeHavenOptions) {
  editedConfig.value[name] = JSON.parse(JSON.stringify(fetchedConfig))[name]
}
</script>
