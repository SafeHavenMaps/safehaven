<template>
  <div class="flex align-items-center gap-2">
    <label>Adresses <RequiredIndicator /></label>
    <Button
      size="small"
      outlined
      severity="success"
      label="Ajouter une adresse"
      @click="addNewAddress"
    />
  </div>

  <DataTable :value="props.locations">
    <Column
      field="plain_text"
      header="Adresse"
      sortable
    />
    <Column>
      <template #body="slotProps">
        <div class="flex justify-content-end">
          <Button
            outlined
            rounded
            severity="warning"
            class="mr-1"
            @click="() => editAddress(slotProps.index)"
          >
            <template #icon>
              <AppIcon icon-name="edit" />
            </template>
          </Button>

          <Button
            outlined
            rounded
            severity="danger"
            @click="() => removeAddress(slotProps.index)"
          >
            <template #icon>
              <AppIcon icon-name="delete" />
            </template>
          </Button>
        </div>
      </template>
    </Column>
  </DataTable>

  <Dialog
    v-model:visible="displayNominatimPicker"
    header="Sélectionner une adresse"
    :modal="true"
    :closable="false"
    dismissable-mask
    :style="{ width: '40rem' }"
  >
    <NominatimPicker
      v-model="currentlyEditedLocation!.loc"
      @select="handleAddressSelect"
    />
    <template #footer>
      <Button
        label="Annuler"
        severity="secondary"
        @click="displayNominatimPicker = false"
      />
      <Button
        label="Sélectionner"
        :disabled="!addressSelected"
        @click="applyAddressChanges"
      />
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import type { UnprocessedLocation } from '~/lib'

const props = defineProps<{
  locations: UnprocessedLocation[]
}>()

const emit = defineEmits(['update:locations'])

const toast = useToast()

const currentlyEditedLocation = ref<{ index: number, loc: UnprocessedLocation } | null>(null)
const displayNominatimPicker = ref(false)
const addressSelected = ref(false)

function addNewAddress() {
  currentlyEditedLocation.value = { index: props.locations.length, loc: { plain_text: '', lat: 0, long: 0 } }
  displayNominatimPicker.value = true
  addressSelected.value = false
}

function removeAddress(index: number) {
  const updatedLocations = [...props.locations]
  updatedLocations.splice(index, 1)
  emit('update:locations', updatedLocations)
}

function editAddress(index: number) {
  currentlyEditedLocation.value = { index: index, loc: { ...props.locations[index] } }
  displayNominatimPicker.value = true
  addressSelected.value = false
}

function applyAddressChanges() {
  if (addressSelected.value) {
    if (props.locations.some(loc => loc.plain_text == currentlyEditedLocation.value!.loc.plain_text))
      toast.add({ severity: 'error', summary: 'Erreur', detail: `Addresse déjà présente`, life: 3000 })
    else {
      const updatedLocations = [...props.locations]
      updatedLocations[currentlyEditedLocation.value!.index] = { ...currentlyEditedLocation.value!.loc }
      emit('update:locations', updatedLocations)
    }
    displayNominatimPicker.value = false
  }
}

function handleAddressSelect(location: UnprocessedLocation) {
  currentlyEditedLocation.value!.loc = location
  addressSelected.value = true
}
</script>
