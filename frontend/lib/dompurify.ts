import DOMPurify from 'dompurify'

function purify_strict(txt: string): string {
  return DOMPurify.sanitize(txt, { ALLOWED_TAGS: ['b', 'q', 'i', 'strong', 'em', 'br', 'p', 'sup', 'sub', 's', 'u'], ALLOWED_ATTR: [] })
}

function purify_lenient(txt: string): string {
  return DOMPurify.sanitize(txt)
}

export { purify_strict, purify_lenient }
