<template>
  <div>
    <form
      class="flex flex-column gap-3 mx-4"
      style="width:68rem;"
    >
      <!-- @submit.prevent="onSave" -->
      <VAceEditor
        ref="editor"
        v-model:value="editorContent"
        lang="json"
        theme="monokai"
        style="height: 400px; width: 100%;"
        @init="onEditorInit"
      />
      <button @click="logEditorContent">
        Log Content
      </button>

      <!-- <span class="flex gap-1 justify-content-end">
        <NuxtLink to="/admin/families">
          <Button
            label="Annuler"
            severity="secondary"
            :loading="processingRequest"
            :disabled="processingRequest"
          />
        </NuxtLink>
        <Button
          label="Sauvegarder"
          type="submit"
          :loading="processingRequest"
          :disabled="processingRequest || anyFieldTitleOrKeyEmpty"
        />
      </span> -->
    </form>
  </div>
</template>

<script setup lang="ts">
// import type { ConfirmationOptions } from 'primevue/confirmationoptions'
// import type { FormField, StringFieldTypeMetadata, OptionsFieldTypeMetadata } from '~/lib'

import 'ace-builds/src-noconflict/ace' // Core Ace styles

import { VAceEditor } from 'vue3-ace-editor'
import * as ace from 'ace-builds'

// Import the necessary modes and themes from ace-builds
import 'ace-builds/src-noconflict/mode-javascript'
import 'ace-builds/src-noconflict/theme-monokai' // Theme CSS

ace.config.set('basePath', '/node_modules/ace-builds/src-noconflict')
const editorContent = ref<string>('console.log("Hello, world!")')

function logEditorContent() {
  console.log('Editor content:', 'editorContent.value')
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
