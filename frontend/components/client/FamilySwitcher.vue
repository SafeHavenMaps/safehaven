<template>
    <SelectButton :modelValue="selectedFamily!.id" :options="families" @change="state.changeActiveFamily" optionLabel="title" optionValue="id"
        aria-labelledby="custom">
        <template #option="slotProps">
            <i :class="slotProps.option.icon" class="mr-2"></i>
            {{ slotProps.option.title }}
        </template>
    </SelectButton>
</template>


<script  setup lang="ts">
import type { Family } from '~/lib';
import state from "~/lib/state";


const props = defineProps<{
  selectedFamily: Family|null;
  families: Family[];
}>(); 

</script>