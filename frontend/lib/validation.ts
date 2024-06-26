import validator from 'validator'

export function isValidText(s: string | undefined | null) {
  return typeof s == 'string' && s.trim().length > 0
}

export function isValidRichText(s: string | undefined | null) {
  return typeof s == 'string' && s.trim().length > 0
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

export function isValidNumber(s: number | undefined | null) {
  return !Number.isNaN(s)
}
