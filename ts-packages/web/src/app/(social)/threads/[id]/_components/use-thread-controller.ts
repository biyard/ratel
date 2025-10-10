import { useState } from 'react';
import { useThreadData } from './use-thread-data';
import { useParams } from 'react-router';
import { useLoggedIn } from '@/lib/api/hooks/users';
import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { PostDetailResponse } from '@/lib/api/ratel/posts.v3';
import { useCommentMutation } from '@/hooks/feeds/use-comment-mutation';
import { State } from '@/types/state';
import { logger } from '@/lib/logger';
import { useReplyCommentMutation } from '@/hooks/feeds/use-reply-comment-mutation';

export class ThreadController {
  constructor(
    public postId: string,
    public data,
    public expandComment: State<boolean>,
    public isLoggedIn: boolean,
    public feed: PostDetailResponse,
    public mutateComment,
    public mutateReplyToComment,
  ) {}

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

  handleLikeComment = async (commentId: string) => {
    logger.debug('handleLikeComment', commentId);
  };
}

export function useThreadController() {
  const { post_id: postId } = useParams();
  logger.debug('post id', postId);
  const isLoggedIn = useLoggedIn();
  const { data: feed } = useFeedById(postId);

  // TODO: use or define hooks
  const data = useThreadData(postId);
  const expandComment = useState(false);

  const { mutateAsync } = useCommentMutation();
  const reply = useReplyCommentMutation();

  return new ThreadController(
    postId,
    data,
    new State(expandComment),
    isLoggedIn,
    feed,
    mutateAsync,
    reply.mutateAsync,
  );
}
