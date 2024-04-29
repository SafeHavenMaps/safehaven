<template>
    <SelectButton v-model="selectedFamily" :options="families" @change="setFamily" optionLabel="name" optionValue="id"
        aria-labelledby="custom">
        <template #item="slotProps">
            <i :class="slotProps.option.icon" class="mr-2"></i>
            {{ slotProps.option.name }}
        </template>
    </SelectButton>
</template>


<script  setup lang="ts">
import type { Family } from '~/lib';


const props = defineProps<{
  selectedFamily: Family|null;
  families: Family[];
}>();
</script>