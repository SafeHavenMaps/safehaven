<template>
  <div class="flex flex-column h-full w-full logoized">
    <AdminNavbar @toggle-sidebar="() => { sidebarCollapsed = !sidebarCollapsed; }" />

    <div class="flex w-full flex-grow-1 content-container">
      <div
        ref="sidebarRef"
        class="sidebar"
        :class="{ collapsed: sidebarCollapsed }"
      >
        <div class="sidebar-title mb-1">
          Navigation
        </div>
        <AdminSidebar />
      </div>

      <div
        class="flex flex-column w-full main-content"
        :class="{ collapsed: sidebarCollapsed }"
      >
        <Breadcrumb
          :home="{ label: '', url: '' }"
          :model="currentBreadcrumbs"
          class="breadcrumb p-1"
        >
          <template #item="{ item, props }">
            <NuxtLink
              v-if="item.label"
              v-slot="{ href, navigate }"
              :to="item.url"
              custom
            >
              <a
                :href="href"
                v-bind="props.action"
                @click="navigate"
              >
                <span class="p-text-secondary">{{ item.label }}</span>
              </a>
            </NuxtLink>
          </template>
        </Breadcrumb>

        <Card class="m-3 ml-0 mt-1 flex-grow-1 w-full scroll-container">
          <template #title>
            <div class="flex align-items-end">
              <AppIcon
                :icon-name="cardIconName"
                class="mr-2"
              />
              {{ cardTitle }}
              <div
                v-for="(action, index) in currentActions"
                :key="index"
              >
                <Button
                  outlined
                  class="ml-2 py-0 px-1"
                  :label="action.label"
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

    <Toast />

    <ConfirmPopup group="delete">
      <template #message="slotProps">
        <div class="flex flex-row align-items-center w-full gap-2 p-3 mb-2 pb-0">
          <AppIcon :icon-name="slotProps.message.icon!" />
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
  label: string
  severity: string
  url: string
}

export type InitAdminLayout = (
  newCardTitle: string,
  newCardIconName: string,
  actions: ActionItem[],
  breadcrumb: BreadcrumbItem[]
) => void

interface ExtendedConfirmationOptions extends ConfirmationOptions {
  objectId?: string
}

const currentBreadcrumbs = ref<BreadcrumbItem[]>([])
const currentActions = ref<ActionItem[]>([])
const cardTitle = ref('')
const cardIconName = ref('')
const sidebarRef: Ref<HTMLElement | null> = ref(null)
const sidebarCollapsed = ref(false)

function initAdminLayout(
  newCardTitle: string,
  newCardIconName: string,
  actions: ActionItem[],
  breadcrumb: BreadcrumbItem[],
) {
  cardTitle.value = newCardTitle
  cardIconName.value = newCardIconName
  currentActions.value = actions
  currentBreadcrumbs.value = breadcrumb
}

provide('initAdminLayout', initAdminLayout)
</script>

<style>
html, body {
  background-color: #f7f7f7;
  margin: 0;
  padding: 0;
  height: 100%;
}

.admin-navbar {
  background-color: #E86BA7;
  border-radius: 0;
  border-left-width: 0;
  border-right-width: 0;
  border-top-width: 0;
  z-index: 1000;
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
}

.content-container {
  margin-top: 80px; /* Adjust this value based on your navbar's height */
  transition: margin-top 0.3s;
  display: flex;
}

.sidebar {
  position: fixed;
  top: 80px;
  bottom: 0;
  left: 0;
  width: 250px; /* Adjust the sidebar width as needed */
  background-color: #f7f7f7; /* Ensure the sidebar has a background color */
  z-index: 1001;
  transition: transform 0.3s ease-in-out;
  transform: translateX(0);
  padding: 1rem;
}

.sidebar.collapsed {
  transform: translateX(-120%);
}

.sidebar-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: rgb(51, 65, 85);
}

.main-content {
  margin-left: 18rem; /* Adjust this value based on the sidebar width */
  transition: margin-left 0.3s ease-in-out;
  width: calc(100% - 18rem);
}

.main-content.collapsed {
  margin-left: 0;
  width: 100%;
}

.scroll-container {
  overflow-x: auto;
  height: 100%;
}

.breadcrumb {
  background-color: transparent;
}
</style>
