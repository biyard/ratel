import usePostById from '@/features/posts/hooks/use-post';

export function useThreadData(postId: string) {
  const post = usePostById(postId);

  return { post };
}
