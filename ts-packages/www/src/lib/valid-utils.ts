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
    /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[!@#$%^&*()_+{}\[\]:;<>,.?~\\/-]).{8,}$/;
  return regex.test(password);
}

export function validateEmail(email: string) {
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return regex.test(email);
}
