import { useTopPromotion } from '@/lib/api/ratel/promotions.v3';
import useInfinitePosts from './_hooks/use-infinite-posts';

export function useHomeData() {
  const topPromotion = useTopPromotion();
  const posts = useInfinitePosts();

  return { topPromotion, posts };
}
