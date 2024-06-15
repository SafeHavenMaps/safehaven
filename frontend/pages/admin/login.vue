<template>
  <form
    method="post"
    class="flex flex-column justify-content-center align-items-center h-full gap-5 hot-pink-bg"
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
        <div class="flex flex-column gap-3 max-w-17rem">
          <Message
            v-if="failed_attempt"
            severity="error"
            :closable="false"
            class="-my-1"
          >
            Nom d'utilisateur⋅ice ou
            mot de passe incorrect
          </Message>
          <label for="username">
            Nom d'utilisateur⋅ice
          </label>
          <InputText
            id="username"
            v-model="username"
            :invalid="failed_attempt"
            class="w-full -mt-2"
          />
          <label for="password">
            Mot de passe
          </label>
          <Password
            id="password"
            v-model="password"
            :feedback="false"
            :invalid="failed_attempt"
            toggle-mask
            class="w-full -mt-2"
          />
          <span class="hidden">
            <Checkbox
              v-model="remember_me"
              input-id="remember_me"
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
            class="w-full justify-content-center"
            :disabled="awaiting_auth_response"
          >
            <AppIcon
              v-if="awaiting_auth_response"
              icon-name="loading"
              rotating
            />
          </Button>
        </div>
      </template>
    </Card>
  </form>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'
import state from '~/lib/admin-state'

if (await state.check_login()) {
  await navigateTo('/admin/home')
}

const username: Ref<string> = ref('')
const password: Ref<string> = ref('')
const remember_me: Ref<boolean> = ref(false)
const failed_attempt: Ref<boolean> = ref(false)
const awaiting_auth_response: Ref<boolean> = ref(false)

const redirect_query_param = useRoute().query.redirect
let redirectUrl = '/admin/home'
// type checking of the query parameter to correspond to the signature of navigateTo
if (typeof redirect_query_param === 'string') {
  // matching to keep only internal urls
  const match = redirect_query_param.match('/admin*')
  if (match) {
    redirectUrl = match[0]
  }
}

async function login() {
  awaiting_auth_response.value = true
  failed_attempt.value = false

  try {
    await state.login(username.value, password.value, remember_me.value)
    await navigateTo(redirectUrl)
  }
  catch (error) {
    failed_attempt.value = true
    awaiting_auth_response.value = false
    console.error('Login failed:', error)
  }
}
</script>
