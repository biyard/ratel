import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import useSpacePost from '../../../hooks/use-space-post';
import { useState } from 'react';
import { NavigateFunction, useNavigate } from 'react-router';
import { useCommentSpacePostMutation } from '../../../hooks/use-create-space-post-comment-mutation';
import { useCommentLikeSpacePostMutation } from '../../../hooks/use-space-post-comment-like-mutation';
import { useCommentReplySpacePostMutation } from '../../../hooks/use-space-post-comment-reply-mutation';
import { SpacePostResponse } from '../../../types/space-post-response';
import { State } from '@/types/state';
import { route } from '@/route';
import { useUserInfo } from '@/hooks/use-user-info';
import { UserResponse } from '@/lib/api/ratel/users.v3';

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
      commentSk: commentSk,
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
}

export function useSpaceBoardsViewerDetailController(
  spacePk: string,
  postPk: string,
) {
  const { data: space } = useSpaceById(spacePk);
  const { data: post } = useSpacePost(spacePk, postPk);
  const { data: user } = useUserInfo();

  const expandComment = useState(false);

  const navigate = useNavigate();
  const commentSpacePosts = useCommentSpacePostMutation();
  const commentLikeSpacePosts = useCommentLikeSpacePostMutation();
  const commentReplySpacePosts = useCommentReplySpacePostMutation();

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

    user,
  );
}
