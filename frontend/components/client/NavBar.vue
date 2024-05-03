<template>
  <Toolbar>
    <template #start>
      <div class="grid nested-grid align-items-center -my-5 px-4">
        <div class="col-2">
          <img class="max-h-2rem" alt="icon" src="/assets/default-logo.png" />
        </div>
        <div class="col-10">
          <h3 class="my-0">Carte associative</h3>
          <span class="text-xs font-italic">Powered by SafeHaven</span>
        </div>
      </div>
    </template>

    <template #center>
      <ClientFamilySwitcher />
    </template>

    <template #end>
      <div class="align-items-center">
        <Button label="Informations" class="p-button-text mr-2">
          <template #icon>
            <AppIcon icon-name="information" />
          </template>
        </Button>
        <Button label="Ajouter" class="p-button-success mr-2">
          <template #icon>
            <AppIcon icon-name="addEntity" class="-ml-1 mr-1" />
          </template>
        </Button>
        <Button label="Filtres" class="p-button-help mr-2">
          <template #icon>
            <AppIcon icon-name="filter" class="-ml-1 mr-1" />
          </template>
        </Button>
        <Button icon="pi pi-search" class="p-button-warning mr-2">
          <template #icon>
            <AppIcon icon-name="mapSearch" />
          </template>
        </Button>
        <Button icon="pi pi-cog" class="p-button-secondary">
          <template #icon>
            <AppIcon icon-name="lightDark" />
          </template>
        </Button>
      </div>
    </template>
  </Toolbar>
</template>

<script setup lang="ts"></script>
