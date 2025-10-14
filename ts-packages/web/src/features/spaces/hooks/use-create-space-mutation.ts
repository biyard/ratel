import { feedKeys } from '@/constants';
import { PostDetailResponse } from '@/lib/api/ratel/posts.v3';
import { createSpace } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { BoosterType } from '@/features/spaces/types/booster-type';
import { SpaceType } from '@/features/spaces/types/space-type';
import { useMutation } from '@tanstack/react-query';

export function useCreateSpaceMutation() {
  const mutation = useMutation({
    mutationKey: ['create-space'],
    mutationFn: async ({
      postPk,
      spaceType,
      startedAt,
      endedAt,
      booster,
    }: {
      postPk: string;
      spaceType: SpaceType;
      startedAt: number | null;
      endedAt: number | null;
      booster: BoosterType | null;
    }) => {
      const res = await createSpace(
        postPk,
        spaceType,
        startedAt,
        endedAt,
        booster,
      );
      return res;
    },
    onSuccess: async (res, { postPk, booster, spaceType }) => {
      const { space_pk } = res;
      // Update Post
      const postQk = feedKeys.detail(postPk);
      await optimisticUpdate<PostDetailResponse>(
        { queryKey: postQk },
        (post) => {
          post.post.booster = booster;
          post.post.space_pk = space_pk;
          post.post.space_type = spaceType;

          return post;
        },
      );
    },
  });

  return mutation;
}
