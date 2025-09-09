'use client';

import React, { useState, useEffect, useRef } from 'react';
import { createSpaceRequest, Space, SpaceType } from '@/lib/api/models/spaces';
import { BoosterType, noticeSpaceCreateRequest } from '@/lib/api/models/notice';
import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { ArrowLeft, Internet, Fire, Remove } from '@/components/icons';
import TimeDropdown from '@/components/time-dropdown';
import CalendarDropdown from '@/components/calendar-dropdown';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import { useRouter } from 'next/navigation';
import { route } from '@/route';
import { useSprintLeagueSpaceMutation } from '@/hooks/use-sprint-league';
import { useTranslations } from 'next-intl';

interface SpaceConfigFormProps {
  spaceType: SpaceType;
  feedId: number;
  onBack: () => void;
  onConfirm: () => void;
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
  feedId,
  onBack,
  onConfirm,
}: SpaceConfigFormProps) {
  const t = useTranslations('SpaceForms');
  const popup = usePopup();
  const router = useRouter();
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

  const [isLoading, setIsLoading] = useState(false);
  const [isBoosterDropdownOpen, setIsBoosterDropdownOpen] = useState(false);

  // References for click outside detection
  const dropdownRef = useRef<HTMLDivElement>(null);
  const toggleButtonRef = useRef<HTMLDivElement>(null);

  // Effect to handle clicks outside the dropdown
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        toggleButtonRef.current &&
        !dropdownRef.current.contains(event.target as Node) &&
        !toggleButtonRef.current.contains(event.target as Node)
      ) {
        setIsBoosterDropdownOpen(false);
      }
    };

    // Add event listener when dropdown is open
    if (isBoosterDropdownOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    // Clean up
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isBoosterDropdownOpen]);
  const { create } = useSprintLeagueSpaceMutation();
  const handleSubmit = async () => {
    setIsLoading(true);
    try {
      // Convert milliseconds to seconds for API
      const startedAt = Math.floor(startTimestamp / 1000);
      const endedAt = Math.floor(endTimestamp / 1000);

      // Set booster type based on user selection
      const boosterType = formConfig.activateBooster
        ? formConfig.boosterType
        : BoosterType.NoBoost;
      let data: Space | null = null;
      if (spaceType === SpaceType.SprintLeague) {
        data = await create.mutateAsync({
          spaceReq: createSpaceRequest(
            spaceType,
            feedId,
            [],
            0,
            startedAt,
            endedAt,
            boosterType,
          ),
        });
      } else {
        const res = await apiFetch<Space>(
          `${config.api_url}${ratelApi.spaces.createSpace()}`,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(
              noticeSpaceCreateRequest(
                spaceType,
                feedId,
                [],
                0,
                startedAt,
                endedAt,
                boosterType,
              ),
            ),
          },
        );
        data = res.data;
      }
      if (data) {
        logger.debug(
          `${getSpaceTypeTitle(spaceType)} space created successfully:`,
          data.id,
        );
        // Navigate to the new notice space page
        router.push(route.noticeSpaceById(data.id));
        popup.close();
        onConfirm();
      }
    } catch (error) {
      logger.error(
        `Error creating ${getSpaceTypeTitle(spaceType)} space:`,
        error,
      );
    } finally {
      setIsLoading(false);
    }
  };

  const getSpaceTypeTitle = (type: SpaceType) => {
    switch (type) {
      case SpaceType.Notice:
        return 'Notice';
      case SpaceType.Deliberation:
        return 'Deliberation';
      case SpaceType.SprintLeague:
        return 'Sprint League';
      default:
        return 'Space';
    }
  };

  return (
    <div className="flex flex-col gap-4 w-full max-w-[906px] max-h-[550px] overflow-y-auto pt-1 pr-1 pb-1 pl-1 px-4 sm:px-1">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <button onClick={onBack} className="p-1 rounded-md transition-colors">
            <ArrowLeft className="w-7 h-7 text-[var(--color-neutral-500)]" />
          </button>
          <h2 className="text-xl sm:text-3xl font-semibold text-foreground">
            {t('notice_title')}
          </h2>
        </div>
        <button
          onClick={() => popup.close()}
          className="p-1 hover:bg-gray-800 rounded-md transition-colors"
        >
          <Remove className="w-7 h-7 text-[var(--color-neutral-500)]" />
        </button>
      </div>

      {/* Warning Message */}
      <div className="text-base text-create-space-desc">
        <p>
          {t.rich('notice_description_1', {
            spaceType: getSpaceTypeTitle(spaceType),
            b: (chunks) => (
              <span className="font-semibold text-gray-300 light:text-foreground">
                {chunks}
              </span>
            ),
          })}
        </p>
        <p className="mt-0.5">{t('notice_description_2')}</p>
      </div>

      {/* Date and Time Section */}
      <div className="flex flex-col gap-3">
        <div className="flex items-center gap-1">
          <label className="text-base font-medium text-create-space-label py-1">
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
          <div className="flex flex-row items-center w-fit border border-[#525252] bg-create-space-bg light:border-create-space-border rounded-lg px-5 py-[10.5px] gap-2.5 mt-2 sm:mt-0">
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
      </div>

      {/* Boost Section */}
      <div className="flex flex-col gap-3">
        <h3 className="text-base font-semibold text-create-space-label py-1">
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
              className="text-base font-medium text-foreground cursor-pointer"
            >
              {t('active_booster')}
            </label>
            <p className="text-base text-create-space-desc mt-0.5">
              {t.rich('active_booster_desc', {
                btn: (chunks) => (
                  <button className="text-create-space-desc underline">
                    {chunks}
                  </button>
                ),
              })}
            </p>
          </div>
        </div>

        {formConfig.activateBooster && (
          <div className="ml-7 flex flex-col gap-2">
            <label className="text-base font-medium text-foreground py-1">
              {t('booster')}
            </label>
            <div className="relative">
              <div
                ref={toggleButtonRef}
                className="border border-[var(--color-c-wg-70)] bg-transparent text-foreground pl-14 pr-10 py-3 rounded-md focus:outline-none focus-within:ring-2 focus-within:ring-primary w-full text-base font-medium cursor-pointer z-10"
                onClick={() => setIsBoosterDropdownOpen(!isBoosterDropdownOpen)}
                tabIndex={0}
              >
                <div className="flex items-center justify-between">
                  <span>
                    Booster x{' '}
                    {/* Get the multiplier value from the enum key name */}
                    {Object.entries(BoosterType)
                      .find(
                        ([, value]) => value === formConfig.boosterType,
                      )?.[0]
                      ?.replace('X', '') || ''}
                  </span>
                  <svg
                    className={`w-8 h-8 text-[var(--color-secondary)] transition-transform duration-200 ${isBoosterDropdownOpen ? 'rotate-180' : ''}`}
                    fill="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path d="M7 10l5 5 5-5z" />
                  </svg>
                </div>
                <Fire className="absolute left-5 top-1/2 transform -translate-y-1/2 w-6 h-6 text-[var(--color-primary)] pointer-events-none" />

                {/* Dropdown Menu */}
                {isBoosterDropdownOpen && (
                  <div
                    ref={dropdownRef}
                    className="fixed border border-[var(--color-c-wg-70)] rounded-md overflow-hidden shadow-lg z-[9999] min-w-[200px]"
                    style={{
                      top: toggleButtonRef.current
                        ? toggleButtonRef.current.getBoundingClientRect()
                            .bottom +
                          4 +
                          window.scrollY
                        : 0,
                      left: toggleButtonRef.current
                        ? toggleButtonRef.current.getBoundingClientRect().left +
                          window.scrollX
                        : 0,
                      width: toggleButtonRef.current
                        ? toggleButtonRef.current.offsetWidth
                        : 'auto',
                      backgroundColor: 'var(--color-background)',
                    }}
                  >
                    {Object.entries(BoosterType)
                      // Filter out NoBoost and non-numeric keys (we only want the enum values)
                      .filter(
                        ([, value]) =>
                          typeof value === 'number' &&
                          value !== BoosterType.NoBoost,
                      )
                      .map(([key, value], index, array) => {
                        // Get the multiplier from the enum key (X2, X10, X100)
                        const multiplier = key.replace('X', '');

                        return (
                          <React.Fragment key={value}>
                            <div
                              className={`px-4 py-3 hover:bg-black/10 flex items-center w-full text-foreground ${formConfig.boosterType === value ? 'bg-black/10' : ''}`}
                              style={{
                                backgroundColor: 'var(--color-background)',
                              }}
                              onClick={(e) => {
                                e.stopPropagation();
                                setFormConfig({
                                  ...formConfig,
                                  boosterType: value as BoosterType,
                                });
                                setIsBoosterDropdownOpen(false);
                              }}
                            >
                              <Fire className="w-5 h-5 mr-3 text-[var(--color-primary)]" />
                              <span>Booster x {multiplier}</span>
                            </div>
                            {/* Add separator if not the last item */}
                            {index < array.length - 1 && (
                              <div
                                className="mx-0 border-t border-[var(--color-neutral-700)] h-[1px] w-full"
                                style={{
                                  backgroundColor: 'var(--color-background)',
                                }}
                              ></div>
                            )}
                          </React.Fragment>
                        );
                      })}
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Create Button */}
      <div className="border-t border-[var(--color-neutral-700)]/50 light:border-[#e5e5e5] pt-5 pb-1">
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
