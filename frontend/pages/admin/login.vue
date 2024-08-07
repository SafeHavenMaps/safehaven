<template>
  <form
    method="post"
    class="flex flex-col justify-center items-center h-full gap-8 hot-pink-bg"
    @submit.prevent="login"
  >
    <img
      src="assets/logo_secondary.svg"
      class="w-60 -mt-20 -mb-3"
    >
    <Card class="p-3 m-2">
      <template #title>
        Espace d'administration
      </template>
      <template #content>
        <div class="flex flex-col gap-4 max-w-[17rem]">
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
            v-model="password"
            input-id="password"
            input-class="w-full"
            :feedback="false"
            :invalid="failed_attempt"
            toggle-mask
            class="w-full -mt-2"
          />
          <span>
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
            class="w-full justify-center"
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
  const match = redirect_query_param.match(/\/admin\/.+/)
  if (match) {
    redirectUrl = match[0]
  }
}

if (await state.check_login()) {
  window.location.href = redirectUrl
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
