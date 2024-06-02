import DOMPurify from 'dompurify'

function purify_strict(txt: string): string {
  DOMPurify.sanitize(txt, { ALLOWED_TAGS: ['b', 'q', 'i', 'strong', 'em', 'br', 'p'], ALLOWED_ATTR: [] })
  return DOMPurify.sanitize(txt).replaceAll('\n', '<br />')
}

function purify_lenient(txt: string): string {
  DOMPurify.sanitize(txt)
  return DOMPurify.sanitize(txt).replaceAll('\n', '<br />')
}

export { purify_strict, purify_lenient }
