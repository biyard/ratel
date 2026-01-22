import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

export interface UpdateGroupRequest {
  name?: string;
  description?: string;
  permissions?: number[];
}

async function updateGroup(
  teamPk: string,
  groupPk: string,
  request: UpdateGroupRequest,
): Promise<void> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupPk)}`,
    request,
  );
}

export function useUpdateGroup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      teamPk,
      groupPk,
      request,
    }: {
      teamPk: string;
      groupPk: string;
      request: UpdateGroupRequest;
    }): Promise<void> => {
      return await updateGroup(teamPk, groupPk, request);
    },
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: teamKeys.detail(variables.teamPk),
      });
    },
  });
}
