import { useMutation } from '@tanstack/react-query';
import { feedKeys, spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticListUpdate, optimisticUpdate } from '@/lib/hook-utils';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import {
  PollSpaceResponse,
  updatePollSpace,
} from '@/lib/api/ratel/poll.spaces.v3';
import { SurveyQuestion } from '@/types/survey-type';
import { TimeRange } from '@/types/time-range';
import { FeedStatus } from '@/lib/api/models/feeds';
import { PostResponse } from '@/lib/api/ratel/posts.v3';

export function useUpdatePollSpaceMutation() {
  const { data: user } = useSuspenseUserInfo();

  const username = user?.username;

  return useMutation({
    mutationFn: async ({
      spacePk,
      title,
      htmlContent,
      timeRange,
      questions,
    }: {
      postPk: string;
      spacePk: string;
      title: string;
      htmlContent: string;
      timeRange: TimeRange;
      questions: SurveyQuestion[];
    }) => {
      await updatePollSpace(spacePk, title, htmlContent, timeRange, questions);
      return { spacePk };
    },

    onMutate: async ({
      postPk,
      spacePk,
      title,
      htmlContent,
      timeRange,
      questions,
    }) => {
      const rollbackPost = await optimisticUpdate<PostResponse>(
        { queryKey: feedKeys.detail(postPk) },
        (post) => {
          return {
            ...post!,
            title,
            content: htmlContent,
          };
        },
      );

      const rollbackPosts = await optimisticListUpdate<PostResponse>(
        {
          queryKey: feedKeys.list({
            username,
            status: FeedStatus.Published,
          }),
        },
        (post) => {
          if (post.pk !== postPk) return post;

          return {
            ...post,
            title,
            content: htmlContent,
          };
        },
      );

      const rollbackSpace = await optimisticUpdate<PollSpaceResponse>(
        { queryKey: spaceKeys.detail(spacePk) },
        (space) => {
          return {
            ...space!,
            started_at: timeRange[0],
            ended_at: timeRange[1],
            questions,
          };
        },
      );

      return { rollbackPost, rollbackPosts, rollbackSpace };
    },

    onError: (error: Error, _variables, context) => {
      context?.rollbackPost?.rollback();
      context?.rollbackPosts?.rollback();
      context?.rollbackSpace?.rollback();

      showErrorToast(error.message || 'Failed to delete feed');
    },

    onSettled: () => {
      // TODO: Run after completed, as invalidation
      // const queryClient = getQueryClient();
      // queryClient.invalidateQueries({ queryKey });
    },
  });
}
