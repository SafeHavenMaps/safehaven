<template>
  <div class="h-full">
    <AdminNavbar />
    <div
      class="h-full flex"
    >
      <div
        class="h-full flex"
      >
        <AdminSidebar />
      </div>
      <div
        class="h-full flex"
      >
        <slot />
      </div>
    </div>
    <ConfirmPopup group="delete">
      <template #message="slotProps">
        <div class="flex flex-row align-items-center w-full gap-2 p-3 mb-2 pb-0">
          <AppIcon
            :icon-name="slotProps.message.icon!"
          />
          <span>{{ slotProps.message.message }} <b>{{ (slotProps.message as ExtendedConfirmationOptions).objectId }}</b></span>
        </div>
      </template>
    </ConfirmPopup>
  </div>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'

interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}
</script>
