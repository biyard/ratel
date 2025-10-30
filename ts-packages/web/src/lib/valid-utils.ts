export function checkLowerAlphaNumeric(str: string): boolean {
  return /^[a-z0-9_-]+$/.test(str);
}

export function validateNickname(userName: string) {
  const regex = /^[a-zA-Z0-9_]+$/;
  return regex.test(userName);
}

export function validatePassword(password: string) {
  if (password.length < 8) return false;
  const regex =
    /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[!@#$%^&*()_+{}\\[\]:;<>,.?~\\/-]).{8,}$/;
  return regex.test(password);
}

export function validateEmail(email: string) {
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return regex.test(email);
}

export function validateTitle(title: string) {
  const len = title.length;
  return len >= 3 && len <= 50;
}

export function validateContent(content: string) {
  const plainText = extractPlainText(content);
  const len = plainText.length;
  return len >= 10 && len <= 5000;
}

export function extractPlainText(html: string): string {
  const tempDiv = document.createElement('div');
  tempDiv.innerHTML = html;

  const images = tempDiv.querySelectorAll('img');
  images.forEach((img) => img.remove());

  const text = tempDiv.textContent || tempDiv.innerText || '';

  const textWithoutUrls = text.replace(/https?:\/\/[^\s]+/g, '');

  return textWithoutUrls.replace(/\s+/g, ' ').trim();
}
