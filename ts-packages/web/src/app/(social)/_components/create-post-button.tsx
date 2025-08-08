'use client';
import { Edit1 } from '@/components/icons';
import React from 'react';
import { usePostDraft } from './create-post';
import { useLoggedIn } from '@/lib/api/hooks/users';
import { useTheme } from '@/app/_providers/ThemeProvider';

export default function CreatePostButton() {
  const { theme } = useTheme();
  const loggedIn = useLoggedIn();
  const { newDraft } = usePostDraft();

  return (
    <div
      className={`cursor-pointer flex flex-row w-full justify-start items-center gap-1 rounded-[100px] px-4 py-3 mb-[10px] aria-hidden:hidden ${theme === 'light' ? 'bg-primary' : 'bg-white'}`}
      aria-hidden={!loggedIn}
      onClick={() => {
        newDraft();
      }}
    >
      <Edit1
        className={`w-4 h-4 ${theme === 'light' ? '[&>path]:stroke-neutral-800' : '[&>path]:stroke-[#737373]'}`}
      />
      <div
        className={`font-bold text-base/[22px] ${theme === 'light' ? 'text-neutral-800' : 'text-neutral-900'}`}
      >
        Create Post
      </div>
    </div>
  );
}
