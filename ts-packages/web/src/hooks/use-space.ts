import { QK_GET_SPACE } from '@/constants';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { useMutation } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { getQueryClient } from '@/providers/getQueryClient';
import { showErrorToast } from '@/lib/toast';
import { CreateSpaceRequest, Space } from '@/lib/api/models/spaces';
import { getQueryKey as getSpaceByIdQk } from './use-space-by-id';

export async function createSpace(
  req: CreateSpaceRequest,
): Promise<FetchResponse<Space | null>> {
  return apiFetch<Space | null>(
    `${config.api_url}${ratelApi.spaces.createSpace()}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
}

export const getQueryKey = () => [QK_GET_SPACE];

export function useFeedMutation() {
  const queryClient = getQueryClient();

  const createMutation = useMutation({
    mutationFn: async (req: CreateSpaceRequest) => {
      const { data } = await createSpace(req);
      if (!data) {
        throw new Error('Create space response did not include data.');
      }
      return data;
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
