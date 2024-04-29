<template>
  <Toolbar>
    <template #start>
      <div class="grid nested-grid align-items-center -my-5 px-4">
        <div class="col-2">
          <img class="max-h-2rem" alt="icon" src="/assets/default-logo.png">
        </div>
        <div class="col-10">
          <h3 class="my-0 ">Carte associative</h3>
          <span class="text-xs font-italic">Powered by SafeHaven</span>
        </div>
      </div>
    </template>
    <template #center>
      <ClientFamilySwitcher
        :selectedFamily=selectedFamily
        :families=families
        />
    </template>

    <template #end>
      <Button label="Informations" icon="pi pi-info-circle" class="p-button-text mr-2" />
      <Button label="Ajouter" icon="pi pi-plus" class="p-button-success mr-2" />
      <Button label="Filtres" icon="pi pi-filter" class="p-button-help mr-2" />
      <Button icon="pi pi-search" class="p-button-warning" />
      <Button icon="pi pi-cog" class="p-button-secondary" />
    </template>
  </Toolbar>
</template>

<script  setup lang="ts">
import type { Family } from '~/lib';

const props = defineProps<{
  selectedFamily: Family|null;
  families: Family[];
}>();
</script>
