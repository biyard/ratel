import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

async function deleteGroup(
  teamUsername: string,
  groupId: string,
): Promise<void> {
  return await call(
    'DELETE',
    `/v3/teams/${encodeURIComponent(teamUsername)}/groups/${encodeURIComponent(groupId)}`,
    {},
  );
}

export function useDeleteGroup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      teamUsername,
      groupId,
    }: {
      teamUsername: string;
      groupId: string;
      teamPk?: string;
    }): Promise<void> => {
      return await deleteGroup(teamUsername, groupId);
    },
    onSuccess: (data, variables) => {
      // Invalidate team detail
      queryClient.invalidateQueries({
        queryKey: teamKeys.detail(variables.teamUsername),
      });

      // Invalidate groups list
      if (variables.teamPk) {
        queryClient.invalidateQueries({
          queryKey: teamKeys.groups(variables.teamPk),
        });
      }
    },
  });
}
