<template>

<form method="post" @submit.prevent="login" class="flex flex-column justify-content-center align-items-center h-full gap-4">
    <AppIcon icon-name="login" class="h-8rem w-8rem -mt-8 -mb-2"/>
    <FloatLabel>
        <InputText id="username" type="text" v-model=username />
        <label for="username">Username</label>
    </FloatLabel>
    <FloatLabel>
        <InputText id="password" v-model=password />
        <label for="password">Password</label>
    </FloatLabel>
    <Button label="Login" type="submit" class=""/>
</form>
</template>


<script setup lang="ts">
import state from '~/lib/admin-state'
const username : Ref<string> = ref("");
const password : Ref<string> = ref("");

async function login() {
  console.log("Logging in with", username.value, password.value);
  try {
      await state.initWithUsernamePassword(username.value, password.value);
  } catch (error) {
      console.error("Login failed:", error);
  }
}
</script>