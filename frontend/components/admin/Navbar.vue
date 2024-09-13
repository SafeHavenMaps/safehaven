<template>
  <Toolbar
    ref="navbarRef"
    class="admin-navbar mb-2 coco-text top-0 left-0 fixed w-full"
  >
    <template #start>
      <div class="flex items-center">
        <Button
          outlined
          severity="secondary"
          small
          class="!p-1 !mr-2"
          style="color: white; border-color: white;"
          @click="() => { emit('toggleSidebar') }"
        >
          <template #default>
            <AppIcon
              icon-name="menu"
              size="24px"
            />
          </template>
        </Button>

        <NuxtLink
          to="/admin"
          class="ml-1 "
        >
          <img
            height="40"
            width="40"
            alt="icon"
            :src="safehaven_logo"
          >
        </NuxtLink>
        <div class="navbar-text pl-4">
          <div class="my-0 text-lg font-extrabold">
            SafeHaven
          </div>
          <div class="text-xs italic">
            {{ state.options.general.title }}
          </div>
        </div>
      </div>
    </template>

    <template #center />

    <template #end>
      <Button
        severity="secondary"
        outlined
        style="color: white; border-color: white;"
        @click="toggleDarkMode()"
      >
        <template #icon>
          <AppIcon icon-name="lightDark" />
        </template>
      </Button>

      <template v-if="isUpdateAvailable">
        <OverlayBadge
          value="1"
          severity="danger"
        >
          <Button
            severity="secondary"
            outlined
            class="ml-2"
            style="color: white; border-color: white;"
            @click="toggleVersionPopup()"
          >
            <template #icon>
              <AppIcon icon-name="information" />
            </template>
          </Button>
        </OverlayBadge>
      </template>
      <template v-else>
        <Button
          severity="secondary"
          outlined
          class="ml-2"
          style="color: white; border-color: white;"
          @click="toggleVersionPopup()"
        >
          <template #icon>
            <AppIcon icon-name="information" />
          </template>
        </Button>
      </template>

      <Button
        rounded
        severity="secondary"
        class="p-0 mx-2"
        aria-haspopup="true"
        aria-controls="accountMenu"
        @click="toggleAccountMenu"
      >
        <template #default>
          <AdminUserAvatar
            :username="state.username ?? 'not logged'"
            size="normal"
          />
        </template>
      </Button>

      <Menu
        id="accountMenu"
        ref="accountMenu"
        :model="items"
        :popup="true"
      />
    </template>
  </Toolbar>

  <Dialog
    v-model:visible="versionModalVisible"
    header="Information de version"
    modal
    dismissable-mask
  >
    <Message
      v-if="isUpdateAvailable"
      severity="warn"
      class="m-1"
    >
      Une mise à jour est disponible.<br>
      <a href="https://github.com/SafeHavenMaps/safehaven/releases">Rendez-vous sur GitHub</a> pour voir et mettre à jour vers la dernière version.
    </Message>
    <p>
      Version actuelle: <pre>{{ state.versionInformation?.version ?? 'Inconnu' }}</pre>
    </p>
    <p>
      Git hash: <pre>{{ state.versionInformation?.git_hash ?? 'Inconnu' }}</pre>
    </p>
    <p v-if="state.versionInformation?.github_latest_version">
      Dernière version: <pre>{{ state.versionInformation?.github_latest_version }}</pre>
    </p>
  </Dialog>
</template>

<script setup lang="ts">
import type Menu from 'primevue/menu'
import state from '~/lib/admin-state'
import safehaven_logo from '~/assets/logo_square_white.svg'

const emit = defineEmits(['toggleSidebar'])
const darkMode = useDarkMode()

const accountMenu = ref<typeof Menu | null>(null)
const versionModalVisible = ref(false)
const navbarRef: Ref<HTMLElement | null> = ref(null)

try {
  await state.check_login()
  await state.fetchConfig()
  await state.fetchVersionInformaton()
}
catch {
  // Do nothing
}

const isUpdateAvailable = computed(() => {
  return state.versionInformation?.version !== 'main'
    && state.versionInformation?.github_latest_version !== state.versionInformation?.version
})

const items = [
  {
    label: state.username!,
    items: [
      {
        label: 'Mon compte',
        command: () => {
          navigateTo('/admin/users/self')
        },
      },
      {
        label: 'Me déconnecter',
        command: () => {
          state.logout()
        },
      },
    ],
  },
]

function toggleAccountMenu(event: Event) {
  accountMenu.value?.toggle(event)
}

function toggleDarkMode() {
  darkMode.toggle()
}

function toggleVersionPopup() {
  versionModalVisible.value = !versionModalVisible.value
}
</script>

<style>
.admin-navbar {
  background-color: #E86BA7 !important;
  border-radius: 0 !important;
  border-left-width: 0 !important;
  border-right-width: 0 !important;
  border-top-width: 0 !important;
  z-index: 1000 !important;
}

.navbar-text {
  color: white;
}
</style>
