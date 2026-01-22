import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

async function deleteGroup(
  teamUsername: string,
  groupPk: string,
): Promise<void> {
  return await call(
    'DELETE',
    `/v3/teams/${encodeURIComponent(teamUsername)}/groups/${encodeURIComponent(groupPk)}`,
    {},
  );
}

export function useDeleteGroup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      teamUsername,
      groupPk,
    }: {
      teamUsername: string;
      groupPk: string;
    }): Promise<void> => {
      return await deleteGroup(teamUsername, groupPk);
    },
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: teamKeys.detail(variables.teamUsername),
      });
    },
  });
}
