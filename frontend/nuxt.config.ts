// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import theme from './theme.mjs'

export default defineNuxtConfig({
  compatibilityDate: '2024-09-13',
  modules: [
    '@primevue/nuxt-module',
    '@nuxt/eslint',
    '@nuxtjs/tailwindcss',
    '@vueuse/nuxt',
    '@nuxtjs/i18n',
  ],
  ssr: false,
  devtools: { enabled: true },
  typescript: {
    typeCheck: true,
    strict: true,
  },
  primevue: {
    options: {
      ripple: true,
      theme: {
        preset: theme,
        options: {
          darkModeSelector: '.sh-dark',
        },
      },
    },
  },
  i18n: {
    locales: [{
      code: 'en',
      file: 'en.json',
    }, {
      code: 'fr',
      file: 'fr.json',
    }],
    langDir: 'langs/',
    lazy: true,
    strategy: 'no_prefix',
    defaultLocale: 'en',

    detectBrowserLanguage: {
      useCookie: true, // Use a cookie to remember the user's chosen language
      cookieKey: 'i18n_redirected',
      alwaysRedirect: true, // Always redirect to browser locale if it's different
      fallbackLocale: 'en', // Fallback if no match is found or browser locale unsupported
    },
  },
  css: [
    '~/assets/main.css',
    '~/assets/richtext.css',
  ],
  eslint: {
    config: {
      stylistic: true,
    },
  },
  app: {
    head: {
      title: 'Safehaven',
    },
  },
  vite: {
    resolve: {
      alias: {
        ace: 'ace-builds/src-noconflict',
      },
    },
  },
})
