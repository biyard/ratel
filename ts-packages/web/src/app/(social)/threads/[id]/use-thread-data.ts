import useSuspensePostById from '@/features/posts/hooks/use-post';

export function useThreadData(postId: string) {
  const post = useSuspensePostById(postId);

  return { post };
}
