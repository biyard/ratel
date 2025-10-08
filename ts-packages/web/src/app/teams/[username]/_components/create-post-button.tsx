'use client';
import { Edit1 } from '@/components/icons';
import { useTranslation } from 'react-i18next';
import { usePostEditorContext } from '@/app/(social)/_components/post-editor';
import { createPost } from '@/lib/api/ratel/posts.v3';
import { useParams } from 'react-router';

export default function CreatePostButton() {
  const { t } = useTranslation('Team');
  const p = usePostEditorContext();
  const { username } = useParams();

  if (!p) return null;

  const { openPostEditorPopup, setClose } = p;

  return (
    <div
      className="cursor-pointer flex flex-row w-full justify-start items-center gap-1 bg-create-button-bg rounded-[100px] px-4 py-3 mb-[10px]"
      onClick={async () => {
        setClose(false);
        // TODO: Update createPost to accept team username instead of team_pk
        const { post_pk } = await createPost(username as string);
        openPostEditorPopup(post_pk);
      }}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('create_post')}
      </div>
    </div>
  );
}
