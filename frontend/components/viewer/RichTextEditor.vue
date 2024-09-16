<template>
  <Editor
    :model-value="props.modelValue!"
    editor-style="height:320px"
    :formats="['bold', 'italic', 'strike', 'script', 'underline']"
    @update:model-value="updateValue"
  >
    <template #toolbar>
      <span class="ql-formats">
        <button
          v-tooltip.bottom="$t('cmp.viewer.richTextEditor.bold')"
          class="ql-bold"
        />
        <button
          v-tooltip.bottom="$t('cmp.viewer.richTextEditor.italic')"
          class="ql-italic"
        />
        <button
          v-tooltip.bottom="$t('cmp.viewer.richTextEditor.underline')"
          class="ql-underline"
        />
        <button
          v-tooltip.bottom="$t('cmp.viewer.richTextEditor.strike')"
          class="ql-strike"
        />

        <!-- <select
          v-tooltip.bottom="'Taille'"
          class="ql-size"
        >
          <option value="small" />
          <option selected />
          <option value="large" />
        </select> -->
        <button
          v-tooltip.bottom="$t('cmp.viewer.richTextEditor.subscript')"
          class="ql-script"
          value="sub"
        />
        <button
          v-tooltip.bottom="$t('cmp.viewer.richTextEditor.superscript')"
          class="ql-script"
          value="super"
        />
      </span>
    </template>
  </Editor>
</template>

<script setup lang="ts">
import Editor from 'primevue/editor'

const props = defineProps<{
  modelValue: string | undefined | null
}>()

const emit = defineEmits(['update:modelValue'])

function updateValue(value: undefined | string) {
  emit('update:modelValue', value)
}
</script>
