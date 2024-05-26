<template>
  <TabView>
    <TabPanel
      :header="state.title"
    >
      <!-- eslint-disable-next-line vue/no-v-html -->
      <div v-html="siteInfo" />
    </TabPanel>
    <TabPanel header="SafeHaven">
      <div class="flex align-items-center justify-content-center">
        <div class="logo hot-pink-bg p-3 flex align-items-center justify-content-center">
          <img
            src="assets/logo_secondary.svg"
            alt="SafeHaven"
          >
        </div>
      </div>
      <p class="text-center">
        SafeHaven est une solution opensource de création de carte collaborative.<br>Vous pouvez facilement créer votre carte !
      </p>
      <p class="text-center">
        <a
          :href="safeHavenRepoUrl"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Button
            label="Découvrir sur GitHub"
          >
            <template #icon>
              <AppIcon icon-name="github" />
            </template>
          </Button>
        </a>
      </p>
    </TabPanel>
  </TabView>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'

const siteInfo = await state.getSanitizedInformation()
const safeHavenRepoUrl = 'https://github.com/SafeHavenMaps/safehaven'
</script>

<style scoped>
.logo {
    width: 220px;
    border-radius: 2rem;
}
</style>
