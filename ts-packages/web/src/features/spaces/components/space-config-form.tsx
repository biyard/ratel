import { useState } from 'react';
import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { ArrowLeft, Internet, Fire } from '@/components/icons';
import TimeDropdown from '@/components/time-dropdown';
import CalendarDropdown from '@/components/calendar-dropdown';
import { useTranslation, Trans } from 'react-i18next';
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from '@radix-ui/react-dropdown-menu';
import { SpaceType } from '../types/space-type';
import { BoosterType } from '../types/booster-type';

interface SpaceConfigFormProps {
  spaceType: SpaceType;
  isLoading: boolean;
  onBack: () => void;
  onConfirm: (
    startedAt: number,
    endedAt: number,
    boosterType: BoosterType,
  ) => Promise<void>;
}

export interface SpaceConfig {
  startDate: string;
  startTime: string;
  endDate: string;
  endTime: string;
  timezone: string;
  activateBooster: boolean;
  boosterType: BoosterType;
}

export default function SpaceConfigForm({
  spaceType,
  isLoading,
  onBack,
  onConfirm,
}: SpaceConfigFormProps) {
  const { t } = useTranslation('SpaceForms');

  // Initial date setup - 1 hour from now and 2 hours from now
  const now = new Date();
  const oneHourFromNow = new Date(now.getTime() + 60 * 60 * 1000);
  const twoHoursFromNow = new Date(now.getTime() + 2 * 60 * 60 * 1000);

  const [startTimestamp, setStartTimestamp] = useState<number>(
    oneHourFromNow.getTime(),
  );
  const [endTimestamp, setEndTimestamp] = useState<number>(
    twoHoursFromNow.getTime(),
  );

  const [formConfig, setFormConfig] = useState<SpaceConfig>({
    startDate: oneHourFromNow.toISOString().split('T')[0],
    startTime: `${oneHourFromNow.getHours()}:00`,
    endDate: twoHoursFromNow.toISOString().split('T')[0],
    endTime: `${twoHoursFromNow.getHours()}:00`,
    timezone: 'Pacific Time',
    activateBooster: false,
    boosterType: BoosterType.X10,
  });

  const handleSubmit = async () => {
    // Convert milliseconds to seconds for API
    const startedAt = Math.floor(startTimestamp / 1000);
    const endedAt = Math.floor(endTimestamp / 1000);

    // Set booster type based on user selection
    const boosterType = formConfig.activateBooster
      ? formConfig.boosterType
      : BoosterType.NoBoost;

    onConfirm(startedAt, endedAt, boosterType);
  };

  const getSpaceTypeTitle = (type: SpaceType) => {
    switch (type) {
      case SpaceType.Notice:
        return 'Notice';
      case SpaceType.Deliberation:
        return 'Deliberation';
      case SpaceType.SprintLeague:
        return 'Sprint League';
      case SpaceType.Poll:
        return 'Poll';
      case SpaceType.dAgit:
        return 'd.AGIT';
      case SpaceType.Nft:
        return 'NFT';
      default:
        return 'Space';
    }
  };

  return (
    <div className="flex overflow-y-auto flex-col gap-4 px-4 pt-1 pr-1 pb-1 pl-1 -mt-16 w-full sm:px-1 max-w-[906px]">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div className="flex gap-2 items-center">
          <button onClick={onBack} className="p-1 rounded-md transition-colors">
            <ArrowLeft className="w-7 h-7 text-neutral-500" />
          </button>
          {/* <h2 className="text-xl font-semibold text-white sm:text-3xl">
            {t('SetBoostModal.title')}
          </h2>
        </div>
        <button
          onClick={() => popup.close()}
          className="p-1 rounded-md transition-colors hover:bg-gray-800"
        >
          <Remove className="w-7 h-7 text-[var(--color-neutral-500)]" />
        </button> */}
        </div>
      </div>

      {/* Warning Message */}

      <div className="flex flex-col gap-2 w-full max-mobile:h-[350px] max-mobile:overflow-y-scroll">
        <div className="pt-5 text-base text-create-space-desc">
          <p>
            <Trans
              i18nKey="SetBoostModal.description"
              ns="SpaceForms"
              values={{ spaceType: getSpaceTypeTitle(spaceType) }}
              components={{
                b: <span className="font-semibold text-desc-text" />,
              }}
            />
          </p>
        </div>

        {/* Date and Time Section */}
        <div className="flex flex-row gap-2 max-tablet:flex-wrap max-tabletitems-center">
          <div className="flex flex-col gap-2 w-full sm:flex-row sm:items-center sm:w-auto">
            <div className="w-full max-tablet:w-auto">
              <CalendarDropdown
                value={startTimestamp}
                onChange={(timestamp) => {
                  const delta = endTimestamp - startTimestamp;
                  setStartTimestamp(timestamp);
                  setEndTimestamp(timestamp + delta);
                }}
              />
            </div>
            <div className="w-full sm:w-auto">
              <TimeDropdown
                value={startTimestamp}
                onChange={(timestamp) => {
                  const delta = endTimestamp - startTimestamp;
                  setStartTimestamp(timestamp);
                  setEndTimestamp(timestamp + delta);
                }}
              />
            </div>
          </div>

          <div className="hidden self-center h-0.5 sm:block w-[15px] bg-neutral-600" />

          <div className="flex flex-col gap-2 w-full sm:flex-row sm:items-center sm:w-auto max-tablet:mt-2.5">
            <div className="w-full sm:w-auto">
              <CalendarDropdown
                value={endTimestamp}
                onChange={(timestamp) => setEndTimestamp(timestamp)}
              />
            </div>
            <div className="w-full sm:w-auto">
              <TimeDropdown
                value={endTimestamp}
                onChange={(timestamp) => setEndTimestamp(timestamp)}
              />
            </div>
          </div>

          <div className="flex flex-row gap-2.5 items-center px-5 mt-2 w-full rounded-lg border sm:mt-0 border-select-date-border bg-select-date-bg py-[10.5px] sm:w-fit">
            <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
              Pacific Time
            </div>
            <Internet
              className="w-5 h-5 [&>path]:stroke-neutral-600 [&>circle]:stroke-neutral-600"
              width="20"
              height="20"
            />
          </div>
        </div>

        {/* Boost Section */}
        <div className="flex flex-col gap-3">
          <h3 className="py-1 text-base font-semibold text-modal-label-text">
            {t('boost')}
          </h3>

          <div className="flex gap-2 items-start">
            <input
              type="checkbox"
              id="activateBooster"
              checked={formConfig.activateBooster}
              onChange={(e) =>
                setFormConfig({
                  ...formConfig,
                  activateBooster: e.target.checked,
                })
              }
              className="mt-0.5 w-4 h-4 rounded cursor-pointer focus:ring-offset-0 border-[var(--color-c-wg-70)] accent-[var(--color-primary)] checked:bg-[var(--color-primary)] checked:border-[var(--color-primary)] focus:ring-[var(--color-primary)]"
              style={{
                accentColor: 'var(--color-primary)',
                backgroundColor: 'inherit',
              }}
            />
            <div className="flex-1">
              <label
                htmlFor="activateBooster"
                className="text-base font-medium cursor-pointer text-text-primary"
              >
                {t('active_booster')}
              </label>
              <p className="mt-0.5 text-base text-desc-text">
                <Trans
                  i18nKey="active_booster_desc"
                  ns="SpaceForms"
                  components={{
                    btn: <button className="underline text-desc-text" />,
                  }}
                />
              </p>
            </div>
          </div>

          {formConfig.activateBooster && (
            <div className="flex flex-col gap-2 ml-7">
              <label className="py-1 text-base font-medium text-text-primary">
                {t('booster')}
              </label>

              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <button
                    type="button"
                    className="relative py-3 pr-4 pl-14 w-full text-base font-medium bg-transparent rounded-md border cursor-pointer focus:ring-2 focus:outline-none border-[var(--color-c-wg-70)] text-text-primary z-1 focus:ring-primary"
                  >
                    <div className="flex justify-between items-center">
                      <span className="text-text-primary">
                        Booster x{' '}
                        {Object.entries(BoosterType)
                          .find(
                            ([, value]) => value === formConfig.boosterType,
                          )?.[0]
                          ?.replace('X', '') || ''}
                      </span>
                      <svg
                        className="w-8 h-8 text-[var(--color-secondary)]"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path d="M7 10l5 5 5-5z" />
                      </svg>
                    </div>
                    <Fire className="absolute left-5 top-1/2 w-6 h-6 transform -translate-y-1/2 pointer-events-none text-[var(--color-primary)]" />
                  </button>
                </DropdownMenuTrigger>

                <DropdownMenuContent
                  align="start"
                  className="w-[var(--radix-dropdown-menu-trigger-width)] bg-[var(--color-background)] border-[var(--color-c-wg-70)] text-text-primary"
                >
                  {Object.entries(BoosterType)
                    .filter(
                      ([, value]) =>
                        typeof value === 'number' &&
                        value !== BoosterType.NoBoost,
                    )
                    .map(([key, value]) => {
                      const multiplier = key.replace('X', '');
                      return (
                        <DropdownMenuItem
                          key={value}
                          onSelect={() => {
                            setFormConfig({
                              ...formConfig,
                              boosterType: value as BoosterType,
                            });
                          }}
                          className={`px-4 py-3 hover:bg-black/10 flex items-center w-full ${formConfig.boosterType === value ? 'bg-black/10' : ''}`}
                        >
                          <Fire className="mr-3 w-5 h-5 text-[var(--color-primary)]" />
                          <span>Booster x {multiplier}</span>
                        </DropdownMenuItem>
                      );
                    })}
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          )}
        </div>
      </div>

      {/* Create Button */}
      <div className="pt-5 pb-1 border-t border-divider">
        <div className="flex justify-end">
          <LoadablePrimaryButton
            className="rounded-md w-[132px] h-[48px] flex items-center justify-center font-raleway font-bold text-[16px] leading-[100%] tracking-normal text-center cursor-pointer"
            onClick={handleSubmit}
            isLoading={isLoading}
          >
            {t('create')}
          </LoadablePrimaryButton>
        </div>
      </div>
    </div>
  );
}
