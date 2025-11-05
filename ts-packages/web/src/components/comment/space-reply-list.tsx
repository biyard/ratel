import { TiptapEditor } from '../text-editor';
import { useSpaceReplies } from '@/features/spaces/boards/hooks/use-space-replies';

export type SpaceReplyListProp = {
  spacePk: string;
  postPk: string;
  commentSk: string;
};

export function SpaceReplyList({
  spacePk,
  postPk,
  commentSk,
}: SpaceReplyListProp) {
  const { data } = useSpaceReplies(spacePk, postPk, commentSk);

  const replies = data.pages.flatMap((p) => p.items);

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
            content={reply.content}
            editable={false}
            showToolbar={false}
          />
        </div>
      ))}
    </div>
  );
}
