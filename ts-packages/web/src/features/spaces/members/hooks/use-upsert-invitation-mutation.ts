import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type Vars = {
  spacePk: string;
  new_user_pks: string[];
  removed_user_pks: string[];
};

export function useUpsertInvitationMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ['upsert-invitation'],
    mutationFn: async (v: Vars) => {
      const { spacePk, new_user_pks, removed_user_pks } = v;
      await call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/members`, {
        new_user_pks,
        removed_user_pks,
      });
      return v;
    },
    onSuccess: async (_, { spacePk }) => {
      const invitationQk = spaceKeys.invitations(spacePk);
      await queryClient.invalidateQueries({ queryKey: invitationQk });
    },
  });
}
