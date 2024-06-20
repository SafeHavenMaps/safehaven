export function installHCaptcha() {
  const script = document.createElement('script')
  script.src = `https://js.hcaptcha.com/1/api.js?hl=fr`
  script.async = true
  script.defer = true
  document.head.appendChild(script)
}
