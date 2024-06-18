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
          v-tooltip.bottom="'Gras'"
          class="ql-bold"
        />
        <button
          v-tooltip.bottom="'Italique'"
          class="ql-italic"
        />
        <button
          v-tooltip.bottom="'Souligner'"
          class="ql-underline"
        />
        <button
          v-tooltip.bottom="'Barrer'"
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
          v-tooltip.bottom="'Indice'"
          class="ql-script"
          value="sub"
        />
        <button
          v-tooltip.bottom="'Exposant'"
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
