<template>
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
</template>

<script setup lang="ts">

</script>
