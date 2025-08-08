import { QK_GET_SPACE } from '@/constants';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { Space, SpaceUpdateRequest } from '@/lib/api/models/spaces';
import {
  useMutation,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { getQueryClient } from '@/providers/getQueryClient';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { PublishingScope } from '@/lib/api/models/notice';
import { PublishType } from '@/components/post-header/modals/publish-space';

export async function getSpace(
  space_id: number,
): Promise<FetchResponse<Space | null>> {
  return apiFetch<Space | null>(
    `${config.api_url}${ratelApi.spaces.getSpaceBySpaceId(space_id)}`,
  );
}

export async function updateSpace(
  space_id: number,
  req: SpaceUpdateRequest,
): Promise<FetchResponse<Space | null>> {
  return apiFetch<Space | null>(
    `${config.api_url}${ratelApi.spaces.getSpaceBySpaceId(space_id)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
}

export async function shareSpace(
  space_id: number,
): Promise<FetchResponse<Space | null>> {
  const req = {
    share: {},
  };
  return apiFetch<Space | null>(
    `${config.api_url}${ratelApi.spaces.shareSpace(space_id)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
}

async function makePublicSpace(
  space_id: number,
): Promise<FetchResponse<Space | null>> {
  throw new Error(`This function is not implemented yet. ${space_id}`);
}

async function publishSpace(
  space_id: number,
  publishingScope: PublishingScope,
): Promise<FetchResponse<Space | null>> {
  throw new Error(
    `This function is not implemented yet. ${space_id} ${publishingScope}`,
  );
}

export const getQueryKey = (space_id: number) => [QK_GET_SPACE, space_id];

export function getOption(space_id: number) {
  return {
    queryKey: getQueryKey(space_id),
    queryFn: async () => {
      console.log('Fetching space with ID:', space_id, typeof space_id);
      const { data } = await getSpace(space_id);
      if (!data) {
        throw new Error('Space not found');
      }
      return data;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpaceById(
  space_id: number,
): UseSuspenseQueryResult<Space> {
  const query = useSuspenseQuery(getOption(space_id));
  return query;
}

export function useUpdateSpace(space_id: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(space_id);

  return useMutation({
    mutationFn: async (req: SpaceUpdateRequest) => {
      const { data } = await updateSpace(space_id, req);
      if (!data) {
        throw new Error('Update response did not include data.');
      }
      return data;
    },

    onMutate: async (updatedData: SpaceUpdateRequest) => {
      await queryClient.cancelQueries({ queryKey });
      const {
        update_space: { title, html_contents, started_at, ended_at },
      } = updatedData;

      const previousData = queryClient.getQueryData<Space>(queryKey);

      queryClient.setQueryData<Space>(queryKey, (old) => {
        if (!old) return undefined;
        // Only merge fields that are safe and compatible with Space type
        return {
          ...old,
          title,
          html_contents,
          started_at,
          ended_at,
        };
      });

      return { previousData };
    },

    onSuccess: (savedData: Space) => {
      logger.debug('Space updated successfully:', savedData);

      queryClient.invalidateQueries({ queryKey });

      showSuccessToast('Space updated successfully');
    },

    onError: (error: Error, _, context) => {
      if (context?.previousData) {
        queryClient.setQueryData(queryKey, context.previousData);
      }
      showErrorToast(error.message || 'Failed to update space');
    },
  });
}

export function useShareSpace(space_id: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(space_id);

  return useMutation({
    mutationFn: async () => {
      const { data } = await shareSpace(space_id);
      if (!data) {
        throw new Error('Share response did not include data.');
      }
      return data;
    },

    onMutate: async () => {
      await navigator.clipboard.writeText(window.location.href);
      await queryClient.cancelQueries({ queryKey });

      const previousData = queryClient.getQueryData<Space>(queryKey);

      queryClient.setQueryData<Space>(queryKey, (old) => {
        if (!old) return undefined;
        return { ...old, shares: old.shares + 1 };
      });
      return { previousData };
    },

    onSuccess: (savedData: Space) => {
      logger.debug('Space shared successfully:', savedData);
      queryClient.invalidateQueries({ queryKey });

      showSuccessToast('Space shared successfully');
    },
    onError: (error: Error, _, context) => {
      if (context?.previousData) {
        queryClient.setQueryData(queryKey, context.previousData);
      }
      showErrorToast(error.message || 'Failed to share space');
    },
  });
}

export function usePublishSpace(space_id: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(space_id);

  return useMutation({
    mutationFn: async (type: PublishType) => {
      const publishingScope =
        type === PublishType.Public
          ? PublishingScope.Public
          : PublishingScope.Private;
      const { data } = await publishSpace(space_id, publishingScope);
      if (!data) {
        throw new Error('Posting response did not include data.');
      }
      return data;
    },

    onMutate: async () => {
      await queryClient.cancelQueries({ queryKey });

      const previousData = queryClient.getQueryData<Space>(queryKey);

      return { previousData };
    },

    onSuccess: (savedData: Space) => {
      logger.debug('Space posted successfully:', savedData);
      queryClient.setQueryData(queryKey, savedData);

      showSuccessToast('Space posted successfully');
    },

    onError: (error: Error, _, context) => {
      if (context?.previousData) {
        queryClient.setQueryData(queryKey, context.previousData);
      }
      showErrorToast(error.message || 'Failed to post space');
    },
  });
}

export function useMakePublicSpace(space_id: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(space_id);

  return useMutation({
    mutationFn: async () => {
      const { data } = await makePublicSpace(space_id);
      if (!data) {
        throw new Error('Make public response did not include data.');
      }
      return data;
    },

    onMutate: async () => {
      await queryClient.cancelQueries({ queryKey });

      const previousData = queryClient.getQueryData<Space>(queryKey);

      queryClient.setQueryData<Space>(queryKey, (old) => {
        if (!old) return undefined;
        return { ...old, publishing_scope: PublishingScope.Public };
      });
      return { previousData };
    },

    onSuccess: (savedData: Space) => {
      logger.debug('Space made public successfully:', savedData);
      queryClient.setQueryData(queryKey, savedData);

      showSuccessToast('Space made public successfully');
    },
    onError: (error: Error, _, context) => {
      if (context?.previousData) {
        queryClient.setQueryData(queryKey, context.previousData);
      }
      showErrorToast(error.message || 'Failed to make space public');
    },
  });
}
