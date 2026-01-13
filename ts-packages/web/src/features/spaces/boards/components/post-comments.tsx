import { CommentIcon } from '@/components/icons';
import { NewComment, Comment } from '@/components/comment';

import { TFunction } from 'i18next';
import { State } from '@/types/state';
import { SpacePostResponse } from '../types/space-post-response';
import { SpacePostCommentResponse } from '../types/space-post-comment-response';
import PostComment from '@/features/posts/types/post-comment';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';

export type PostCommentsProps = {
  t: TFunction<'Thread', undefined>;
  spacePk: string;
  post: SpacePostResponse;
  comments: SpacePostCommentResponse[];
  isFinished: boolean;
  isLoggedIn: boolean;
  expandComment: State<boolean>;
  handleCommentDelete: (commentSk: string) => Promise<void>;
  handleCommentUpdate: (commentSk: string, contents: string) => Promise<void>;
  handleComment: (content: string) => Promise<void>;
  handleReplyToComment: (commentSk: string, content: string) => Promise<void>;
  handleLikeComment: (commentId: string, like: boolean) => Promise<void>;

  hasPrevPage: boolean;
  hasNextPage: boolean;
  onPrevPage: () => void;
  onNextPage: () => void;
};

export default function PostComments({
  t,
  spacePk,
  post,
  isFinished,
  isLoggedIn,
  expandComment,
  handleCommentDelete,
  handleCommentUpdate,
  handleComment,
  handleReplyToComment,
  handleLikeComment,
  comments,

  hasPrevPage,
  hasNextPage,
  onPrevPage,
  onNextPage,
}: PostCommentsProps) {
  const { data: user } = useSuspenseUserInfo();
  const startedAt = post?.started_at ?? 0;
  const endedAt = post?.ended_at ?? 0;
  const now = Date.now();

  const toPostComment = (c: SpacePostCommentResponse): PostComment => {
    return {
      pk: c.pk,
      sk: c.sk,
      updated_at: c.updated_at,
      created_at: c.created_at,
      content: c.content,
      author_pk: c.author_pk,
      author_display_name: c.author_display_name,
      author_profile_url: c.author_profile_url,
      author_username: c.author_username,
      likes: c.likes,
      replies: c.replies,
      parent_comment_pk: c.parent_comment_sk ?? '',
      liked: c.liked,
    };
  };

  return (
    <>
      <div id="comments" className="flex flex-col gap-2.5">
        <div className="flex flex-row gap-2 text-text-primary">
          <CommentIcon
            width={24}
            height={24}
            className="[&>path]:stroke-text-primary [&>line]:stroke-text-primary"
          />
          <span className="font-medium text-base/6">
            {(post?.number_of_comments ?? 0).toLocaleString()}{' '}
            {(post?.number_of_comments ?? 0) > 1 ? t('replies') : t('reply')}
          </span>
        </div>
        {startedAt <= now && now <= endedAt && isLoggedIn && !isFinished && (
          <>
            {!expandComment.get() && (
              <button
                onClick={() => expandComment.set(true)}
                data-testid="open-new-comment-box-button"
                className="flex flex-row gap-2 items-center py-3 px-3.5 w-full rounded-lg border transition-all duration-200 cursor-pointer bg-write-comment-box-bg border-write-comment-box-border group hover:bg-write-comment-box-bg/80 hover:border-primary/50"
              >
                <CommentIcon
                  width={24}
                  height={24}
                  className="[&>path]:stroke-write-comment-box-icon group-hover:[&>path]:stroke-primary transition-colors"
                />
                <span className="font-medium transition-colors text-write-comment-box-text text-[15px]/[24px] group-hover:text-primary">
                  {t('share_your_thoughts')}
                </span>
              </button>
            )}
            {expandComment.get() && (
              <NewComment
                onClose={() => expandComment.set(false)}
                onSubmit={handleComment}
                t={t}
              />
            )}
          </>
        )}
      </div>
      {comments.map((comment) => (
        <Comment
          spacePk={spacePk}
          key={comment?.pk + ' ' + comment?.sk}
          isLoggedIn={isLoggedIn}
          comment={toPostComment(comment)}
          onComment={handleReplyToComment}
          onLike={handleLikeComment}
          onDelete={handleCommentDelete}
          onUpdate={handleCommentUpdate}
          t={t}
          canComment={
            startedAt <= now && now <= endedAt && isLoggedIn && !isFinished
          }
          canDelete={comment?.author_pk === user?.pk}
          canEdit={comment?.author_pk === user?.pk}
        />
      ))}

      <div className="flex gap-3 justify-center mt-4">
        {hasPrevPage && (
          <button
            type="button"
            onClick={onPrevPage}
            className="py-1.5 px-3 text-sm rounded-md border border-border-subtle hover:bg-bg-elevated"
          >
            {t('prev_comment')}
          </button>
        )}
        {hasNextPage && (
          <button
            type="button"
            onClick={onNextPage}
            className="py-1.5 px-3 text-sm rounded-md border border-border-subtle hover:bg-bg-elevated"
          >
            {t('next_comment')}
          </button>
        )}
      </div>
    </>
  );
}
