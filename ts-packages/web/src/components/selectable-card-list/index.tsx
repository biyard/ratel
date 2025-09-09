import { config } from '@/config';
import RadioButton from '../radio-button';

export interface CardItemProps {
  value: string;
  Icon: React.JSX.Element;
  label: string;
  description: string;
  experiment?: boolean;
  disabled?: boolean;
}

export default function SelectableCardList({
  items,
  value,
  onSelect,
}: {
  items: CardItemProps[];
  value: string | null;
  onSelect: (value: string) => void;
}) {
  return (
    <div className="flex flex-col gap-2.5 p-1.5">
      {items.map((item) => (
        <CardItem
          key={item.value}
          data={item}
          selected={value === item.value}
          onClick={() => onSelect(item.value)}
        />
      ))}
    </div>
  );
}

function CardItem({
  data,
  selected,
  onClick,
}: {
  data: CardItemProps;
  selected: boolean;
  onClick: () => void;
}) {
  const disabled = data.disabled || (data.experiment && !config.experiment);
  return (
    <div
      className={`flex flex-row gap-2.5 justify-center items-center w-full p-5 border rounded-[10px] ${selected ? 'border-primary light:bg-primary/10' : 'border-neutral-800'} ${disabled ? 'opacity-50 cursor-not-allowed' : ''}} `}
      onClick={() => {
        if (!disabled) {
          onClick();
        }
      }}
    >
      <div className="size-8 [&>svg]:size-8">{data.Icon}</div>
      <div className="flex flex-col flex-1 gap-1">
        <span className="font-bold text-[15px]/[20px] text-foreground svg">
          {data.label}
        </span>
        <span className="font-normal text-[15px]/6 min-h-12 text-neutral-400 light:text-[#525252] line-clamp-2">
          {data.description}
        </span>
      </div>
      <RadioButton selected={selected} onClick={onClick} />
    </div>
  );
}
