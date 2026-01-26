import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

export interface CreateGroupRequest {
  name: string;
  description: string;
  image_url: string;
  permissions: number[]; // TeamGroupPermission values
}

export interface CreateGroupResponse {
  group_pk: string;
  group_sk: string;
}

async function createGroup(
  teamPk: string,
  request: CreateGroupRequest,
): Promise<CreateGroupResponse> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups`,
    request,
  );
}

export function useCreateGroup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      teamPk,
      request,
    }: {
      teamPk: string;
      request: CreateGroupRequest;
    }): Promise<CreateGroupResponse> => {
      return await createGroup(teamPk, request);
    },
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: teamKeys.detail(variables.teamPk),
      });
    },
  });
}
