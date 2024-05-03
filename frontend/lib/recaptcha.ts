export function installRecaptchaV3(siteKey: string) {
  const script = document.createElement('script')
  script.src = `https://www.google.com/recaptcha/api.js?render=${siteKey}`
  script.async = true
  script.defer = true
  document.head.appendChild(script)
}
