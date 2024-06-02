<template>
  <Button
    outlined
    rounded
    class="mx-2"
    severity="warning"
    @click="() => emit('edit', props.id)"
  >
    <template #icon>
      <AppIcon icon-name="edit" />
    </template>
  </Button>

  <Button
    outlined
    rounded
    severity="danger"
    :disabled="preventDelete"
    @click="onDelete"
  >
    <template #icon>
      <AppIcon
        :icon-name="props.loading ? 'loading' : 'delete'"
        :rotating="props.loading"
      />
    </template>
  </Button>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'
import AppIcon from '~/components/AppIcon.vue'

const props = defineProps<{
  modelName: string
  preventDelete?: boolean
  id: string
  name: string
  loading: boolean
}>()

const emit = defineEmits<{
  (e: 'edit', id: string): void
  (e: 'delete', id: string, name: string): void
}>()

const confirm = useConfirm()
interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}

function onDelete(event: Event) {
  const options: ExtendedConfirmationOptions = {
    target: event.currentTarget as HTMLElement,
    group: 'delete',
    message: `Confirmer la suppression de ${props.modelName}`,
    objectId: props.name,
    icon: 'warning',
    rejectClass: 'p-button-secondary p-button-outlined p-button-sm',
    acceptClass: 'p-button-sm',
    rejectLabel: 'Annuler',
    acceptLabel: 'Confirmer',
    reject: () => {},
    accept: () => emit('delete', props.id, props.name),
  }
  confirm.require(options)
}
</script>
