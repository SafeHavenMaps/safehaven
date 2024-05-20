// @ts-check
import stylistic from '@stylistic/eslint-plugin'
import withNuxt from './.nuxt/eslint.config.mjs'

export default withNuxt(
  stylistic.configs['recommended-flat'],
  {
    rules: {
      'risxss/catch-potential-xss-vue': 'error',
    },
  },
)
