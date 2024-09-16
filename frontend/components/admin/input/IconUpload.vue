<template>
  <span
    class="flex gap-2 items-center"
  >
    <Button
      :label="$t('admin.input.iconUpload.changeIcon')"
      @click="visible = true"
    />
    <small>(.png, .svg, .bmp, .jpg, .jpeg, .ico, ...)</small>
  </span>
  <Dialog
    v-model:visible="visible"
    modal
    :header="$t('admin.input.iconUpload.changeIcon')"
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
          :choose-label="$t('admin.input.iconUpload.upload')"
          @upload="() => {
            toast.add({ severity: 'success', summary: $t('admin.input.iconUpload.success'), detail: $t('admin.input.iconUpload.uploadSuccess'), life: 3000 });
            visible = false
          }"
          @error="() => toast.add({ severity: 'error', summary: $t('admin.input.iconUpload.error'), detail: $t('admin.input.iconUpload.uploadFailed'), life: 3000 })"
        />
        <small>{{ $t('admin.input.iconUpload.localFile') }}</small>
      </div>
      <Divider
        layout="vertical"
        class="hidden md:flex"
      >
        <b>{{ $t('admin.input.iconUpload.or') }}</b>
      </Divider>
      <div class="flex flex-col gap-1 items-center">
        <InputText
          v-model="downloadFromUrl"
          :placeholder="$t('admin.input.iconUpload.urlPlaceholder')"
          :invalid="!isValidUrl(downloadFromUrl)"
        />
        <Button
          :label="$t('admin.input.iconUpload.save')"
          :disabled="!isValidUrl(downloadFromUrl)"
          @click="downloadImage"
        />
        <small>{{ $t('admin.input.iconUpload.downloadFromUrl') }}</small>
      </div>
    </span>
  </Dialog>
</template>

<script setup lang="ts">
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import { isValidUrl } from '~/lib/validation'

const toast = useToast()
const { t } = useI18n()
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
      throw new Error(t('admin.input.iconUpload.downloadFailed'))
    }
    const blob: Blob = await response.blob()
    const file: File = new File([blob], 'downloaded-image', { type: blob.type })
    uploadFile(file)
    visible.value = false
  }
  catch (error) {
    toast.add({ severity: 'error', summary: t('admin.input.iconUpload.error'), detail: (error as Error).message, life: 3000 })
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
        toast.add({ severity: 'success', summary: t('admin.input.iconUpload.success'), detail: t('admin.input.iconUpload.uploadSuccess'), life: 3000 })
      }
      else {
        toast.add({ severity: 'error', summary: t('admin.input.iconUpload.error'), detail: t('admin.input.iconUpload.uploadFailedWithStatus', { status: response.status }), life: 3000 })
      }
    })
    .catch((error: Error) => {
      toast.add({ severity: 'error', summary: t('admin.input.iconUpload.error'), detail: t('admin.input.iconUpload.uploadFailedWithMessage', { message: error.message }), life: 3000 })
    })
}
</script>
