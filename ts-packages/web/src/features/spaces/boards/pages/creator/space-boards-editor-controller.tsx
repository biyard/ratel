import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import useSpacePosts from '../../hooks/use-space-posts';
import { SpacePostResponse } from '../../types/space-post-response';
import useSpaceCategory from '../../hooks/use-space-category';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { call } from '@/lib/api/ratel/call';
import { ListSpacePostsResponse } from '../../types/list-space-posts-response';
import { useEffect, useState } from 'react';
import { State } from '@/types/state';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';

export class SpaceBoardsEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public navigate: ReturnType<typeof useNavigate>,
    public posts: State<SpacePostResponse[]>,
    public bookmark: State<string | null | undefined>,
    public t: TFunction<'SpaceBoardsEditor', undefined>,
    public categories: string[],
  ) {}

  handleCreatePage = () => {
    this.navigate(route.spaceCreatePost(this.spacePk));
  };

  handleDetailPage = (postPk: string) => {
    this.navigate(route.spaceBoardPost(this.spacePk, postPk));
  };

  changeCategory = async (categoryName: string) => {
    if (categoryName == '') {
      const next = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(this.spacePk)}/boards`,
      );

      const p = new ListSpacePostsResponse(next);
      this.posts.set(p.posts);
      this.bookmark.set(p.bookmark ?? null);
    } else {
      const next = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(this.spacePk)}/boards?category=${encodeURIComponent(categoryName)}`,
      );

      const p = new ListSpacePostsResponse(next);
      this.posts.set(p.posts);
      this.bookmark.set(p.bookmark ?? null);
    }
  };
}

export function useSpaceBoardsEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: category } = useSpaceCategory(spacePk);
  const { data: post } = useSpacePosts(spacePk);
  const { t } = useTranslation('SpaceBoardsEditor');

  const navigate = useNavigate();

  const posts = useState<SpacePostResponse[]>(post?.posts || []);
  const bookmark = useState<string | null>(post?.bookmark ?? null);

  useEffect(() => {
    posts[1](post?.posts ?? []);
    bookmark[1](post?.bookmark ?? null);
  }, [post?.bookmark, post?.posts]);

  return new SpaceBoardsEditorController(
    spacePk,
    space,
    navigate,
    new State(posts),
    new State(bookmark),
    t,
    category?.categories ?? [],
  );
}
