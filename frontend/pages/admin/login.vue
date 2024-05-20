<template>
  <form
    method="post"
    class="flex flex-column justify-content-center align-items-center h-full gap-5 body"
    @submit.prevent="login"
  >
    <img
      src="assets/logo_secondary.svg"
      class="w-14rem -mt-8 -mb-2"
    >
    <Card>
      <template #title>
        <span>Espace d'administration</span>
      </template>
      <template #content>
        <div class="flex flex-column gap-3">
          <label for="username">
            Nom d'utilisateur⋅ice
          </label>
          <InputText
            id="username"
            v-model="username"
            class="w-full -mt-2"
          />
          <label for="password">
            Mot de passe
          </label>
          <Password
            id="password"
            v-model="password"
            :feedback="false"
            toggle-mask
            class="w-full -mt-2"
          />
          <span>
            <Checkbox
              id="remember_me"
              v-model="remember_me"
              :binary="true"
            />
            <label
              for="remember_me"
              class="ml-1 text-sm"
            > Se souvenir de moi </label>
          </span>

          <Button
            label="Login"
            type="submit"
            class="w-full"
          />
        </div>
      </template>
    </Card>
  </form>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'

const username: Ref<string> = ref('')
const password: Ref<string> = ref('')
const remember_me: Ref<boolean> = ref(false)

async function login() {
  console.log('Logging in with', username.value, password.value, 'with remember me set to', remember_me.value)
  try {
    await state.initWithUsernamePassword(username.value, password.value, remember_me.value)
  }
  catch (error) {
    console.error('Login failed:', error)
  }
}
</script>

<style scoped>
.body {
  background-color: #E86BA7;
}
</style>
