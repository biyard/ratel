export const OAuthProvider = {
  Google: 'Google',
} as const;

export type OAuthProvider = typeof OAuthProvider[keyof typeof OAuthProvider];
