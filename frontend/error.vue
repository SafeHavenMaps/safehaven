<template>
  <NuxtLayout class=" h-full w-full">
    <div class="error-page flex flex-col gap-5 justify-center items-center h-full w-full">
      <div>
        <img
          src="assets/logo_secondary.svg"
          class="w-60 -mt-20 -mb-3"
        >
      </div>
      <div class="text-5xl font-bold text-center">
        {{ $t('error.somethingWentWrong') }}
      </div>
      <div>
        {{ error.message }}
      </div>
    </div>
  </NuxtLayout>
</template>

<script setup>
const error = useError()
</script>

<style>
.error-page {
  height: 100%;
  width: 100%;
  background-color: #E86BA7;
  color: white;
}
</style>
