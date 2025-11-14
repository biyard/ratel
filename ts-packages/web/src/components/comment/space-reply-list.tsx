import { TiptapEditor } from '../text-editor';
import { useSpaceReplies } from '@/features/spaces/boards/hooks/use-space-replies';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { ThumbUp } from '../icons';
import { TFunction } from 'i18next';

export type SpaceReplyListProp = {
  spacePk: string;
  postPk: string;
  commentSk: string;
  isLoggedIn: boolean;
  onLike?: (commentId: string, like: boolean) => Promise<void>;
  t: TFunction<'Thread', undefined>;
};

export function SpaceReplyList({
  spacePk,
  postPk,
  commentSk,
  isLoggedIn,
  onLike,
  t,
}: SpaceReplyListProp) {
  const { data: user } = useSuspenseUserInfo();
  const { replies, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useSpaceReplies(spacePk, postPk, commentSk);

  return (
    <div className="flex flex-col gap-2.5">
      {replies.map((reply) => (
        <div
          key={reply.sk}
          className="flex flex-col gap-2 p-5 rounded-lg bg-reply-box border border-transparent"
        >
          <div className="flex flex-row gap-2 items-center">
            <img
              alt={reply.author_display_name}
              src={reply.author_profile_url}
              width={40}
              height={40}
              className="rounded-full object-cover object-top"
            />
            <div className="flex flex-col gap-[2px]">
              <div className="font-semibold text-title-text text-[15px]/[15px]">
                {reply.author_display_name ?? ''}
              </div>
            </div>
          </div>

          <TiptapEditor
            isMe={user.pk === reply.author_pk}
            content={reply.content}
            editable={false}
            showToolbar={false}
          />

          <div className="flex flex-row w-full justify-end">
            {isLoggedIn && (
              <button
                aria-label="Like Comment"
                className="flex flex-row gap-2 justify-center items-center"
                onClick={() => {
                  if (onLike) {
                    onLike(reply.sk, !reply.liked);
                  } else {
                    throw new Error('onLike is not set');
                  }
                }}
              >
                <ThumbUp
                  width={24}
                  height={24}
                  className={
                    reply.liked
                      ? '[&>path]:fill-primary [&>path]:stroke-primary'
                      : '[&>path]:stroke-comment-icon'
                  }
                />
                <div className="font-medium text-base/[24px] text-comment-icon-text ">
                  {reply.likes ?? 0}
                </div>
              </button>
            )}
          </div>
        </div>
      ))}

      {hasNextPage && (
        <div className="flex justify-center mt-1 w-full">
          <button
            type="button"
            onClick={() => fetchNextPage()}
            disabled={isFetchingNextPage}
            className="px-2 py-1 text-xs rounded-md border border-border-subtle hover:bg-bg-elevated disabled:opacity-60"
          >
            {t('reply_more')}
          </button>
        </div>
      )}
    </div>
  );
}
