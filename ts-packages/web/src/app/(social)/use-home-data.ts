import { useTopPromotion } from '@/lib/api/ratel/promotions.v3';
import useInfinitePosts from '../../features/posts/hooks/use-posts';

export function useHomeData() {
  const topPromotion = useTopPromotion();
  const posts = useInfinitePosts();

  return { topPromotion, posts };
}
