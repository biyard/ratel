import { feedKeys } from '@/constants';
import { deleteSpace } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SpaceCommon } from '@/features/spaces/types/space-common';
import { useMutation } from '@tanstack/react-query';
import { FeedStatus } from '@/lib/api/models/feeds';

export function useDeleteSpaceMutation<T extends SpaceCommon>() {
  const mutation = useMutation({
    mutationKey: ['delete-space'],
    mutationFn: async ({ spacePk }: { spacePk: string }) => {
      await deleteSpace(spacePk);
    },
    onSuccess: async (_) => {
      const feedQk = feedKeys.list({ status: FeedStatus.Published });
      await optimisticUpdate<T>({ queryKey: feedQk }, (v) => {
        return v;
      });
    },
  });

  return mutation;
}
