<template>
  <div class="flex flex-col h-full w-full logoized">
    <AdminNavbar
      @toggle-sidebar="toggleSidebar"
    />

    <div
      ref="contentContainerRef"
      class="flex w-full grow content-container"
    >
      <div
        ref="sidebarRef"
        class="sidebar overflow-y-auto"
        :class="{ collapsed: sidebarCollapsed }"
      >
        <div class="sidebar-title mb-1">
          Navigation
        </div>
        <AdminSidebar />
      </div>

      <div
        class="flex flex-col w-full main-content"
        :class="{ collapsed: sidebarCollapsed }"
      >
        <Breadcrumb
          :home="{ label: '', url: '' }"
          :model="currentBreadcrumbs"
          class="breadcrumb !p-2"
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

        <Card class="ml-0 mt-1 grow w-full scroll-container">
          <template #title>
            <div class="flex items-center gap-2 font-semibold">
              <AppIcon
                :icon-name="cardIconName"
                class=""
              />
              {{ cardTitle }}
              <div
                v-for="(action, index) in currentActions"
                :key="index"
                class="flex items-end"
              >
                <Button
                  outlined
                  class="!py-0 !px-1"
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

    <ConfirmPopup group="delete">
      <template #message="slotProps">
        <div class="flex flex-row items-center w-full gap-2 p-4 mb-2 pb-0">
          <AppIcon :icon-name="slotProps.message.icon!" />
          <span>{{ slotProps.message.message }} <b>{{ (slotProps.message as ExtendedConfirmationOptions).objectId }}</b></span>
        </div>
      </template>
    </ConfirmPopup>
  </div>
</template>

<script setup lang="ts">
import type { ConfirmationOptions } from 'primevue/confirmationoptions'
import AdminNavbar from '~/components/admin/Navbar.vue'

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
const contentContainerRef: Ref<HTMLElement | null> = ref(null)
const adminBreakingPoint = 768
const sidebarCollapsed = ref(window.innerWidth < adminBreakingPoint)

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

let hasBeenChangedManually = false

function toggleSidebar() {
  sidebarCollapsed.value = !sidebarCollapsed.value
  hasBeenChangedManually = true
}

provide('initAdminLayout', initAdminLayout)

function resizeEverything() {
  const navbar = document.querySelector('.admin-navbar')
  const sidebar = sidebarRef.value
  const mainContent = contentContainerRef.value

  // If the sidebar has been changed manually, we don't want to change it automatically
  if (!hasBeenChangedManually) {
  // If the sidebar is not collapsed but we go under the breaking point, we collapse it
    if (window.innerWidth < adminBreakingPoint && !sidebarCollapsed.value) {
      sidebarCollapsed.value = true
    }
    // Else, if the sidebar is collapsed but we go over the breaking point, we expand it
    else if (window.innerWidth >= adminBreakingPoint && sidebarCollapsed.value) {
      sidebarCollapsed.value = false
    }
  }

  if (navbar && sidebar && mainContent) {
    sidebar.style.top = `${navbar.clientHeight}px`
    mainContent.style.marginTop = `${navbar.clientHeight}px`
  }
}

onMounted(() => {
  window.addEventListener('resize', resizeEverything)
  nextTick(resizeEverything)
})
</script>

<style>
html, body {
  margin: 0 !important;
  padding: 0 !important;
  height: 100% !important;
}

html::not(.sh-dark) {
  background-color: #f7f7f7 !important;
}

html.sh-dark {
  background-color: #282828 !important;
}

.admin-navbar {
  border-radius: 0 !important;
  border-left-width: 0 !important;
  border-right-width: 0 !important;
  border-top-width: 0 !important;
  z-index: 1000 !important;
  position: fixed !important;
  top: 0 !important;
  left: 0 !important;
  width: 100% !important;
}

html.sh-dark .admin-navbar {
  background-color: #8a4467 !important;
}

html::not(.sh-dark) .admin-navbar {
  background-color: #E86BA7 !important;
}

.content-container {
  display: flex !important;
}

.sidebar {
  position: fixed !important;
  bottom: 0 !important;
  left: 0 !important;
  top: 74px;
  width: 17.7rem !important;
  z-index: 1001 !important;
  transition: transform 0.3s ease-in-out !important;
  transform: translateX(0) !important;
  padding: 1rem !important;
}

html::not(.sh-dark) .sidebar {
  background-color: #f7f7f7 !important;
}

html.sh-dark .sidebar {
  background-color: #282828 !important;
}

.sidebar.collapsed {
  transform: translateX(-100%) !important;
}

.sidebar-title {
  font-size: 1.25rem !important;
  font-weight: 600 !important;
}

.main-content {
  margin-left: 18rem !important;
  transition: margin-left 0.3s ease-in-out !important;
  width: calc(100% - 18rem) !important;
}

.main-content.collapsed {
  margin-left: 0 !important;
  width: 100% !important;
}

.scroll-container {
  overflow-x: auto !important;
  height: 100% !important;
}

.breadcrumb {
  background-color: transparent !important;
}
</style>
