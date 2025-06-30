'use client';
import React from 'react';
import Comment from '@/assets/icons/comment.svg';
import { SpaceComment } from '@/lib/api/models/comments';
import { getTimeAgo } from '@/lib/time-utils';
import { useSpaceBySpaceId } from '@/app/(social)/_hooks/use-spaces';
import { checkString } from '@/lib/string-filter-utils';
import Image from 'next/image';
import { useCommitteeSpaceByIdContext } from '../providers.client';

export default function SpaceComments({ spaceId }: { spaceId: number }) {
  const { close, setClose, setExpand } = useCommitteeSpaceByIdContext();

  const space = useSpaceBySpaceId(spaceId);
  const numberOfComments = space.data.feed_comments.length;
  const comments = space.data.feed_comments;
  return (
    <div className="flex flex-col mt-[20px] gap-[20px] ">
      <div className="flex flex-row gap-2 items-center justify-start">
        <Comment width={24} height={24} className="[&>path]:stroke-white" />
        <div className="font-medium text-white text-base/[24px]">
          {numberOfComments?.toLocaleString()} Reply
        </div>
      </div>
      <CreateComment
        setClose={() => {
          setClose(!close);
          if (close) {
            setExpand(true);
          }
        }}
      />
      {comments
        ?.filter((v) => !checkString(v.html_contents))
        .map((comment, index) => (
          <CommentInfo comment={comment} key={'comment ' + index} />
        ))}
    </div>
  );
}

function CreateComment({ setClose }: { setClose: () => void }) {
  return (
    <div
      className="cursor-pointer flex flex-row w-full px-[14px] py-[12px] bg-neutral-800 border border-neutral-700 gap-[8px] items-center rounded-[8px]"
      onClick={() => {
        setClose();
      }}
    >
      <Comment width={24} height={24} className="[&>path]:stroke-neutral-500" />
      <div className="font-medium text-neutral-500 text-[15px] text-center">
        Share your thoughts...
      </div>
    </div>
  );
}

function CommentInfo({ comment }: { comment: SpaceComment }) {
  return (
    <div className="flex flex-col gap-[14px]  border-b border-b-neutral-800">
      <div className="flex flex-row gap-2 items-center">
        {comment.author?.[0]?.profile_url ? (
          <Image
            alt={comment.author?.[0]?.nickname ?? ''}
            src={comment.author?.[0]?.profile_url ?? ''}
            width={40}
            height={40}
            className="rounded-full object-cover object-top"
          />
        ) : (
          <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
        )}

        <div className="flex flex-col gap-[2px]">
          <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
            {comment.author?.[0]?.nickname ?? ''}
          </div>
          <div className="font-semibold text-xs/[20px] text-[#6d6d6d]">
            {getTimeAgo(comment.created_at)}
          </div>
        </div>
      </div>

      <div className="flex flex-col mx-10 gap-5">
        <div className="font-normal text-neutral-300 text-[15px]/[22.5px]">
          {comment.html_contents}
        </div>

        {/* <div className="flex flex-row w-full justify-end items-center gap-2">
          <ThumbUp
            width={24}
            height={24}
            className="[&>path]:stroke-[#aeaaab]"
          />
          <div className="font-medium text-base/[24px] text-[#aeaaab]">{0}</div>
        </div> */}
      </div>
    </div>
  );
}
