<template>
  <Button
    v-if="!props.editAbsent"
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

  <Dialog
    v-if="props.secureDelete"
    v-model:visible="secureDeleteDialogVisible"
    modal
    dismissable-mask
    :closable="false"
    :header="`Suppression ${props.modelName} ${props.name}`"
    :style="{ width: '25rem' }"
  >
    <span class="p-text-secondary block mb-3 -mt-2">
      <p>La suppression {{ props.modelName }} {{ props.name }}
        entraînera <b>la suppression définitive de {{ props.secureDeleteEntityCount }} entités</b>.</p>
      <p>Cette action ne pourra pas être annulée. Pour confirmer, veuillez entrer le nom {{ props.modelName }} ci-dessous :</p>
    </span>
    <InputText
      id="repeatName"
      v-model="repeatName"
      class="mb-4"
      autocomplete="off"
    />
    <div class="flex justify-content-end gap-2">
      <Button
        type="button"
        label="Annuler"
        severity="secondary"
        @click="secureDeleteDialogVisible = false"
      />
      <Button
        :disabled="repeatName.toUpperCase() != props.name.toUpperCase()"
        type="button"
        label="Confirmer"
        @click="() => { secureDeleteDialogVisible = false, emit('delete', props.id, props.name) }"
      />
    </div>
  </Dialog>
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
  secureDelete?: boolean
  secureDeleteEntityCount?: number
  editAbsent?: boolean
}>()

const emit = defineEmits<{
  (e: 'edit', id: string): void
  (e: 'delete', id: string, name: string): void
}>()

const confirm = useConfirm()
interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}

const secureDeleteDialogVisible = ref(false)
const repeatName = ref('')

function onDelete(event: Event) {
  if (props.secureDelete) {
    secureDeleteDialogVisible.value = true
  }
  else {
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
}
</script>
