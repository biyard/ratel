import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpacePostResponse } from '../types/space-post-response';

export function deleteSpacePost(
  spacePk: string,
  postPk: string,
): Promise<void> {
  return call(
    'DELETE',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(postPk)}`,
    {},
  );
}

export function useDeleteSpacePostMutation<T extends SpacePostResponse>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['delete-space-post'],
    mutationFn: async ({
      spacePk,
      postPk,
    }: {
      spacePk: string;
      postPk: string;
    }) => {
      await deleteSpacePost(spacePk, postPk);
    },
    onSuccess: async (_, { spacePk, postPk }) => {
      const spaceQK = spaceKeys.boards_posts(spacePk);
      const spacePostQk = spaceKeys.boards_post(spacePk, postPk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        return response;
      });

      qc.invalidateQueries({ queryKey: spaceKeys.boards_category(spacePk) });
      qc.invalidateQueries({
        queryKey: spacePostQk,
      });
      qc.invalidateQueries({ queryKey: spaceKeys.files(spacePk) });
      qc.invalidateQueries({ queryKey: spaceKeys.file_links(spacePk) });
    },
  });

  return mutation;
}
