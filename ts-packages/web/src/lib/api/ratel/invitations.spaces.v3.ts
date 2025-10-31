import { call } from './call';

export function upsertSpaceInvitation(
  spacePk: string,
  userPks: string[],
): Promise<void> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/invitations`, {
    user_pks: userPks,
  });
}

export function verifySpaceCode(spacePk: string, code: string): Promise<void> {
  return call(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/invitations/verifications`,
    {
      code: code,
    },
  );
}
