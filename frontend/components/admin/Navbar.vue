<template>
  <Toolbar>
    <template #start>
      <div class="flex align-items-center">
        <img
          height="40"
          width="40"
          alt="icon"
          :src="safehaven_logo"
        >
        <div class="pl-3">
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
      <Button
        label="Logout"
        link
        @click="(() => { state.logout() })"
      />
    </template>
  </Toolbar>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'
import safehaven_logo from '~/assets/logo_square.svg'

await state.fetchConfig()
</script>

<style scoped>
.round {
  width: 1.5rem;
  height: 1.5rem;
  border-radius: 50%;
  border-width: 2px;
  border-style: solid;
  display: inline-block;
}

.result {
  cursor: pointer;
  transition: background-color 0.2s;
  border-radius: 0.25rem;
}

.result:hover {
  background-color: #f0f0f0;
}

.filter-settings .button-content {
  z-index: 9;
}
</style>
