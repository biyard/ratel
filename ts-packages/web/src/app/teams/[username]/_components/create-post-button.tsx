import { Edit1 } from '@/components/icons';
import { useTranslation } from 'react-i18next';

export default function CreatePostButton({}: { teamPk: string }) {
  const { t } = useTranslation('Team');

  return (
    <div
      className="cursor-pointer flex flex-row w-full justify-start items-center gap-1 bg-create-button-bg rounded-[100px] px-4 py-3 mb-[10px]"
      onClick={() => {}}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('create_post')}
      </div>
    </div>
  );
}
