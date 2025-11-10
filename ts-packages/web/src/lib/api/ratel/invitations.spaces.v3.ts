import { call } from './call';

export function upsertSpaceInvitation(
  spacePk: string,
  userPks: string[],
): Promise<void> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/members`, {
    user_pks: userPks,
  });
}

export function verifySpaceCode(spacePk: string): Promise<void> {
  return call(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/members/verifications`,
    {},
  );
}

export function resentVerificationCode(
  spacePk: string,
  email: string,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}/members`, {
    email,
  });
}
