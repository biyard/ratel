import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpacePostResponse } from '../types/space-post-response';
import FileModel from '../../files/types/file';

export function createSpacePost(
  spacePk: string,
  title: string,
  htmlContents: string,
  categoryName: string,
  image: string | null,
  files: FileModel[],
): Promise<void> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/boards`, {
    title,
    html_contents: htmlContents,
    category_name: categoryName,
    urls: image ? [image] : [],
    files,
  });
}

export function useCreateSpacePostMutation<T extends SpacePostResponse>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['create-space-post'],
    mutationFn: async ({
      spacePk,
      title,
      htmlContents,
      categoryName,
      image,
      files,
    }: {
      spacePk: string;
      title: string;
      htmlContents: string;
      categoryName: string;
      image: string | null;
      files: FileModel[];
    }) => {
      await createSpacePost(
        spacePk,
        title,
        htmlContents,
        categoryName,
        image,
        files,
      );
    },
    onSuccess: async (_, { spacePk }) => {
      const spaceQK = spaceKeys.boards_posts(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        return response;
      });

      qc.invalidateQueries({ queryKey: spaceKeys.boards_category(spacePk) });
    },
  });

  return mutation;
}
