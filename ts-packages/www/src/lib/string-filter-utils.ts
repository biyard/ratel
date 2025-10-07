//TODO: This regex does not detect the string 'ATEST'.
// Please check this is the intended behavior.
export function checkString(v: string | null | undefined): boolean {
  const safeValue = v ?? '';
  return /(?:^|\s)(test|테스트)\w*/i.test(safeValue);
}

export function validateString(v: string | null | undefined): boolean {
  const safeValue = v ?? '';
  return !/(test|테스트)/i.test(safeValue);
}

export function stripHtml(html: string): string {
  if (!html) return '';
  const div = document.createElement('div');
  div.innerHTML = html;
  return div.textContent || div.innerText || '';
}
