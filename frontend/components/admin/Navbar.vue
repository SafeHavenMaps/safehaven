<template>
  <div class="lilac-bg">
    <Toolbar class="lilac-bg border-none">
      <template #start>
        <div class="flex align-items-center">
          <img
            height="40"
            width="40"
            alt="icon"
            :src="safehaven_logo"
          >
          <div class="pl-3 coco-text">
            <h3 class="my-0">
              Espace d'administration SafeHaven
            </h3>
            <span class="text-xs font-italic">
              {{ state.options.general.title }}
            </span>
          </div>
        </div>
      </template>

      <template #center />

      <template #end>
        <div class="flex flex-column align-items-center ">
          <span class="coco-text"> Utilisateur⋅ice : {{ state.username }}</span>
          <Button
            class="flex"
            label="Logout"
            link
            @click="(() => { state.logout() })"
          />
        </div>
      </template>
    </Toolbar>
  </div>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'
import safehaven_logo from '~/assets/logo_square.svg'

await state.fetchConfig()
await state.check_login()
</script>
