export interface SignupRequest {
  nickname: string;
  email: string;
  profile_url: string;
  term_agreed: boolean;
  informed_agreed: boolean;
  username: string;
  evm_address?: string;
  telegram_raw?: string;
}

export function signupRequest(
  nickname: string,
  email: string,
  profile_url: string,
  term_agreed: boolean,
  informed_agreed: boolean,
  username: string,
  evm_address?: string,
  telegram_raw?: string,
): SignupRequest {
  return {
    nickname,
    email,
    profile_url,
    term_agreed,
    informed_agreed,
    username,
    evm_address,
    telegram_raw,
  };
}
