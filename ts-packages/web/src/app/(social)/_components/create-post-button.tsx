'use client';
import { Edit1 } from '@/components/icons';
import React from 'react';
import { usePostDraft } from './create-post';
import { useLoggedIn } from '@/lib/api/hooks/users';

export default function CreatePostButton() {
  const loggedIn = useLoggedIn();
  const { newDraft } = usePostDraft();

  return (
    <div
      className={`cursor-pointer flex flex-row w-full justify-start items-center gap-1 rounded-[100px] px-4 py-3 mb-[10px] aria-hidden:hidden bg-white light:bg-primary`}
      aria-hidden={!loggedIn}
      onClick={() => {
        newDraft();
      }}
    >
      <Edit1
        className={`w-4 h-4 [&>path]:stroke-[#737373] light:[&>path]:stroke-neutral-800`}
      />
      <div
        className={`font-bold text-base/[22px] text-neutral-900 light:text-neutral-800`}
      >
        Create Post
      </div>
    </div>
  );
}
