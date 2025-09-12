'use client';

import Comment, { NewComment } from '@/components/comment';
import { CommentIcon } from '@/components/icons';
import { useLoggedIn, useSuspenseUserInfo } from '@/lib/api/hooks/users';
// import { writeCommentRequest } from '@/lib/api/models/feeds/comment';
import { useTranslations } from 'next-intl';
import { useState } from 'react';

import { useDraftMutations } from '@/hooks/feeds/use-create-feed-mutation';
import useFeedById from '@/hooks/feeds/use-feed-by-id';

export default function ThreadComment({ postId }: { postId: number }) {
  const t = useTranslations('Threads');
  const isLogin = useLoggedIn();
  const { data: feed } = useFeedById(postId);
  const [expand, setExpand] = useState(false);
  const { data: user } = useSuspenseUserInfo();
  const {
    createComment: { mutateAsync },
  } = useDraftMutations(user?.id || 0);
  const handleSubmit = async (
    postId: number,
    parentId: number,
    content: string,
  ) => {
    console.log('USER ID', user.id);
    await mutateAsync({
      userId: user.id,
      postId: postId,
      parentId: parentId,
      content: content,
    });
    setExpand(false);
  };

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
            {(feed?.comments ?? 0).toLocaleString()}{' '}
            {(feed?.comments ?? 0) > 1 ? t('replies') : t('reply')}
          </span>
        </div>
        {isLogin && (
          <>
            {!expand && (
              <button
                onClick={() => setExpand(true)}
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
            {expand && (
              <NewComment
                onClose={() => setExpand(false)}
                onSubmit={async (content) =>
                  await handleSubmit(postId, postId, content)
                }
              />
            )}
          </>
        )}
      </div>
      {(feed?.comment_list ?? []).map((comment) => (
        <Comment key={comment.id} comment={comment} onSubmit={handleSubmit} />
      ))}
    </>
  );
}
