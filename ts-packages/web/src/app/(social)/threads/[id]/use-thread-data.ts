import useFeedById from '@/hooks/feeds/use-feed-by-id';

export function useThreadData(postId: string) {
  const post = useFeedById(postId);

  return { post };
}
