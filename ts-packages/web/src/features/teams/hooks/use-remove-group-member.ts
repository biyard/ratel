import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

export interface RemoveGroupMemberRequest {
  user_pks: string[];
}

export interface RemoveGroupMemberResponse {
  total_added: number;
  failed_pks: string[];
}

async function removeGroupMember(
  teamPk: string,
  groupPk: string,
  request: RemoveGroupMemberRequest,
): Promise<RemoveGroupMemberResponse> {
  return await call(
    'DELETE',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupPk)}/member`,
    request,
  );
}

export function useRemoveGroupMember() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      teamPk,
      groupPk,
      request,
    }: {
      teamPk: string;
      groupPk: string;
      request: RemoveGroupMemberRequest;
    }): Promise<RemoveGroupMemberResponse> => {
      return await removeGroupMember(teamPk, groupPk, request);
    },
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: teamKeys.detail(variables.teamPk),
      });
      queryClient.invalidateQueries({
        queryKey: teamKeys.members(variables.teamPk),
      });
    },
  });
}
