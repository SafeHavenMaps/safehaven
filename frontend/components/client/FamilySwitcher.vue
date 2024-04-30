<template>
    <SelectButton 
        v-model="state.activeFamily"
        :options="state.families"
        optionLabel="title"
        @onchange="triggerChange"
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

async function triggerChange() {
  emit('change');
}

const emit = defineEmits<{
  change: [];
}>();
</script>
