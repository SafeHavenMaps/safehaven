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
          :invalid="!editedConfig.general.title"
        />

        <AdminInputTextField
          id="subtitle"
          v-model="editedConfig.general.subtitle"
          label="Sous-titre"
          :variant="hasBeenEdited('general', 'subtitle')"
          :invalid="!editedConfig.general.subtitle"
        />

        <AdminInputTextField
          id="information"
          v-model="editedConfig.general.information"
          label="Information"
          :variant="hasBeenEdited('general', 'information')"
          :invalid="!editedConfig.general.information"
        />

        <AdminInputTextField
          id="logo_url"
          v-model="editedConfig.general.logo_url"
          label="URL du Logo"
          :variant="hasBeenEdited('general', 'logo_url')"
          :invalid="!editedConfig.general.logo_url || !validator.isURL(editedConfig.general.logo_url)"
        />

        <span class="flex gap-1 justify-content-end">
          <NuxtLink to="/admin/tags">
            <Button
              label="Annuler"
              severity="secondary"
              :disabled="processingRequest"
            />
          </NuxtLink>
          <Button
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('general')"
          />
          <Button
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
          :invalid="!editedConfig.cartography_cluster.characteristic_distance"
        />

        <AdminInputNumberField
          id="declustering_speed"
          v-model="editedConfig.cartography_cluster.declustering_speed"
          label="Vitesse de Dégroupement"
          :variant="hasBeenEdited('cartography_cluster', 'declustering_speed')"
          :invalid="!editedConfig.cartography_cluster.declustering_speed"
        />

        <AdminInputNumberField
          id="minimal_cluster_size"
          v-model="editedConfig.cartography_cluster.minimal_cluster_size"
          label="Taille Minimale du Cluster"
          :variant="hasBeenEdited('cartography_cluster', 'minimal_cluster_size')"
          :invalid="!editedConfig.cartography_cluster.minimal_cluster_size"
        />

        <span class="flex gap-1 justify-content-end">
          <NuxtLink to="/admin/tags">
            <Button
              label="Annuler"
              severity="secondary"
              :disabled="processingRequest"
            />
          </NuxtLink>
          <Button
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('cartography_cluster')"
          />
          <Button
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
          :invalid="!editedConfig.cartography_init.center_lat"
        />

        <AdminInputNumberField
          id="center_lng"
          v-model="editedConfig.cartography_init.center_lng"
          label="Longitude du Centre"
          :variant="hasBeenEdited('cartography_init', 'center_lng')"
          :invalid="!editedConfig.cartography_init.center_lng"
        />

        <AdminInputNumberField
          id="zoom"
          v-model="editedConfig.cartography_init.zoom"
          label="Niveau de Zoom"
          :variant="hasBeenEdited('cartography_init', 'zoom')"
          :invalid="!editedConfig.cartography_init.zoom"
        />

        <span class="flex gap-1 justify-content-end">
          <NuxtLink to="/admin/tags">
            <Button
              label="Annuler"
              severity="secondary"
              :disabled="processingRequest"
            />
          </NuxtLink>
          <Button
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('cartography_init')"
          />
          <Button
            label="Sauvegarder"
            type="submit"
            :disabled="processingRequest"
          />
        </span>
      </form>
    </TabPanel>
    <TabPanel header="Mode Sécurisé">
      <form
        class="flex flex-column gap-3 max-w-30rem mx-4"
        @submit.prevent="onSave('safe_mode', editedConfig.safe_mode)"
      >
        <AdminInputTextField
          id="popup_message"
          v-model="editedConfig.safe_mode.popup_message"
          label="Message Pop-up"
          :variant="hasBeenEdited('safe_mode', 'popup_message')"
          :invalid="!editedConfig.safe_mode.popup_message"
        />

        <AdminInputTextField
          id="popup_title"
          v-model="editedConfig.safe_mode.popup_title"
          label="Titre Pop-up"
          :variant="hasBeenEdited('safe_mode', 'popup_title')"
          :invalid="!editedConfig.safe_mode.popup_title"
        />

        <AdminInputTextField
          id="recaptcha_v3_sitekey"
          v-model="editedConfig.safe_mode.recaptcha_v3_sitekey"
          label="Clé de Site Recaptcha v3"
          :variant="hasBeenEdited('safe_mode', 'recaptcha_v3_sitekey')"
          :invalid="!editedConfig.safe_mode.recaptcha_v3_sitekey"
        />

        <span class="flex gap-1 justify-content-end">
          <NuxtLink to="/admin/tags">
            <Button
              label="Annuler"
              severity="secondary"
              :disabled="processingRequest"
            />
          </NuxtLink>
          <Button
            label="Réinitialiser"
            :disabled="processingRequest"
            @click="onDelete('safe_mode')"
          />
          <Button
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
  console.log(name, config)
  await state.client.updateConfig(name, config)
  fetchedConfig = await state.options
  console.log(name, fetchedConfig)
  processingRequest.value = false
}

async function onDelete(name: keyof SafeHavenOptions) {
  processingRequest.value = true
  console.log(name)
  await state.client.deleteConfig(name)
  fetchedConfig = await state.options
  console.log(name, fetchedConfig)
  processingRequest.value = false
}
</script>
