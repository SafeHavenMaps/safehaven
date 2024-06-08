<!-- eslint-disable vue/no-v-html -->
<template>
  <Dialog
    v-if="popupData?.sanitizedContent"
    v-model:visible="visible"
    modal
    :header="popupData.siteTitle + ` : Informations`"
    :closable="false"
    :draggable="false"
    :style="{ width: '50rem' }"
    :breakpoints="{ '1199px': '75vw', '575px': '90vw' }"
  >
    <div v-html="popupData.sanitizedContent" />

    <div v-if="popupData?.sanitizedCheckbox">
      <span class="flex align-items-center gap-2">
        <InputSwitch
          v-model="validationCheckbox"
          input-id="validated_popup"
        />
        <label
          for="validated_popup"
          v-html="popupData.sanitizedCheckbox"
        />
      </span>
    </div>

    <template #footer>
      <Button
        label="Valider"
        text
        severity="primary"
        autofocus
        :disabled="!canBeClicked()"
        @click="validatePopup"
      />
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'

const popupData = state.getSanitizedPopup!
const visible = ref(false)
const validationCheckbox = ref(false)

function canBeClicked() {
  return !popupData?.sanitizedCheckbox || validationCheckbox.value
}

const pastValidationRaw = localStorage.getItem('validatedPopup')
const pastValidation = pastValidationRaw === 'true'

if (popupData?.sanitizedContent && !pastValidation) {
  visible.value = true
}

function validatePopup() {
  localStorage.setItem('validatedPopup', 'true')
  visible.value = false
}
</script>
