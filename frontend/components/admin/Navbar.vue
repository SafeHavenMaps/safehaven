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
          class="ml-3"
        >
          <img
            height="46"
            width="46"
            alt="icon"
            :src="safehaven_logo"
          >
        </NuxtLink>
        <div class="navbar-text pl-4">
          <div class="my-0 font-extrabold">
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
        rounded
        severity="secondary"
        class="p-0 mx-2"
        aria-haspopup="true"
        aria-controls="accountMenu"
        @click="toggleAccountMenu"
      >
        <template #default>
          <AdminUserAvatar
            :username="state.username!"
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
</template>

<script setup lang="ts">
import type Menu from 'primevue/menu'
import state from '~/lib/admin-state'
import safehaven_logo from '~/assets/logo_square_white.svg'

const emit = defineEmits(['toggleSidebar'])

const accountMenu = ref<typeof Menu | null>(null)

const navbarRef: Ref<HTMLElement | null> = ref(null)

await state.fetchConfig()
await state.check_login()

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
