import { ListPostResponse, PostResponse } from '@/lib/api/ratel/posts.v3';
import { useHomeData } from './use-home-data';
import { TopPromotionResponse } from '@/lib/api/ratel/promotions.v3';
import { useCallback } from 'react';
import { usePostEditorContext } from './_components/post-editor';
import { useObserver } from '@/hooks/use-observer';

class HomeController {
  posts: PostResponse[];
  topPromotion: TopPromotionResponse;
  close: boolean;
  isLoading: boolean;
  hasNext: boolean;

  constructor(
    public data,
    public popup,
    public handleIntersect,
    public observerRef,
  ) {
    this.posts =
      this.data.posts.data?.pages.flatMap(
        (page: ListPostResponse) => page.items,
      ) ?? [];
    this.topPromotion = this.data.topPromotion.data;
    this.close = this.popup?.close ?? true;
    this.isLoading = this.data.posts?.isLoading ?? true;
    this.hasNext = this.data.posts?.hasNextPage ?? false;
  }
}

export function useHomeController() {
  const data = useHomeData();
  const popup = usePostEditorContext();

  const { posts } = data;

  const handleIntersect = useCallback(() => {
    if (posts.hasNextPage && !posts.isFetchingNextPage) {
      posts.fetchNextPage();
    }
  }, [posts.fetchNextPage, posts.hasNextPage, posts.isFetchingNextPage]);

  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  return new HomeController(data, popup, handleIntersect, observerRef);
}
