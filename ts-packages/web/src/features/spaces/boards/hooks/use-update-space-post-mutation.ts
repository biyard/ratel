import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpacePostResponse } from '../types/space-post-response';

export function updateSpacePost(
  spacePk: string,
  postPk: string,
  title: string,
  htmlContents: string,
  categoryName: string,
  image: string | null,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(postPk)}`,
    {
      title,
      html_contents: htmlContents,
      category_name: categoryName,
      urls: image ? [image] : [],
    },
  );
}

export function useUpdateSpacePostMutation<T extends SpacePostResponse>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['update-space-post'],
    mutationFn: async ({
      spacePk,
      postPk,
      title,
      htmlContents,
      categoryName,
      image,
    }: {
      spacePk: string;
      postPk: string;
      title: string;
      htmlContents: string;
      categoryName: string;
      image: string | null;
    }) => {
      await updateSpacePost(
        spacePk,
        postPk,
        title,
        htmlContents,
        categoryName,
        image,
      );
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
    },
  });

  return mutation;
}
