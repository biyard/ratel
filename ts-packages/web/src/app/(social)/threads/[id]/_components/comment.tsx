import { CommentIcon } from '@/components/icons';
import { NewComment, Comment } from '@/components/comment';

import { TFunction } from 'i18next';
import { State } from '@/types/state';
import { PostDetailResponse } from '@/features/posts/dto/post-detail-response';

export type ThreadCommentProps = {
  t: TFunction<'Thread', undefined>;
  feed: PostDetailResponse;
  isLoggedIn: boolean;
  expandComment: State<boolean>;
  handleComment: (content: string) => Promise<void>;
  handleReplyToComment: (commentSk: string, content: string) => Promise<void>;
  handleLikeComment: (commentId: string, like: boolean) => Promise<void>;
};

export default function ThreadComment({
  t,
  feed,
  isLoggedIn,
  expandComment,
  handleComment,
  handleReplyToComment,
  handleLikeComment,
}: ThreadCommentProps) {
  return (
    <>
      <div id="comments" className="flex flex-col gap-2.5">
        <div className="flex flex-row text-text-primary gap-2 ">
          <CommentIcon
            width={24}
            height={24}
            className="[&>path]:stroke-text-primary [&>line]:stroke-text-primary"
          />
          <span className="text-base/6 font-medium">
            {(feed.post.comments ?? 0).toLocaleString()}{' '}
            {(feed.post.comments ?? 0) > 1 ? t('replies') : t('reply')}
          </span>
        </div>
        {isLoggedIn && (
          <>
            {!expandComment.get() && (
              <button
                onClick={() => expandComment.set(true)}
                className="flex flex-row w-full px-3.5 py-3 gap-2 bg-write-comment-box-bg border border-write-comment-box-border items-center rounded-lg hover:bg-write-comment-box-bg/80 hover:border-primary/50 transition-all duration-200 cursor-pointer group"
              >
                <CommentIcon
                  width={24}
                  height={24}
                  className="[&>path]:stroke-write-comment-box-icon group-hover:[&>path]:stroke-primary transition-colors"
                />
                <span className="text-write-comment-box-text text-[15px]/[24px] font-medium group-hover:text-primary transition-colors">
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
      {feed.comments.map((comment) => (
        <Comment
          key={comment.pk}
          comment={comment}
          onComment={handleReplyToComment}
          onLike={handleLikeComment}
          t={t}
        />
      ))}
    </>
  );
}
