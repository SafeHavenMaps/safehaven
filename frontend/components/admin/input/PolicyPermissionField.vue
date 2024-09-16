<template>
  <div class="flex flex-col gap-4">
    <span class="flex items-center gap-2">
      <ToggleSwitch
        :model-value="props.modelValue.allow_all"
        :input-id="`${props.label}.allow_all`"
        @update:model-value="(value:boolean) => emit('update:model-value', { ...props.modelValue, allow_all: value })"
      />
      <label :for="`${props.label}.allow_all`">{{ $t('cmp.admin.input.policyPermissionField.accessAll', { label: props.label }) }}</label>
    </span>
    <div
      v-if="props.modelValue.allow_all"
      class="flex flex-col gap-2"
    >
      <label :for="`${props.label}.force_exclude`">{{ $t('cmp.admin.input.policyPermissionField.selectExclude', { label: props.label }) }}</label>
      <MultiSelect
        :model-value="props.modelValue.force_exclude"
        :options="props.options"
        option-label="title"
        option-value="id"
        :input-id="`${props.label}.force_exclude`"
        @update:model-value="(value: string[]) => emit('update:model-value', { ...props.modelValue, force_exclude: value })"
      />
    </div>
    <div
      v-if="!props.modelValue.allow_all"
      class="flex flex-col gap-2"
    >
      <label :for="`${props.label}.allow_list`">{{ $t('cmp.admin.input.policyPermissionField.selectInclude', { label: props.label }) }}</label>
      <MultiSelect
        :model-value="props.modelValue.allow_list"
        :options="props.options"
        option-label="title"
        option-value="id"
        :input-id="`${props.label}.allow_list`"
        @update:model-value="(value: string[]) => emit('update:model-value', { ...props.modelValue, allow_list: value })"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PermissionPolicy } from '~/lib'

interface Options {
  id: string
  title: string
}

const props = defineProps<{
  modelValue: PermissionPolicy
  options: Array<Options>
  label: string
}>()

const emit = defineEmits(['update:model-value'])
</script>
