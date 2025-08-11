import { QK_GET_SPACE } from '@/constants';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import {
  postingSpaceRequest,
  Space,
  spaceUpdateRequest,
  SpaceUpdateRequest,
} from '@/lib/api/models/spaces';
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
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { route } from '@/route';

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

async function updateSpaceScope(
  spaceId: number,
  prevSpace: Space,
  scope: PublishingScope = PublishingScope.Private,
): Promise<FetchResponse<Space | null>> {
  const req = spaceUpdateRequest(
    prevSpace.html_contents,
    prevSpace.files,
    //FIXME: This should be updated to use the correct type for the space
    [],
    [],
    [],
    [],
    prevSpace.title,
    prevSpace.started_at,
    prevSpace.ended_at,
    scope,
    null,
  );
  return apiFetch<Space | null>(
    `${config.api_url}${ratelApi.spaces.getSpaceBySpaceId(spaceId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
}

export async function publishSpace(
  spaceId: number,
): Promise<FetchResponse<Space | null>> {
  return apiFetch<null>(
    `${config.api_url}${ratelApi.spaces.getSpaceBySpaceId(spaceId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(postingSpaceRequest()),
    },
  );
}

export const getQueryKey = (spaceId: number) => [QK_GET_SPACE, spaceId];

export function getOption(spaceId: number) {
  return {
    queryKey: getQueryKey(spaceId),
    queryFn: async () => {
      const { data } = await getSpace(spaceId);
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

export function useUpdateSpace(spaceId: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(spaceId);

  return useMutation({
    mutationFn: async (req: SpaceUpdateRequest) => {
      const { data } = await updateSpace(spaceId, req);
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
        if (!old) return old;
        const patch: Partial<Space> = {};
        if (title !== undefined) patch.title = title;
        if (html_contents !== undefined) patch.html_contents = html_contents;
        if (started_at !== undefined) patch.started_at = started_at;
        if (ended_at !== undefined) patch.ended_at = ended_at;
        return { ...old, ...patch };
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
  const { data: userInfo } = useUserInfo();

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
      let url = `${window.location.origin}${route.space(space_id)}`;
      if (userInfo?.referral_code) {
        url += `?referral=${userInfo.referral_code}`;
      }
      try {
        await navigator.clipboard.writeText(url);
      } catch {
        logger.debug('Clipboard API unavailable; falling back to prompt');
        window.prompt('Copy this link', url);
      }

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

export function usePublishSpace(spaceId: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(spaceId);

  return useMutation({
    mutationFn: async (type: PublishType) => {
      const publishingScope =
        type === PublishType.Public
          ? PublishingScope.Public
          : PublishingScope.Private;

      const space = getQueryClient().getQueryData<Space>(getQueryKey(spaceId));
      if (!space) {
        throw new Error('No space data available for publishing.');
      }

      await publishSpace(spaceId);

      const { data } = await updateSpaceScope(spaceId, space, publishingScope);
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
      queryClient.invalidateQueries({ queryKey });
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

export function useMakePublicSpace(spaceId: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(spaceId);

  return useMutation({
    mutationFn: async () => {
      const space = queryClient.getQueryData<Space>(queryKey);
      if (!space) {
        throw new Error('No space data available for publishing.');
      }
      const { data } = await updateSpaceScope(
        spaceId,
        space,
        PublishingScope.Public,
      );
      if (!data) {
        throw new Error('Make public response did not include data.');
      }
      return data;
    },

    onMutate: async () => {
      await queryClient.cancelQueries({ queryKey });

      const previousData = queryClient.getQueryData<Space>(queryKey);

      return { previousData };
    },

    onSuccess: (savedData: Space) => {
      logger.debug('Space made public successfully:', savedData);
      queryClient.invalidateQueries({ queryKey });
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
