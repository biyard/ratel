'use client';

import React, { useState } from 'react';
import { SpaceType } from '@/lib/api/models/spaces';
import { BoosterType } from '@/lib/api/models/notice';
import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { ArrowLeft, Internet, Fire } from '@/components/icons';
import TimeDropdown from '@/components/time-dropdown';
import CalendarDropdown from '@/components/calendar-dropdown';
import { useTranslations } from 'next-intl';
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from '@radix-ui/react-dropdown-menu';

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
  const t = useTranslations('SpaceForms');

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
    <div className="-mt-16 flex flex-col gap-4 w-full max-w-[906px] overflow-y-auto pt-1 pr-1 pb-1 pl-1 px-4 sm:px-1">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <button onClick={onBack} className="p-1 rounded-md transition-colors">
            <ArrowLeft className="w-7 h-7 text-neutral-500" />
          </button>
          {/* <h2 className="text-xl sm:text-3xl font-semibold text-white">
            {t('SetBoostModal.title')}
          </h2>
        </div>
        <button
          onClick={() => popup.close()}
          className="p-1 hover:bg-gray-800 rounded-md transition-colors"
        >
          <Remove className="w-7 h-7 text-[var(--color-neutral-500)]" />
        </button> */}
        </div>
      </div>

      {/* Warning Message */}

      <div className="text-base text-gray-300 pt-5">
        <p>
          {t.rich('SetBoostModal.description', {
            spaceType: getSpaceTypeTitle(spaceType),
            b: (chunks) => (
              <span className="font-semibold text-white">{chunks}</span>
            ),
          })}
        </p>
      </div>

      {/* Date and Time Section */}
      <div className="flex flex-col gap-3">
        <div className="flex items-center gap-1">
          <label className="text-base font-medium text-neutral-400 py-1">
            {t('date')}
          </label>
          <span className="text-red-500 text-base">*</span>
        </div>

        <div className="flex flex-col sm:flex-row sm:items-center gap-2">
          {/* Start Date and Time */}
          <div className="flex items-center gap-2">
            <CalendarDropdown
              value={startTimestamp}
              onChange={(timestamp) => {
                setStartTimestamp(timestamp);
              }}
            />
            <TimeDropdown
              value={startTimestamp}
              onChange={(timestamp) => {
                setStartTimestamp(timestamp);
              }}
            />
          </div>

          {/* Separator */}
          <div className="w-[15px] h-0.25 bg-neutral-600 self-center hidden sm:block" />
          <div className="text-center text-neutral-400 text-sm sm:hidden">
            to
          </div>

          {/* End Time and Date */}
          <div className="flex items-center gap-2">
            <CalendarDropdown
              value={endTimestamp}
              onChange={(timestamp) => {
                setEndTimestamp(timestamp);
              }}
            />
            <TimeDropdown
              value={endTimestamp}
              onChange={(timestamp) => {
                setEndTimestamp(timestamp);
              }}
            />
          </div>

          {/* Timezone */}
          {/* FIXME: use variable, not value color */}
          <div className="flex flex-row items-center w-fit border border-[#464646] rounded-lg px-5 py-[10.5px] gap-2.5 mt-2 sm:mt-0">
            <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
              Pacific Time
            </div>
            <Internet className="size-5" />
          </div>
        </div>
      </div>

      {/* Boost Section */}
      <div className="flex flex-col gap-3">
        <h3 className="text-base font-semibold text-neutral-400 py-1">
          {t('boost')}
        </h3>

        <div className="flex items-start gap-2">
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
            className="mt-0.5 w-4 h-4 rounded border-[var(--color-c-wg-70)] checked:bg-[var(--color-primary)] checked:border-[var(--color-primary)] focus:ring-[var(--color-primary)] focus:ring-offset-0 accent-[var(--color-primary)] cursor-pointer"
            style={{
              accentColor: 'var(--color-primary)',
              backgroundColor: 'inherit',
            }}
          />
          <div className="flex-1">
            <label
              htmlFor="activateBooster"
              className="text-base font-medium text-white cursor-pointer"
            >
              {t('active_booster')}
            </label>
            <p className="text-base text-neutral-300 mt-0.5">
              {t.rich('active_booster_desc', {
                btn: (chunks) => (
                  <button className="text-neutral-300 underline">
                    {chunks}
                  </button>
                ),
              })}
            </p>
          </div>
        </div>

        {formConfig.activateBooster && (
          <div className="ml-7 flex flex-col gap-2">
            <label className="text-base font-medium text-white py-1">
              {t('booster')}
            </label>

            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button
                  type="button"
                  className="border border-[var(--color-c-wg-70)] bg-transparent text-white pl-14 pr-4 py-3 rounded-md focus:outline-none focus:ring-2 focus:ring-primary w-full text-base font-medium cursor-pointer relative z-1"
                >
                  <div className="flex items-center justify-between">
                    <span>
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
                  <Fire className="absolute left-5 top-1/2 transform -translate-y-1/2 w-6 h-6 text-[var(--color-primary)] pointer-events-none" />
                </button>
              </DropdownMenuTrigger>

              <DropdownMenuContent
                align="start"
                className="w-[var(--radix-dropdown-menu-trigger-width)] bg-[var(--color-background)] border-[var(--color-c-wg-70)] text-white"
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
                        <Fire className="w-5 h-5 mr-3 text-[var(--color-primary)]" />
                        <span>Booster x {multiplier}</span>
                      </DropdownMenuItem>
                    );
                  })}
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        )}
      </div>

      {/* Create Button */}
      <div className="border-t border-[var(--color-neutral-700)]/50 pt-5 pb-1">
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
