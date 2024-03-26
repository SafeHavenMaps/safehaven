export default defineNuxtConfig({
  modules: ["@nuxt/eslint"],
  ssr: false,
  devtools: { enabled: true },
  typescript: {
    typeCheck: true,
  },
  runtimeConfig: {
    public: {
      apiUrl: process.env.API_URL || "/",
    },
  },
});
