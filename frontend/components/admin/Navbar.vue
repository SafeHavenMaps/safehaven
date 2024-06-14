<template>
  <Toolbar
    class="admin-navbar mb-2 coco-text top-0 left-0 fixed w-full"
  >
    <template #start>
      <div class="flex align-items-center">
        <Button
          outlined
          severity="secondary"
          small
          class="p-1 mr-2"
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

        <img
          height="40"
          width="40"
          alt="icon"
          :src="safehaven_logo"
        >
        <div class="pl-3">
          <h3 class="my-0">
            SafeHaven
          </h3>
          <span class="text-xs font-italic">
            {{ state.options.general.title }}
          </span>
        </div>
      </div>
    </template>

    <template #center />

    <template #end>
      <div class="flex flex-column align-items-end">
        <span>Bienvenu⋅e <strong>{{ state.username }}</strong></span>
        <Button
          class="p-0"
          label="Se déconnecter"
          style="color: white; text-decoration: underline;"
          link
          @click="(() => { state.logout() })"
        />
      </div>
      <NuxtLink to="/admin/users/self">
        <Button
          text
          rounded
          severity="secondary"
          class="p-0 mx-2"
        >
          <template #default>
            <Avatar
              :label="state.username![0].toUpperCase()"
              shape="circle"
              size="large"
            />
          </template>
        </Button>
      </NuxtLink>
    </template>
  </Toolbar>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'
import safehaven_logo from '~/assets/logo_square_white.svg'

const emit = defineEmits(['toggleSidebar'])

await state.fetchConfig()
await state.check_login()
</script>

<style>
.admin-navbar {
  background-color: #E86BA7;
  border-radius: 0;
  border-left-width: 0;
  border-right-width: 0;
  border-top-width: 0;
  z-index: 1000;
}
</style>
