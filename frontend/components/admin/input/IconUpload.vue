<template>
  <span
    class="flex gap-2 items-center"
  >
    <Button
      label="Modifier l'icône"
      @click="visible = true"
    />
    <small>(.png, .svg, .bmp, .jpg, .jpeg, .ico, ...)</small>
  </span>
  <Dialog
    v-model:visible="visible"
    modal
    header="Modifier l'icône"
    class="w-[30rem]"
  >
    <span class="flex gap-2 items-center">
      <div class="flex flex-col gap-1 items-center">
        <FileUpload
          mode="basic"
          name="file"
          :url="uploadToUrl"
          accept="image/*"
          auto
          choose-label="Upload"
          @upload="() => {
            toast.add({ severity: 'success', summary: 'Success', detail: 'Image uploaded successfully!', life: 3000 });
            visible = false
          }"
          @error="() => toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to upload image.', life: 3000 })"
        />
        <small>Télecharger depuis vos fichier locaux </small>
      </div>
      <Divider
        layout="vertical"
        class="hidden md:flex"
      >
        <b>OU</b>
      </Divider>
      <div class="flex flex-col gap-1 items-center">
        <InputText
          v-model="downloadFromUrl"
          placeholder="http://url.com/icon.svg"
          :invalid="!isValidUrl(downloadFromUrl)"
        />
        <Button
          label="Sauvegarder"
          :disabled="!isValidUrl(downloadFromUrl)"
          @click="downloadImage"
        />
        <small>Télecharger depuis une url </small>
      </div>
    </span>
  </Dialog>
</template>

<script setup lang="ts">
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import { isValidUrl } from '~/lib/validation'

const toast = useToast()
const props = defineProps<{
  objectId: string
  objectType: 'categories' | 'families'
}>()

const visible = ref(false)
const uploadToUrl = `/api/admin/${props.objectType}/${props.objectId}/icon`
const downloadFromUrl = ref('')

const downloadImage = async () => {
  try {
    const response = await fetch(downloadFromUrl.value)
    if (!response.ok) {
      throw new Error('Failed to download image.')
    }
    const blob: Blob = await response.blob()
    const file: File = new File([blob], 'downloaded-image', { type: blob.type })
    uploadFile(file)
    visible.value = false
  }
  catch (error) {
    toast.add({ severity: 'error', summary: 'Error', detail: (error as Error).message, life: 3000 })
  }
}

const uploadFile = (file: File) => {
  const formData = new FormData()
  formData.append('file', file)

  fetch(uploadToUrl, {
    method: 'POST',
    body: formData,
  })
    .then((response) => {
      if (response.ok) {
        toast.add({ severity: 'success', summary: 'Success', detail: 'Image uploaded successfully!', life: 3000 })
      }
      else {
        toast.add({ severity: 'error', summary: 'Error', detail: `Failed to upload image, status ${response.status}.`, life: 3000 })
      }
    })
    .catch((error: Error) => {
      toast.add({ severity: 'error', summary: 'Error', detail: `Failed to upload image : ${error.message}`, life: 3000 })
    })
}
</script>
