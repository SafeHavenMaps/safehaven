<template>
  <div class="flex flex-column h-full w-full logoized">
    <AdminNavbar />

    <div class="flex w-full flex-grow-1">
      <div class="flex w-full">
        <div class="m-3 mt-1 p-2">
          <div
            class="sidebar-title mb-1"
          >
            Navigation
          </div>

          <AdminSidebar />
        </div>

        <div class="flex flex-column w-full">
          <Breadcrumb
            :home="cardHomeBreadcrumb()"
            :model="cardItemsBreadcrumb()"
            class="breadcrumb p-1"
          >
            <template #item="{ item, props }">
              <NuxtLink
                v-slot="{ href, navigate }"
                :to="item.url"
                custom
              >
                <a
                  :href="href"
                  v-bind="props.action"
                  @click="navigate"
                >
                  <span class="text-primary font-semibold">
                    {{ item.label }}
                  </span>
                </a>
              </NuxtLink>
            </template>
          </Breadcrumb>

          <Card class="m-3 ml-0 mt-1 flex-grow-1 w-full">
            <template #title>
              <div class="flex">
                <AppIcon
                  :icon-name="cardIconName()"
                  class="mr-2"
                />
                {{ cardTitle() }}
                <div
                  v-for="(action, index) in currentActions"
                  :key="index"
                >
                  <Button
                    outlined
                    rounded
                    :severity="action.severity"
                    @click="navigateTo(action.url)"
                  >
                    <template #icon>
                      <AppIcon :icon-name="action.icon" />
                    </template>
                  </Button>
                </div>
              </div>
            </template>

            <template #content>
              <slot />
            </template>
          </Card>
        </div>
      </div>
    </div>

    <ConfirmPopup group="delete">
      <template #message="slotProps">
        <div class="flex flex-row align-items-center w-full gap-2 p-3 mb-2 pb-0">
          <AppIcon
            :icon-name="slotProps.message.icon!"
          />
          <span>{{ slotProps.message.message }} <b>{{ (slotProps.message as ExtendedConfirmationOptions).objectId }}</b></span>
        </div>
      </template>
    </ConfirmPopup>
  </div>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'

type BreadcrumbItem = {
  label: string
  url: string
}

type ActionItem = {
  icon: string
  severity: string
  url: string
}

export type InitAdminLayout = (
  actions: ActionItem[],
  breadcrumb: BreadcrumbItem[]
) => void

interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}

const currentBreadcrumbs = ref<BreadcrumbItem[]>([])
const currentActions = ref<ActionItem[]>([])

function cardIconName() {
  return useRoute().meta.cardIcon as string
}

function cardTitle() {
  return useRoute().meta.cardTitle ?? 'Contenu'
}

function cardHomeBreadcrumb() {
  return currentBreadcrumbs.value[0]!
}

function cardItemsBreadcrumb() {
  return currentBreadcrumbs.value.slice(1)
}

function initAdminLayout(actions: ActionItem[], breadcrumb: BreadcrumbItem[]) {
  currentActions.value = actions
  currentBreadcrumbs.value = breadcrumb
}

provide('initAdminLayout', initAdminLayout)
</script>

<style>
html, body {
  background-color: #f7f7f7;
}
.breadcrumb {
  background-color: transparent;
}
.sidebar-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: rgb(51, 65, 85);
}
</style>
