import { SpaceDefinition } from '../types/space-definition';
import RadioButton from '@/components/radio-button';
import { config } from '@/config';
import { useTranslation } from 'react-i18next';

export type SpaceTypeItem = React.HTMLAttributes<HTMLDivElement> & {
  spaceDefinition: SpaceDefinition;
  selected: boolean;
  onClick: () => void;
};

export default function SpaceTypeItem({
  spaceDefinition: { Icon, labelKey, descKey, experiment },
  selected,
  onClick,
}: SpaceTypeItem) {
  const { t } = useTranslation('SpaceForms');

  if (experiment && !config.experiment) {
    return null;
  }

  return (
    <div
      aria-label={`space-setting-form-${labelKey}`}
      className={`flex flex-row gap-5 justify-center items-center w-full p-5 border rounded-[10px] transition-colors cursor-pointer hover:border-primary
              ${selected ? 'border-primary' : 'border-modal-card-border'}`}
      onClick={onClick}
    >
      {/* <div className="size-8 [&>svg]:size-8">{Icon}</div> */}
      <div className="flex flex-col flex-1 gap-1">
        <span className="font-bold text-[15px]/[20px] text-text-primary">
          {t(labelKey)}
        </span>
        <span className="font-normal text-[15px]/[24px] text-desc-text">
          {t(descKey)}
        </span>
      </div>
      <RadioButton selected={selected} onClick={onClick} />
    </div>
  );
}
