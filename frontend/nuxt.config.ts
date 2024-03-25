export default defineNuxtConfig({
  modules: ["@nuxt/eslint"],
  ssr: false,
  devtools: { enabled: true },
  typescript: {
    typeCheck: true,
  },
});
