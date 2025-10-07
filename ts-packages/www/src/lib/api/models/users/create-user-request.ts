export interface CreateUserRequest {
  signup: {
    nickname: string;
    email: string;
    profile_url: string;
    term_agreed: boolean;
    informed_agreed: boolean;
    username: string;
  };
}

export function createUserRequest(
  nickname: string,
  email: string,
  profile_url: string,
  term_agreed: boolean,
  informed_agreed: boolean,
  username: string,
): CreateUserRequest {
  return {
    signup: {
      nickname,
      email,
      profile_url,
      term_agreed,
      informed_agreed,
      username,
    },
  };
}
