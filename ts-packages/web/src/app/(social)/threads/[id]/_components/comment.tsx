'use client';

import { useFeedByID } from '@/app/(social)/_hooks/feed';
import Comment, { NewComment } from '@/components/comment';
import { CommentIcon } from '@/components/icons';
import { useLoggedIn, useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { writeCommentRequest } from '@/lib/api/models/feeds/comment';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { logger } from '@/lib/logger';
import { useState } from 'react';

export default function ThreadComment({ post_id }: { post_id: number }) {
  const isLogin = useLoggedIn();
  const { data: feed, refetch } = useFeedByID(post_id);
  const [expand, setExpand] = useState(false);
  const { post } = useApiCall();
  const { data: user } = useSuspenseUserInfo();

  const handleSubmit = async (post_id: number, content: string) => {
    try {
      await post(
        ratelApi.feeds.comment(),
        writeCommentRequest(content, user.id, post_id),
      );
      refetch();
    } catch (error) {
      logger.error('Failed to submit comment:', error);
    }
  };
  const handleLike = async (post_id: number, value: boolean) => {
    try {
      const res = await post(ratelApi.feeds.likePost(post_id), {
        like: {
          value,
        },
      });
      if (res) {
        refetch();
      }
    } catch (error) {
      logger.error('Failed to like post:', error);
    }
  };
  return (
    <>
      <div className="flex flex-col gap-2.5">
        <div className="flex flex-row text-white gap-2 ">
          <CommentIcon
            width={24}
            height={24}
            className="[&>path]:stroke-white [&>line]:stroke-white"
          />
          <span className="text-base/6 font-medium">
            {(feed?.comments ?? 0).toLocaleString()}{' '}
            {(feed?.comments ?? 0) > 1 ? 'Replies' : 'Reply'}
          </span>
        </div>
        {isLogin && (
          <>
            {!expand && (
              <button
                onClick={() => setExpand(true)}
                className="flex flex-row w-full px-3.5 py-2 gap-2 bg-neutral-800 border border-neutral-700 items-center rounded-lg"
              >
                <CommentIcon
                  width={24}
                  height={24}
                  className="[&>path]:stroke-neutral-500"
                />
                <span className="text-neutral-500 text-[15px]/[24px] font-medium">
                  Share your thoughts...
                </span>
              </button>
            )}
            {expand && (
              <NewComment
                onClose={() => setExpand(false)}
                onSubmit={async (content) =>
                  await handleSubmit(post_id, content)
                }
              />
            )}
          </>
        )}
      </div>
      {(feed?.comment_list ?? []).map((comment) => (
        <Comment
          key={comment.id}
          comment={comment}
          onSubmit={handleSubmit}
          onLike={handleLike}
        />
      ))}
    </>
  );
}
