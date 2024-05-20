<template>
  <Toolbar>
    <template #start>
      <div class="flex align-items-center">
        <div class="flex-shrink-0">
          <img
            height="40"
            width="40"
            alt="icon"
            :src="state.logo ?? defaultLogo"
          >
        </div>
        <div class="flex-grow-1 pl-3">
          <h3 class="my-0">
            {{ state.title }}
          </h3>
          <span class="text-xs font-italic">
            {{ state.subtitle }}
          </span>
        </div>
      </div>
    </template>

    <template #center>
      <ViewerFamilySwitcher />
    </template>

    <template #end>
      <div class="align-items-center">
        <Button
          label="Informations"
          class="p-button-text mr-2"
        >
          <template #icon>
            <AppIcon icon-name="information" />
          </template>
        </Button>
        <Button
          label="Ajouter"
          class="p-button-success mr-2"
        >
          <template #icon>
            <AppIcon
              icon-name="addEntity"
              class="-ml-1 mr-1"
            />
          </template>
        </Button>
        <Button
          label="Filtres"
          class="p-button-help mr-2"
        >
          <template #icon>
            <AppIcon
              icon-name="filter"
              class="-ml-1 mr-1"
            />
          </template>
        </Button>
        <Button
          icon="pi pi-search"
          class="p-button-warning mr-2"
        >
          <template #icon>
            <AppIcon icon-name="mapSearch" />
          </template>
        </Button>
        <Button
          icon="pi pi-cog"
          class="p-button-secondary"
        >
          <template #icon>
            <AppIcon icon-name="lightDark" />
          </template>
        </Button>
      </div>
    </template>
  </Toolbar>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'
import defaultLogo from '~/assets/logo_square.svg'
</script>
