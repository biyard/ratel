import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

export interface AddMemberRequest {
  user_pks: string[];
}

export interface AddMemberResponse {
  total_added: number;
  failed_pks: string[];
}

async function addGroupMember(
  teamPk: string,
  groupPk: string,
  request: AddMemberRequest,
): Promise<AddMemberResponse> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupPk)}/member`,
    request,
  );
}

export function useAddGroupMember() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      teamPk,
      groupPk,
      request,
    }: {
      teamPk: string;
      groupPk: string;
      request: AddMemberRequest;
    }): Promise<AddMemberResponse> => {
      return await addGroupMember(teamPk, groupPk, request);
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
