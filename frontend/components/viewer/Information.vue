<!-- eslint-disable vue/no-v-html -->

<template>
  <Tabs value="0">
    <TabList>
      <Tab value="0">
        {{ state.title }}
      </Tab>
      <Tab value="1">
        {{ $t('cmp.viewer.information.safeHaven') }}
      </Tab>
    </TabList>

    <TabPanels>
      <TabPanel value="0">
        <div
          class="rich-text-content"
          v-html="siteInfo"
        />
      </TabPanel>

      <TabPanel value="1">
        <div class="flex items-center justify-center">
          <div class="logo hot-pink-bg p-4 flex items-center justify-center">
            <img
              src="assets/logo_secondary.svg"
              alt="SafeHaven"
            >
          </div>
        </div>

        <p class="text-center !m-4">
          {{ $t('cmp.viewer.information.safeHavenDescription') }}
          <br>
          {{ $t('cmp.viewer.information.safeHavenDescriptionPart2') }}
        </p>

        <p class="text-center !m-4">
          <a
            :href="safeHavenRepoUrl"
            target="_blank"
            rel="noopener noreferrer"
          >
            <Button
              :label="$t('cmp.viewer.information.discoverOnGitHub')"
            >
              <template #icon>
                <AppIcon icon-name="github" />
              </template>
            </Button>
          </a>
        </p>
      </TabPanel>
    </TabPanels>
  </Tabs>
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
