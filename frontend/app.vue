<template>
  <NuxtLoadingIndicator />

  <NuxtLayout>
    <NuxtPage v-if="state.loaded" />
    <NuxtPage v-else>
      <h1 class="text-xl">
        {{ $t('app.apiUnavailable') }}
      </h1>
      <p>{{ $t('app.tryAgainLater') }}</p>
    </NuxtPage>
  </NuxtLayout>

  <Toast />

  <ConfirmPopup group="delete">
    <template #message="slotProps">
      <div class="flex flex-row items-center w-full gap-2 p-4 mb-2 pb-0">
        <AppIcon :icon-name="slotProps.message.icon!" />
        <span>{{ slotProps.message.message }} <b>{{ (slotProps.message as ExtendedConfirmationOptions).objectId }}</b></span>
      </div>
    </template>
  </ConfirmPopup>

  <ConfirmDialog group="confirm_dialog">
    <template #message="slotProps">
      <AppIcon :icon-name="slotProps.message.icon!" />
      <span class="max-w-96">{{ slotProps.message.message }}</span>
    </template>
  </ConfirmDialog>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'
import { useHead } from '#imports'
import state from '~/lib/viewer-state'

useDarkMode().apply()
await state.init()

useHead({
  title: state.title,
  link: [{
    rel: 'icon',
    href: state.logo ?? '/default_favicon.ico',
  }],
})

interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}
</script>
