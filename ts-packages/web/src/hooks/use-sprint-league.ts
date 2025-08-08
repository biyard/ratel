import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { useMutation } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { getQueryClient } from '@/providers/getQueryClient';
import { showErrorToast } from '@/lib/toast';
import { CreateSpaceRequest } from '@/lib/api/models/spaces';
import {
  createSprintLeagueRequest,
  SprintLeague,
} from '@/lib/api/models/sprint_league';
import { createSpace } from './use-space';
import { getQueryKey as getSpaceByIdQk } from './use-space-by-id';

export async function createSprintLeague(
  spaceId: number,
): Promise<FetchResponse<SprintLeague | null>> {
  return apiFetch<SprintLeague | null>(
    `${config.api_url}${ratelApi.sprint_league.createSprintLeague(spaceId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(createSprintLeagueRequest()),
    },
  );
}

export function useSprintLeagueSpaceMutation() {
  const queryClient = getQueryClient();

  const createMutation = useMutation({
    mutationFn: async ({ spaceReq }: { spaceReq: CreateSpaceRequest }) => {
      const { data: space } = await createSpace(spaceReq);
      if (!space) {
        throw new Error('Create space failed.');
      }
      const { data } = await createSprintLeague(space.id);
      if (!data) {
        throw new Error('Create sprint league failed.');
      }
      space.sprint_leagues = [data];
      return space;
    },
    onSuccess: (space) => {
      const qk = getSpaceByIdQk(space.id);
      queryClient.setQueryData(qk, space);
    },
    onError: (error) => {
      showErrorToast(error.message || 'Failed to create space');
    },
  });
  return { create: createMutation };
}
