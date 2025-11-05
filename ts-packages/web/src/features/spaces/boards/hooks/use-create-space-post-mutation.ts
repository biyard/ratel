import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpacePostResponse } from '../types/space-post-response';

export function createSpacePost(
  spacePk: string,
  title: string,
  htmlContents: string,
  categoryName: string,
  image: string | null,
): Promise<void> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/boards`, {
    title,
    html_contents: htmlContents,
    category_name: categoryName,
    urls: image ? [image] : [],
  });
}

export function useCreateSpacePostMutation<T extends SpacePostResponse>() {
  const mutation = useMutation({
    mutationKey: ['create-space-post'],
    mutationFn: async ({
      spacePk,
      title,
      htmlContents,
      categoryName,
      image,
    }: {
      spacePk: string;
      title: string;
      htmlContents: string;
      categoryName: string;
      image: string | null;
    }) => {
      await createSpacePost(spacePk, title, htmlContents, categoryName, image);
    },
    onSuccess: async (_, { spacePk }) => {
      const spaceQK = spaceKeys.boards_posts(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        return response;
      });
    },
  });

  return mutation;
}
