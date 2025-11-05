import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import useSpacePost from '../../../hooks/use-space-post';
import { SpacePostResponse } from '../../../types/space-post-response';
import { NavigateFunction, useNavigate } from 'react-router';
import { route } from '@/route';
import { useDeleteSpacePostMutation } from '../../../hooks/use-delete-space-post-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useState } from 'react';
import { State } from '@/types/state';
import { useCommentSpacePostMutation } from '../../../hooks/use-create-space-post-comment-mutation';
import { useCommentLikeSpacePostMutation } from '../../../hooks/use-space-post-comment-like-mutation';
import { useCommentReplySpacePostMutation } from '../../../hooks/use-space-post-comment-reply-mutation';

export class SpaceBoardsEditorDetailController {
  constructor(
    public spacePk: string,
    public postPk: string,
    public space: Space,
    public post: SpacePostResponse,

    public navigate: NavigateFunction,
    public deleteSpacePosts: ReturnType<typeof useDeleteSpacePostMutation>,
    public commentSpacePosts: ReturnType<typeof useCommentSpacePostMutation>,
    public commentLikeSpacePosts: ReturnType<
      typeof useCommentLikeSpacePostMutation
    >,
    public commentReplySpacePosts: ReturnType<
      typeof useCommentReplySpacePostMutation
    >,

    public expandComment: State<boolean>,
  ) {}

  handleEditPost = async () => {
    this.navigate(route.spaceCreatePost(this.spacePk, this.postPk));
  };

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

  handleDeletePost = async () => {
    try {
      await this.deleteSpacePosts.mutateAsync({
        spacePk: this.spacePk,
        postPk: this.postPk,
      });

      showSuccessToast('Success to delete posts');
      this.navigate(route.spaceBoards(this.spacePk));
    } catch {
      showErrorToast('Failed to delete posts.');
    }
  };
}

export function useSpaceBoardsEditorDetailController(
  spacePk: string,
  postPk: string,
) {
  const { data: space } = useSpaceById(spacePk);
  const { data: post } = useSpacePost(spacePk, postPk);
  const expandComment = useState(false);

  const navigate = useNavigate();
  const deleteSpacePosts = useDeleteSpacePostMutation();
  const commentSpacePosts = useCommentSpacePostMutation();
  const commentLikeSpacePosts = useCommentLikeSpacePostMutation();
  const commentReplySpacePosts = useCommentReplySpacePostMutation();

  return new SpaceBoardsEditorDetailController(
    spacePk,
    postPk,
    space,
    post,
    navigate,

    deleteSpacePosts,
    commentSpacePosts,
    commentLikeSpacePosts,
    commentReplySpacePosts,
    new State(expandComment),
  );
}
