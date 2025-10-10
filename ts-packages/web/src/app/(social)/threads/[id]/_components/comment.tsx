import { NewComment, Comment } from '@/components/comment';
import { CommentIcon } from '@/components/icons';
import { useTranslation } from 'react-i18next';

import { ThreadController } from './use-thread-controller';

export default function ThreadComment({ ctrl }: { ctrl: ThreadController }) {
  const { t } = useTranslation('Threads');

  return (
    <>
      <div className="flex flex-col gap-2.5">
        <div className="flex flex-row text-text-primary gap-2 ">
          <CommentIcon
            width={24}
            height={24}
            className="[&>path]:stroke-text-primary [&>line]:stroke-text-primary"
          />
          <span className="text-base/6 font-medium">
            {(ctrl.feed.post.comments ?? 0).toLocaleString()}{' '}
            {(ctrl.feed.post.comments ?? 0) > 1 ? t('replies') : t('reply')}
          </span>
        </div>
        {ctrl.isLoggedIn && (
          <>
            {!ctrl.expandComment.get() && (
              <button
                onClick={() => ctrl.expandComment.set(true)}
                className="flex flex-row w-full px-3.5 py-2 gap-2 bg-write-comment-box-bg border border-write-comment-box-border items-center rounded-lg"
              >
                <CommentIcon
                  width={24}
                  height={24}
                  className="[&>path]:stroke-write-comment-box-icon"
                />
                <span className="text-write-comment-box-text text-[15px]/[24px] font-medium">
                  {t('share_your_thoughts')}
                </span>
              </button>
            )}
            {ctrl.expandComment.get() && (
              <NewComment
                onClose={() => ctrl.expandComment.set(false)}
                onSubmit={ctrl.handleComment}
              />
            )}
          </>
        )}
      </div>
      {/* TODO: Implement v3 comments rendering */}
      {ctrl.feed.comments.map((comment) => (
        <Comment
          key={comment.pk}
          comment={comment}
          onComment={ctrl.handleReplyToComment}
          onLike={ctrl.handleLikeComment}
        />
      ))}
    </>
  );
}
