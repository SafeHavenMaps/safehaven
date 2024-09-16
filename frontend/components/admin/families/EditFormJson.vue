<template>
  <div>
    <form
      class="flex flex-col gap-3 mx-4"
      style="width:68rem;"
      @submit.prevent="onSync"
    >
      <p class="text-muted-color">
        {{ $t('cmp.admin.families.editformjson.description') }}
      </p>
      <!-- @submit.prevent="onSave" -->
      <VAceEditor
        ref="editor"
        v-model:value="editorContent"
        lang="json"
        theme="monokai"
        style="height: 400px; width: 100%;"
        @init="onEditorInit"
      />
      <span class="flex gap-1 justify-content-end">
        <Button
          :label="$t('cmp.admin.families.editformjson.import')"
          severity="info"
          @click="triggerImport"
        >
          <template #icon>
            <AppIcon
              icon-name="upload"
              class="-mx-1"
            />
          </template>
        </Button>
        <input
          ref="fileInput"
          type="file"
          accept=".json"
          style="display: none;"
          @change="onImport"
        >
        <a
          ref="exportLink"
          download="formFields.json"
        >
          <Button
            :label="$t('cmp.admin.families.editformjson.export')"
            severity="info"
            @click="onExport"
          >
            <template #icon>
              <AppIcon
                icon-name="download"
                class="-mx-1"
              />
            </template>
          </Button>
        </a>
      </span>

      <span class="flex gap-1 justify-content-end">
        <NuxtLink to="/admin/families">
          <Button
            :label="$t('cmp.admin.families.editform.cancel')"
            severity="secondary"
          />
        </NuxtLink>
        <Button
          :label="$t('cmp.admin.families.editformjson.syncAndPreview')"
          type="submit"
        />
      </span>
    </form>
  </div>
</template>

<script setup lang="ts">
// import type { ConfirmationOptions } from 'primevue/confirmationoptions'
// import type { FormField, StringFieldTypeMetadata, OptionsFieldTypeMetadata } from '~/lib'
import { toRaw } from 'vue'
import 'ace-builds/src-noconflict/ace' // Core Ace styles

import { VAceEditor } from 'vue3-ace-editor'
import * as ace from 'ace-builds'

// Import the necessary modes and themes from ace-builds
import 'ace-builds/src-noconflict/mode-json'
import 'ace-builds/src-noconflict/theme-monokai' // Theme CSS
import type { FormField } from '~/lib'

ace.config.set('basePath', '/node_modules/ace-builds/src-noconflict')

const props = defineProps<{
  kind: 'entity' | 'comment'
  kindName: string
  originalFormFields: FormField[]
  onSyncCallback: (editedFormFields: FormField[]) => Promise<{ error: Error | undefined }>
}>()

const toast = useToast()
const { t } = useI18n()

const editedFormFields: FormField[] = JSON.parse(JSON.stringify(toRaw(props.originalFormFields))) // deep copy
editedFormFields.sort((field_a, field_b) => field_a.form_weight - field_b.form_weight)

const editorContent = ref<string>(JSON.stringify(toRaw(editedFormFields), null, 2))

// Function to trigger import
function triggerImport() {
  const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement
  fileInput.click()
}

// Function to handle file import
function onImport(event: Event) {
  try {
    const fileInput = event.target as HTMLInputElement
    const file = fileInput.files?.[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (e) => {
        const result = e.target?.result as string
        try {
          editorContent.value = JSON.stringify(JSON.parse(result), null, 2)
        }
        catch {
          toast.add({ severity: 'error', summary: t('cmp.admin.families.editform.error'), detail: t('cmp.admin.families.editformjson.invalidJson'), life: 3000 })
        }
      }
      reader.readAsText(file)
      toast.add({ severity: 'success', summary: t('cmp.admin.families.editform.success'), detail: t('cmp.admin.families.editformjson.importSuccess'), life: 3000 })
    }
  }
  catch {
    toast.add({ severity: 'error', summary: t('cmp.admin.families.editform.error'), detail: t('cmp.admin.families.editformjson.importError'), life: 3000 })
  }
}

const exportLink = ref<HTMLAnchorElement | null>(null)
// Function to handle export
function onExport() {
  const json = editorContent.value
  const blob = new Blob([json], { type: 'application/json' })
  if (exportLink.value) {
    exportLink.value.href = URL.createObjectURL(blob)
    exportLink.value.download = 'formFields.json'
  }
}

// Synchronisation function invoking the callback
async function onSync() {
  try {
    const newFormFields = JSON.parse(editorContent.value)
    await props.onSyncCallback(newFormFields as FormField[])
    toast.add({ severity: 'success', summary: t('cmp.admin.families.editform.success'), detail: t('cmp.admin.families.editformjson.syncSuccess'), life: 3000 })
  }
  catch {
    toast.add({ severity: 'error', summary: t('cmp.admin.families.editform.error'), detail: t('cmp.admin.families.editformjson.syncError'), life: 3000 })
  }
}

interface CustomCompletion {
  caption: string
  value: string
  meta: string
}

// Custom completer function
const customCompleter = {
  getCompletions: function (editor: ace.Ace.Editor,
    session: ace.Ace.EditSession,
    pos: ace.Ace.Point,
    prefix: string,
    callback: (error: null | Error, completions: CustomCompletion[]) => void,
  ) {
    // Define custom completions
    const customCompletions = [
      {
        caption: 'console.log',
        value: 'console.log()',
        meta: 'custom',
      },
      {
        caption: 'alert',
        value: 'alert()',
        meta: 'custom',
      },
      // Add more custom completions here
    ]

    // Provide the custom completions
    callback(null, customCompletions)
  },
}

// Function to handle editor initialization
function onEditorInit(editor: ace.Ace.Editor) {
  // Enable the language tools (autocomplete)
  editor.setOptions({
    enableBasicAutocompletion: true,
    enableLiveAutocompletion: true,
  })

  // Add the custom completer
  editor.completers = [customCompleter]
}
</script>
