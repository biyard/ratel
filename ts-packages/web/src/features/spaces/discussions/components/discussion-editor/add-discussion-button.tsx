import { Add } from '@/components/icons';
import { TFunction } from 'i18next';

export type AddDiscussionButtonProps = {
  onadd: () => void;
  t: TFunction<'SpaceDiscussionEditor', undefined>;
};

export function AddDiscussionButton({ t, onadd }: AddDiscussionButtonProps) {
  return (
    <div
      id="add-discussion-btn"
      onClick={() => {
        onadd();
      }}
      className="cursor-pointer flex flex-row w-fit px-[14px] py-[8px] gap-1 bg-white light:bg-card-bg border border-card-border rounded-[6px] hover:bg-white/80 light:hover:bg-card-bg/50"
    >
      <Add className="w-5 h-5 stroke-neutral-600 text-neutral-600" />
      <span className=" text-[#000203] font-bold text-sm">
        {t('add_discussion')}
      </span>
    </div>
  );
}
