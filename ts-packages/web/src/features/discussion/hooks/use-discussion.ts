import { discussionKeys } from '@/constants';
import {
  DeliberationDiscussionResponse,
  getDiscussionById,
} from '@/features/discussion/utils/discussion.v3';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function getOption(spacePk: string, discussionPk: string) {
  return {
    queryKey: discussionKeys.detail(spacePk, discussionPk),
    queryFn: async () => {
      const discussion = await getDiscussionById(spacePk, discussionPk);
      return discussion;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useDiscussionById(
  spacePk: string,
  discussionPk: string,
): UseSuspenseQueryResult<DeliberationDiscussionResponse> {
  const query = useSuspenseQuery(getOption(spacePk, discussionPk));
  return query;
}
