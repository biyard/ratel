import { spaceKeys } from '@/constants';
import { upsertSpaceInvitation } from '@/lib/api/ratel/invitations.spaces.v3';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type Vars = {
  spacePk: string;
  user_pks: string[];
};

export function useUpsertInvitationMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ['upsert-invitation'],
    mutationFn: async (v: Vars) => {
      const { spacePk, user_pks } = v;
      await upsertSpaceInvitation(spacePk, user_pks);
      return v;
    },
    onSuccess: async (_, { spacePk }) => {
      const invitationQk = spaceKeys.invitations(spacePk);
      await queryClient.invalidateQueries({ queryKey: invitationQk });
    },
  });
}
