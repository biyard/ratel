import { useDeletePostMutation } from '@/features/posts/hooks/use-delete-post-mutation';
import { useLikePostMutation } from '@/features/posts/hooks/use-like-post-mutation';
import { useCommentMutation } from '@/hooks/feeds/use-comment-mutation';
import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { useReplyCommentMutation } from '@/features/comments/hooks/use-reply-comment-mutation';
import { useLoggedIn, useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { FeedStatus } from '@/lib/api/models/feeds';
import { GroupPermission } from '@/lib/api/models/group';
import { PostDetailResponse } from '@/lib/api/ratel/posts.v3';
import { usePopup } from '@/lib/contexts/popup-service';
import { TeamContext } from '@/lib/contexts/team-context';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import { State } from '@/types/state';
import { useContext, useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate, useParams } from 'react-router';
import { usePostEditorContext } from '../../_components/post-editor';
import SpaceCreateModal from '../../../../features/spaces/components/space-create-modal';
import { useThreadData } from './use-thread-data';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';
import { useLikeCommentMutation } from '@/features/comments/hooks/use-like-comment-mutation';

export class ThreadController {
  readonly isPostOwner: boolean;
  readonly username: string;
  permissions: TeamGroupPermissions;
  readonly canEdit: boolean;
  readonly canDelete: boolean;

  constructor(
    public postId: string,
    public data,
    public expandComment: State<boolean>,
    public isLoggedIn: boolean,
    public feed: PostDetailResponse,
    public mutateComment,
    public mutateReplyToComment,
    public popup,
    public t,
    public deletePost,
    public navigate,
    public likePost,
    public user,
    public teams,
    public postEditor,
    public likeComment,
  ) {
    this.username = this.user?.username || '';
    this.isPostOwner =
      this.feed.post.author_username === this.username ||
      this.teams.find(
        (team) => team.username === this.feed.post.author_username,
      );
    this.permissions = new TeamGroupPermissions(this.feed.permissions);
    this.canEdit =
      this.isPostOwner || this.permissions.has(GroupPermission.WritePosts);
    this.canDelete =
      this.isPostOwner || this.permissions.has(GroupPermission.DeletePosts);
    logger.debug('ThreadController', this);
  }

  handleComment = async (content: string) => {
    logger.debug('handleComment', this.postId);
    await this.mutateComment({ postPk: this.postId, content });
    this.expandComment.set(false);
  };

  handleReplyToComment = async (commentSk: string, content: string) => {
    logger.debug('handleReplyToComment', this.postId);
    await this.mutateReplyToComment({
      postPk: this.postId,
      commentSk,
      content,
    });
  };

  handleLikeComment = async (commentId: string, like: boolean) => {
    logger.debug('handleLikeComment', commentId);
    this.likeComment.mutateAsync({
      postPk: this.postId,
      commentSk: commentId,
      like,
    });
  };

  handleLikePost = async () => {
    logger.debug('handleLikePost', this.postId);
    if (!this.likePost.isPending) {
      await this.likePost.mutateAsync({
        feedId: this.postId,
        like: !this.feed.is_liked,
      });
    }
  };

  handleEditPost = async () => {
    logger.debug('handleEditPost', this.postId);
    await this.postEditor?.openPostEditorPopup(this.postId);
  };

  handleDeletePost = async () => {
    logger.debug('handleDeletePost', this.postId);
    if (!this.deletePost.isPending) {
      await this.deletePost.mutateAsync(this.postId);
      this.navigate(route.home());
    }
  };

  handleCreateSpace = async () => {
    logger.debug('handleCreateSpace');
    this.popup
      .open(<SpaceCreateModal feed_id={this.postId} />)
      .withoutBackdropClose()
      .withTitle(this.t('select_space_type'));
  };

  goBack = () => {
    this.navigate(-1);
  };
}

export function useThreadController() {
  const { post_id: postId } = useParams();
  logger.debug('post id', postId);
  const { data: user } = useSuspenseUserInfo();
  const isLoggedIn = useLoggedIn();
  const { data: feed } = useFeedById(postId);

  const data = useThreadData(postId);
  const expandComment = useState(false);

  const { mutateAsync } = useCommentMutation();
  const reply = useReplyCommentMutation();

  const popup = usePopup();

  const { t } = useTranslation('Threads');
  const deletePost = useDeletePostMutation(
    user?.username || '',
    FeedStatus.Published,
  );
  const navigate = useNavigate();
  const likePost = useLikePostMutation();
  const { teams } = useContext(TeamContext);

  const postEditor = usePostEditorContext();
  const likeComment = useLikeCommentMutation();

  useEffect(() => {
    if (feed.post.space_pk) {
      navigate(route.spaceByType(feed.post.space_type, feed.post.space_pk));
    }
  }, [feed, navigate]);

  return new ThreadController(
    postId,
    data,
    new State(expandComment),
    isLoggedIn,
    feed,
    mutateAsync,
    reply.mutateAsync,
    popup,
    t,
    deletePost,
    navigate,
    likePost,
    user,
    teams,
    postEditor,
    likeComment,
  );
}
