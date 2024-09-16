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
    :header="$t('cmp.admin.navbar.versionInfo')"
    modal
    dismissable-mask
  >
    <Message
      v-if="isUpdateAvailable"
      severity="warn"
      class="m-1"
    >
      {{ $t('cmp.admin.navbar.updateAvailable') }}<br>
      <a href="https://github.com/SafeHavenMaps/safehaven/releases">{{ $t('cmp.admin.navbar.githubLink') }}</a>
    </Message>
    <div class="m-1">
      {{ $t('cmp.admin.navbar.currentVersion') }}: <pre>{{ state.versionInformation?.version ?? $t('cmp.admin.navbar.unknown') }}</pre>
    </div>
    <div class="m-1">
      {{ $t('cmp.admin.navbar.gitHash') }}: <pre>{{ state.versionInformation?.git_hash ?? $t('cmp.admin.navbar.unknown') }}</pre>
    </div>
    <div
      v-if="state.versionInformation?.github_latest_version"
      class="m-1"
    >
      {{ $t('cmp.admin.navbar.latestVersion') }}: <pre>{{ state.versionInformation?.github_latest_version }}</pre>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import type Menu from 'primevue/menu'
import state from '~/lib/admin-state'
import safehaven_logo from '~/assets/logo_square_white.svg'

const emit = defineEmits(['toggleSidebar'])
const darkMode = useDarkMode()
const { t } = useI18n()

const accountMenu = ref<typeof Menu | null>(null)
const versionModalVisible = ref(false)
const navbarRef: Ref<HTMLElement | null> = ref(null)

try {
  await state.check_login()
  await state.fetchConfig()
  await state.fetchVersionInformation()
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
        label: t('cmp.admin.navbar.myAccount'),
        command: () => {
          navigateTo('/admin/users/self')
        },
      },
      {
        label: t('cmp.admin.navbar.logout'),
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
