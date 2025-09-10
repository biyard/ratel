'use client';
import { Edit1 } from '@/components/icons';
import React from 'react';
import { usePostDraft } from './create-post';
import { useTranslations } from 'next-intl';

export default function CreatePostButton() {
  const t = useTranslations('Team');
  const { newDraft } = usePostDraft();
  return (
    <div
      className="cursor-pointer flex flex-row w-full justify-start items-center gap-1 bg-create-button-bg rounded-[100px] px-4 py-3 mb-[10px]"
      onClick={() => {
        newDraft();
      }}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('create_post')}
      </div>
    </div>
  );
}
