<template>
  Modifier l'icône
  <div class="flex gap-2 align-items-center">
    <div class="flex flex-column gap-1 align-items-center">
      <FileUpload
        mode="basic"
        name="file"
        url="/api/upload"
        accept="image/*"
        auto
        choose-label="Upload"
        @upload="() => toast.add({ severity: 'success', summary: 'Success', detail: 'Image uploaded successfully!', life: 3000 })"
        @error="() => toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to upload image.', life: 3000 })"
      />
      <small>Télecharger une icône depuis vos fichier locaux (.png, .svg, .bmp, .jpg, .jpeg)</small>
    </div>
    <Divider
      layout="vertical"
      class="hidden md:flex"
    >
      <b>OU</b>
    </Divider>
    <div class="flex flex-column gap-1 align-items-center">
      <InputText
        v-model="imageUrl"
        placeholder="http://url.com/icon.svg"
      />
      <Button
        label="Sauvegarder"
        @click="downloadImage"
      />
      <small>Télecharger une icône depuis une url (.png, .svg, .bmp, .jpg, .jpeg)</small>
    </div>
  </div>
</template>

<script setup>
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'

const imageUrl = ref('')
const toast = useToast()

const downloadImage = async () => {
  if (!imageUrl.value) {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Please enter a valid image URL.', life: 3000 })
    return
  }

  try {
    const response = await fetch(imageUrl.value)
    if (!response.ok) {
      throw new Error('Failed to download image.')
    }
    const blob = await response.blob()
    const file = new File([blob], 'downloaded-image', { type: blob.type })
    uploadFile(file)
  }
  catch (error) {
    toast.add({ severity: 'error', summary: 'Error', detail: error.message, life: 3000 })
  }
}

const uploadFile = (file) => {
  const formData = new FormData()
  formData.append('file', file)

  fetch('/api/upload', {
    method: 'POST',
    body: formData,
  })
    .then(response => response.json())
    .then((data) => {
      if (data.success) {
        toast.add({ severity: 'success', summary: 'Success', detail: 'Image uploaded successfully!', life: 3000 })
      }
      else {
        toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to upload image.', life: 3000 })
      }
    })
    .catch((_error) => {
      toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to upload image.', life: 3000 })
    })
}
</script>
