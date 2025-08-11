import BlackBox from '@/app/(social)/_components/black-box';
import { RewardCoin } from '@/assets/icons/money-payment';
import { Settings2 } from '@/assets/icons/settings';
import { ReactNode } from 'react';

interface RewardProps {
  amount: number;
  text: string;
}

export const colorStyles: Record<string, string> = {
  primary: 'bg-primary/5 text-primary',
};

interface ModifierProps {
  icon: ReactNode;
  multiple: number;
  text: string;
  color: keyof typeof colorStyles;
}
export default function RewardMenu({
  isEditing,
  handleSetting,
  rewardItems,
  modifierItems,
}: {
  isEditing: boolean;
  handleSetting: () => void;
  rewardItems: RewardProps[];
  modifierItems: ModifierProps[];
}) {
  return (
    <BlackBox>
      <div className="flex flex-col w-full text-neutral-500 gap-5">
        <div className="flex flex-row font-bold text-sm justify-between">
          <div className="flex flex-row gap-2">
            <RewardCoin className="size-5 [&>*]:stroke-neutral-500" />
            Rewards
          </div>
          {isEditing && (
            <button
              type="button"
              aria-label="Reward settings"
              onClick={handleSetting}
              className="cursor-pointer"
            >
              <Settings2 className="size-5 [&>*]:stroke-neutral-500" />
            </button>
          )}
        </div>

        {rewardItems.map((item) => (
          <RewardItem key={item.text} amount={item.amount} text={item.text} />
        ))}
        <div className="flex flex-col gap-2 text-sm">
          {modifierItems.map((item) => (
            <ModifierItem key={item.text} {...item} />
          ))}
        </div>
      </div>
    </BlackBox>
  );
}

function RewardItem({ amount, text }: RewardProps) {
  return (
    <div className="flex flex-row w-full gap-2.5 text-[12px]/[12px] text-neutral-500">
      <span>+</span>
      <div className="flex flex-col gap-1">
        <span className="text-white text-[15px]/[12px] font-medium">
          {amount.toLocaleString()} P
        </span>
        <span>{text}</span>
      </div>
    </div>
  );
}

function ModifierItem({ icon, multiple, text, color }: ModifierProps) {
  return (
    <div
      className={`w-full flex flex-row text-[15px]/[12px] font-bold
        justify-between items-center px-2.5 py-2.5 rounded-sm
        ${colorStyles[color] || 'bg-gray-500/5 text-gray-500'}`}
    >
      <div className="[&>svg]:size-6 flex flex-row items-center justify-center gap-2">
        {icon}
        <span>X {multiple}</span>
      </div>
      <span>{text}</span>
    </div>
  );
}
