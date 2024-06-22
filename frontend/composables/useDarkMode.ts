const localStorageKey = 'sh-dark-mode-enabled'
const isSystemDark = window.matchMedia('(prefers-color-scheme: dark)').matches

// If the key is in local storage, use it. Otherwise, use the system preference.
const isDarkModeEnabled = localStorage.getItem(localStorageKey) === 'true' || isSystemDark

export default function useDarkMode() {
  const isDark = ref(isDarkModeEnabled)
  const isLight = computed(() => !isDark.value)

  const apply = () => {
    if (isDark.value) {
      document.documentElement.classList.add('sh-dark')
    }
    else {
      document.documentElement.classList.remove('sh-dark')
    }
  }

  const toggle = () => {
    isDark.value = !isDark.value
    localStorage.setItem(localStorageKey, isDark.value.toString())
    apply()
  }

  return { isDark, isLight, apply, toggle }
}
