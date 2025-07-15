export interface EmailSignupRequest {
  email_signup: {
    nickname: string;
    email: string;
    profile_url: string;
    term_agreed: boolean;
    informed_agreed: boolean;
    username: string;
    password: string;
    telegram_raw?: string;
  };
}

export function emailSignupRequest(
  nickname: string,
  email: string,
  profile_url: string,
  term_agreed: boolean,
  informed_agreed: boolean,
  username: string,
  password: string,
  telegram_raw?: string,
): EmailSignupRequest {
  return {
    email_signup: {
      nickname,
      email,
      profile_url,
      term_agreed,
      informed_agreed,
      username,
      password,
      telegram_raw,
    },
  };
}
