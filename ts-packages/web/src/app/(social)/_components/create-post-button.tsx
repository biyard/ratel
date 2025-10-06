'use client';
import { Edit1 } from '@/components/icons';
import React from 'react';
import { useLoggedIn } from '@/lib/api/hooks/users';
import { useTranslations } from 'next-intl';
import { usePostEditorContext } from './post-editor/provider';
import { createPost } from '@/lib/api/ratel/posts.v3';

export default function CreatePostButton({ team_pk }: { team_pk?: string }) {
  const t = useTranslations('Home');
  const loggedIn = useLoggedIn();
  const { openPostEditorPopup, setClose } = usePostEditorContext();

  return (
    <button
      className="cursor-pointer flex flex-row w-full justify-start items-center gap-1 bg-create-button-bg rounded-[100px] px-4 py-3 mb-[10px] aria-hidden:hidden"
      aria-hidden={!loggedIn}
      onClick={async () => {
        setClose(false);
        const { post_pk } = await createPost(team_pk);
        openPostEditorPopup(post_pk);
      }}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('create_post')}
      </div>
    </button>
  );
}
