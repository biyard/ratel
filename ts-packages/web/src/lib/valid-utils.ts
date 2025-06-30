export function checkLowerAlphaNumeric(str: string): boolean {
  return /^[a-z0-9_-]+$/.test(str);
}

export function checkEmailFormat(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}
