import validator from 'validator'

export function isValidText(s: string | undefined | null) {
  return typeof s == 'string' && s.trim().length > 0
}

export function isValidRichText(richText: string | undefined | null) {
  const textOrImageRegex = /img|>\s*[^<\s]/
  return typeof richText == 'string' && textOrImageRegex.test(richText)
}

export function isValidUrl(s: string | undefined | null) {
  return typeof s == 'string' && validator.isURL(s)
}

export function isValidEmail(s: string | undefined | null) {
  return typeof s == 'string' && validator.isEmail(s)
}

export function isValidHexColor(colorText: string | undefined | null) {
  return typeof colorText == 'string'
    && colorText[0] == '#'
    && colorText.length === 7
    && [...colorText.slice(1)].every(c => '0123456789abcdefABCDEF'.includes(c))
}

export function isValidNumber(n: number | undefined | null, limits?: { min?: number, max?: number }) {
  return typeof n == 'number'
    && !isNaN(n)
    && (limits?.min == undefined || limits.min <= n)
    && (limits?.max == undefined || limits.max >= n)
}
