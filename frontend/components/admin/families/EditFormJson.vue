<template>
  <div>
    <form
      class="flex flex-col gap-3 mx-4"
      style="width:68rem;"
      @submit.prevent
    >
      <p class="text-muted-color">
        Édition directe du formulaire d'ajout en json, utile pour l'import/export.
        Ne réalisez d'autre changements par ce biais que si vous comprenez précisément la structure du formulaire.
        Pour sauvegarder, utilisez le bouton de prévisualisation, qui vous permettra d'abord de vérifier vos changements dans l'outil d'édition visuelle.
        Tout changement réalisé dans l'éditeur visuel auparavant et non sauvegardé sera écrasé.
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
          label="Import"
          severity="info"
        />
        <Button
          label="Export"
          severity="info"
        />
      </span>

      <span class="flex gap-1 justify-content-end">
        <NuxtLink to="/admin/families">
          <Button
            label="Annuler"
            severity="secondary"
          />
        </NuxtLink>
        <Button
          label="Synchroniser et prévisualiser"
          type="submit"
        />
      </span>
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
import 'ace-builds/src-noconflict/mode-json'
import 'ace-builds/src-noconflict/theme-monokai' // Theme CSS
import type { FormField } from '~/lib'

ace.config.set('basePath', '/node_modules/ace-builds/src-noconflict')

const props = defineProps<{
  kind: 'entity' | 'comment'
  kindName: string
  originalFormFields: FormField[]
  onSaveCallback: (editedFormFields: FormField[]) => Promise<{ error: Error | undefined }>
}>()

const edited_form_fields: Ref<FormField[]> = ref(JSON.parse(JSON.stringify(props.originalFormFields))) // deep copy
edited_form_fields.value.sort((field_a, field_b) => field_a.form_weight - field_b.form_weight)

const editorContent = ref<string>(JSON.stringify(edited_form_fields, null, 2))

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
