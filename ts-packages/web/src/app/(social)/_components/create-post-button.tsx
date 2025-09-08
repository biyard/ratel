'use client';
import { Edit1 } from '@/components/icons';
import React from 'react';
import { useLoggedIn } from '@/lib/api/hooks/users';
import { useTranslations } from 'next-intl';
import { usePostEditorContext } from './post-editor/provider';

export default function CreatePostButton() {
  const t = useTranslations('Home');
  const loggedIn = useLoggedIn();
  const { openPostEditorPopup } = usePostEditorContext();

  return (
    <div
      className="cursor-pointer flex flex-row w-full justify-start items-center gap-1 bg-white rounded-[100px] px-4 py-3 mb-[10px] aria-hidden:hidden"
      aria-hidden={!loggedIn}
      onClick={() => {
        openPostEditorPopup();
      }}
    >
      <Edit1 className="w-4 h-4" />
      <div className="font-bold text-base/[22px] text-neutral-900">
        {t('create_post')}
      </div>
    </div>
  );
}
