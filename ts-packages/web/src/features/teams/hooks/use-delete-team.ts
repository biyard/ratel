import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

async function deleteTeam(teamUsername: string): Promise<void> {
  return await call('DELETE', `/v3/teams/${encodeURIComponent(teamUsername)}`);
}

export function useDeleteTeam() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (teamUsername: string): Promise<void> => {
      return await deleteTeam(teamUsername);
    },
    onSuccess: (data, teamUsername) => {
      queryClient.invalidateQueries({
        queryKey: teamKeys.detail(teamUsername),
      });
      queryClient.invalidateQueries({ queryKey: teamKeys.all });
    },
  });
}
