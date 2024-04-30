<template>
    <SelectButton 
        v-model="state.activeFamily"
        :options="state.families"
        optionLabel="title"
        aria-labelledby="custom"
    >
        <template #option="slotProps">
            <i :class="slotProps.option.icon" class="mr-2"></i>
            {{ slotProps.option.title }}
        </template>
    </SelectButton>
</template>


<script  setup lang="ts">
import state from "~/lib/state";
</script>
