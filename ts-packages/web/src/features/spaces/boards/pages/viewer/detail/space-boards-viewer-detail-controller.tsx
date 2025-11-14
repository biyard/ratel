import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import useSpacePost from '../../../hooks/use-space-post';
import { useEffect, useState } from 'react';
import { NavigateFunction, useNavigate } from 'react-router';
import { useCommentSpacePostMutation } from '../../../hooks/use-create-space-post-comment-mutation';
import { useCommentLikeSpacePostMutation } from '../../../hooks/use-space-post-comment-like-mutation';
import { useCommentReplySpacePostMutation } from '../../../hooks/use-space-post-comment-reply-mutation';
import { SpacePostResponse } from '../../../types/space-post-response';
import { State } from '@/types/state';
import { route } from '@/route';
import { useUserInfo } from '@/hooks/use-user-info';
import { UserResponse } from '@/lib/api/ratel/users.v3';
import useSpaceComments, {
  getSpaceComments,
} from '../../../hooks/use-space-comments';
import { SpacePostCommentResponse } from '../../../types/space-post-comment-response';

export class SpaceBoardsViewerDetailController {
  constructor(
    public spacePk: string,
    public postPk: string,
    public space: Space,
    public post: SpacePostResponse,
    public navigate: NavigateFunction,
    public commentSpacePosts: ReturnType<typeof useCommentSpacePostMutation>,
    public commentLikeSpacePosts: ReturnType<
      typeof useCommentLikeSpacePostMutation
    >,
    public commentReplySpacePosts: ReturnType<
      typeof useCommentReplySpacePostMutation
    >,
    public expandComment: State<boolean>,
    public user: UserResponse | null,
    public comments: State<SpacePostCommentResponse[]>,
    public bookmark: State<string | null | undefined>,
    public pages: State<SpacePostCommentResponse[][]>,
    public pageIndex: State<number>,
  ) {}

  handleBack = async () => {
    this.navigate(route.spaceBoards(this.spacePk));
  };

  handleComment = async (content: string) => {
    await this.commentSpacePosts.mutateAsync({
      spacePk: this.spacePk,
      postPk: this.postPk,
      content,
    });
    this.expandComment.set(false);
  };

  handleReplyToComment = async (commentSk: string, content: string) => {
    await this.commentReplySpacePosts.mutateAsync({
      spacePk: this.spacePk,
      postPk: this.postPk,
      commentSk,
      content,
    });
    this.expandComment.set(false);
  };

  handleLikeComment = async (commentId: string, like: boolean) => {
    this.commentLikeSpacePosts.mutateAsync({
      spacePk: this.spacePk,
      postPk: this.postPk,
      commentSk: commentId,
      like,
    });
  };

  hasPrevPage = () => {
    return this.pageIndex.get() > 0;
  };

  hasNextPage = () => {
    const idx = this.pageIndex.get();
    const pages = this.pages.get();
    return idx + 1 < pages.length || this.bookmark.get() != null;
  };

  handleNextCommentsPage = async () => {
    const idx = this.pageIndex.get();
    const pages = this.pages.get();

    if (idx + 1 < pages.length) {
      const nextIdx = idx + 1;
      this.pageIndex.set(nextIdx);
      const nextItems = pages[nextIdx] ?? [];
      this.comments.set(nextItems);
      return;
    }

    const bookmark = this.bookmark.get();
    if (!bookmark) return;

    const resp = await getSpaceComments(this.spacePk, this.postPk, bookmark);
    const items = resp.items ?? [];

    const newPages = [...pages, items];
    const nextIdx = pages.length;

    this.pages.set(newPages);
    this.pageIndex.set(nextIdx);
    this.bookmark.set(resp.bookmark ?? null);

    this.comments.set(items);
  };

  handlePrevCommentsPage = () => {
    const idx = this.pageIndex.get();
    if (idx === 0) return;

    const pages = this.pages.get();
    const prevIdx = idx - 1;

    this.pageIndex.set(prevIdx);
    const prevItems = pages[prevIdx] ?? [];
    this.comments.set(prevItems);
  };
}

export function useSpaceBoardsViewerDetailController(
  spacePk: string,
  postPk: string,
) {
  const { data: space } = useSpaceById(spacePk);
  const { data: post } = useSpacePost(spacePk, postPk);
  const { data: comment } = useSpaceComments(spacePk, postPk);
  const { data: user } = useUserInfo();

  const expandComment = useState(false);

  const navigate = useNavigate();
  const commentSpacePosts = useCommentSpacePostMutation();
  const commentLikeSpacePosts = useCommentLikeSpacePostMutation();
  const commentReplySpacePosts = useCommentReplySpacePostMutation();

  const commentsState = new State(useState<SpacePostCommentResponse[]>([]));
  const bookmarkState = new State(useState<string | null>(null));
  const pagesState = new State(useState<SpacePostCommentResponse[][]>([]));
  const pageIndexState = new State(useState(0));

  useEffect(() => {
    if (!comment) return;

    const items = comment.items ?? [];
    const bookmark = comment.bookmark ?? null;

    commentsState.set(items);
    pagesState.set([items]);
    bookmarkState.set(bookmark);
    pageIndexState.set(0);
  }, [comment]);

  return new SpaceBoardsViewerDetailController(
    spacePk,
    postPk,
    space,
    post,
    navigate,
    commentSpacePosts,
    commentLikeSpacePosts,
    commentReplySpacePosts,
    new State(expandComment),
    user ?? null,
    commentsState,
    bookmarkState,
    pagesState,
    pageIndexState,
  );
}
