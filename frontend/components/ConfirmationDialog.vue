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
import type { ConfirmationOptions } from 'primevue/confirmationoptions'
import AppIcon from '~/components/AppIcon.vue'

const props = withDefaults(defineProps<{
  message?: string
  objectDescriptor?: string
  icon?: string
  rejectLabel?: string
  acceptLabel?: string
}>(),
{
  message: 'Confirmer la suppression de ',
  messageObjectDescriptor: '',
  icon: 'warning',
  rejectLabel: 'Annuler',
  acceptLabel: 'Confirmer',
})

const confirm = useConfirm()
interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}

function displayDialog(event: Event, messageObjectId: string, onAccept?: () => void, onReject?: () => void) {
  // if id==
  const options: ExtendedConfirmationOptions = {
    target: event.currentTarget as HTMLElement,
    group: 'delete',
    message: `${props.message}${props.messageObjectDescriptor}`,
    objectId: messageObjectId,
    icon: props.icon,
    rejectClass: 'p-button-secondary p-button-outlined p-button-sm',
    acceptClass: 'p-button-sm',
    rejectLabel: props.rejectLabel,
    acceptLabel: props.acceptLabel,
    reject: onReject,
    accept: onAccept,
  }
  confirm.require(options)
}

defineExpose({ displayDialog })
</script>
