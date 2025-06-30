'use client';
import { Edit1 } from '@/components/icons';
import React from 'react';
import { usePostDraft } from './create-post';

export default function CreatePostButton() {
  const { newDraft } = usePostDraft();
  return (
    <div
      className="cursor-pointer flex flex-row w-full justify-start items-center gap-1 bg-white rounded-[100px] px-4 py-3 mb-[10px]"
      onClick={() => {
        newDraft();
      }}
    >
      <Edit1 className="w-4 h-4" />
      <div className="font-bold text-base/[22px] text-neutral-900">
        Create Post
      </div>
    </div>
  );
}
