import { useHomeData } from './use-home-data';
import { TopPromotionResponse } from '@/lib/api/ratel/promotions.v3';
import { useCallback } from 'react';
import { useObserver } from '@/hooks/use-observer';
import PostResponse, {
  ListPostResponse,
} from '@/features/posts/dto/list-post-response';

class HomeController {
  posts: PostResponse[];
  topPromotion: TopPromotionResponse;
  close: boolean;
  isLoading: boolean;
  hasNext: boolean;

  constructor(
    public data,
    public handleIntersect,
    public observerRef,
  ) {
    this.posts =
      this.data.posts.data?.pages.flatMap(
        (page: ListPostResponse) => page.items,
      ) ?? [];
    this.topPromotion = this.data.topPromotion.data;
    this.isLoading = this.data.posts?.isLoading ?? true;
    this.hasNext = this.data.posts?.hasNextPage ?? false;
  }
}

export function useHomeController() {
  const data = useHomeData();

  const { posts } = data;

  const handleIntersect = useCallback(() => {
    if (posts.hasNextPage && !posts.isFetchingNextPage) {
      posts.fetchNextPage();
    }
  }, [posts]);

  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  return new HomeController(data, handleIntersect, observerRef);
}
