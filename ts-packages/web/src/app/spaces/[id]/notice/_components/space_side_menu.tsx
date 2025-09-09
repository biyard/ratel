'use client';

import SpaceBlackBox from '@/app/(social)/_components/space-black-box';
import { getTimeWithFormat } from '@/lib/time-utils';
import React from 'react';
import Clock from '@/assets/icons/clock.svg';
import History from '@/assets/icons/history.svg';
import Info from '@/assets/icons/info.svg';
import Check from '@/assets/icons/check.svg';
import Clear from '@/assets/icons/clear.svg';
import Fire from '@/assets/icons/fire.svg';
import Trophy from '@/assets/icons/trophy.svg';
import HexDown from '@/assets/icons/hex-down.svg';
import Remove from '@/assets/icons/remove.svg';
import { Add } from '@/components/icons';
import { Settings } from 'lucide-react';
// import { File, Mega } from '@/components/icons';
// import { NoticeTab } from '../types';
import { useNoticeSpace, useNoticeSpaceContext } from '../provider.client';
import { useSpaceByIdContext } from '../../providers.client';
import { useLatestQuizAttempt, useQuizAttempts } from '@/lib/api/ratel_api';
import {
  calculateBaseReward,
  calculateRewardWithPenalties,
  formatRewardAmount,
  getBoosterMultiplier,
} from '../_utils/reward-calculator';
import { usePopup } from '@/lib/contexts/popup-service';
import SetSchedulePopup from './modal/set_schedule';
import { useQuizDataRefresh } from '@/hooks/use-quiz-updates';
import { useTranslations } from 'next-intl';

export default function SpaceSideMenu() {
  const t = useTranslations('NoticeSpace');
  const popup = usePopup();
  const {
    startedAt,
    endedAt,
    isEdit,
    handleSetStartDate: setStartDate,
    handleSetEndDate: setEndDate,
  } = useNoticeSpaceContext();
  const { spaceId } = useSpaceByIdContext();
  const currentSpace = useNoticeSpace();
  const createdAt = currentSpace.created_at;

  // Get quiz attempts for current user
  const { data: attemptsData } = useQuizAttempts(spaceId || 0);

  // Get latest quiz attempt for current user
  const { data: latestAttempt } = useLatestQuizAttempt(spaceId || 0);

  // Auto-refresh quiz data when component mounts or user changes
  useQuizDataRefresh(spaceId || 0);

  // Function to calculate reward amount based on booster type (using shared utility)
  const getRewardAmount = (boosterType?: string | number): string => {
    const rewardAmount = calculateBaseReward(boosterType);
    return formatRewardAmount(rewardAmount);
  };

  // Function to get booster text based on booster type
  const getBoosterText = (boosterType?: string | number): string => {
    const multiplier = getBoosterMultiplier(boosterType);
    return multiplier === 0 ? 'None' : `x${multiplier}`;
  };

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <SpaceBlackBox>
        <div className="w-full text-sm text-foreground">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px]">
              <History width={28} height={28} />
              {t('reward')}
              {latestAttempt && attemptsData && (
                <span className="text-xs font-normal text-gray-500 ml-2">
                  (#{attemptsData.total_count} -{' '}
                  {latestAttempt.is_successful ? t('correct') : t('wrong')})
                </span>
              )}
            </div>
          </div>

          {/* Reward value content moved to top */}
          <div className="flex flex-col pl-1 gap-3 mb-3">
            {[
              {
                label: getRewardAmount(currentSpace?.booster_type),
                description: t('correct'),
                icon: Add,
              },
              {
                label: '- 50%',
                description: 'Per Incorrect',
                icon: Remove,
              },
            ].map((item) => (
              <div className="flex gap-2" key={item.label}>
                <div className="flex items-center">
                  <item.icon
                    width={24}
                    height={24}
                    className="text-neutral-500 self-center"
                  />
                </div>
                <div className="flex flex-col gap-1.5">
                  <div className="font-medium text-foreground text-[15px]/[12px]">
                    {item.label}
                  </div>
                  <div className="font-medium text-neutral-80 text-xs/[12px]">
                    {item.description}
                  </div>
                </div>
              </div>
            ))}
          </div>

          {/* Booster reward line - only show if booster type provides actual boost */}
          {(() => {
            const boosterType = currentSpace?.booster_type || 'none';
            const multiplier = getBoosterMultiplier(boosterType);

            // Don't show booster section if there's no boost (multiplier is 0)
            if (multiplier === 0) return null;

            // Map booster type values to their display names
            const boosterTypeMap: { [key: string]: string } = {
              '2': 'X2',
              '3': 'X10',
              '4': 'X100',
            };

            const displayBoosterType =
              boosterTypeMap[boosterType] || `X${boosterType}`;

            return (
              <div
                className="flex items-center justify-between rounded-[8px] px-3 py-2 mb-3"
                style={{
                  backgroundColor:
                    'color-mix(in srgb, var(--color-primary) 5%, transparent)', // 5% opacity of primary color
                }}
              >
                <div className="flex items-center gap-2">
                  <Trophy
                    width={22}
                    height={22}
                    style={{ color: 'var(--color-primary)' }}
                  />
                  <span
                    className="font-bold text-[13px]"
                    style={{ color: 'var(--color-primary)' }}
                  >
                    {displayBoosterType}
                  </span>
                </div>
                <span
                  className="font-bold text-[13px]"
                  style={{ color: 'var(--color-primary)' }}
                >
                  {t('boosting')}
                </span>
              </div>
            );
          })()}

          {/* Penalty lines - show based on failed attempts */}
          {(() => {
            const penaltyLines = [];

            // Show penalties: penalty count = failed attempts count, max 2 penalties
            // Only count unsuccessful attempts for penalties
            const failedAttempts =
              attemptsData?.items?.filter(
                (attempt) => !attempt.is_successful,
              ) || [];
            const penaltyCount = Math.min(failedAttempts.length, 2);
            for (let i = 0; i < penaltyCount; i++) {
              penaltyLines.push(
                <div
                  key={`penalty-${i}`}
                  className="flex items-center justify-between rounded-[8px] px-3 py-2 mb-3"
                  style={{
                    backgroundColor:
                      'color-mix(in srgb, var(--color-error) 5%, transparent)', // 5% opacity of error color
                  }}
                >
                  <div className="flex items-center gap-2">
                    <HexDown
                      width={22}
                      height={22}
                      className="[&>path]:stroke-[#EF4444]"
                    />
                    <span
                      className="font-bold text-[13px]"
                      style={{ color: '#EF4444' }}
                    >
                      X 0.5
                    </span>
                  </div>
                  <span
                    className="font-bold text-[13px]"
                    style={{ color: '#EF4444' }}
                  >
                    {t('penalty')}
                  </span>
                </div>,
              );
            }

            return penaltyLines;
          })()}

          {/* Total Estimated Value header at bottom */}
          <div className="flex flex-col pl-1">
            <div className="border-t border-[var(--color-neutral-700)]/50 pt-2 pb-0"></div>
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px] mb-2">
              {t('total_estimated_value')}
            </div>
            {(() => {
              // Calculate total estimated value using shared utility
              const failedAttempts =
                attemptsData?.items?.filter(
                  (attempt) => !attempt.is_successful,
                ) || [];
              const penaltyCount = Math.min(failedAttempts.length, 2);

              const finalValue = calculateRewardWithPenalties(
                currentSpace?.booster_type,
                penaltyCount,
              );

              return (
                <div className="font-medium text-foreground text-[18px]/[16px]">
                  +{finalValue.toLocaleString()} P
                </div>
              );
            })()}
          </div>
        </div>
      </SpaceBlackBox>

      <SpaceBlackBox>
        <div className="w-full text-sm text-foreground">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px]">
              <Info width={28} height={28} />
              {t('scoring_rules')}
            </div>
          </div>

          <div className="flex flex-col pl-1 gap-3">
            {[
              {
                label: getRewardAmount(currentSpace?.booster_type),
                description: t('correct_attempt'),
                icon: Check,
              },
              {
                label: '-50%',
                description: t('wrong_attempt'),
                icon: Clear,
              },
              {
                label: getBoosterText(currentSpace?.booster_type),
                description: t('boosting'),
                icon: Fire,
              },
            ].map((item) => (
              <div className="flex gap-2" key={item.label}>
                <div className="flex items-center">
                  <item.icon
                    width={24}
                    height={24}
                    className="text-neutral-500 self-center"
                  />
                </div>
                <div className="flex flex-col gap-1.5">
                  <div className="font-medium text-foreground text-[15px]/[12px]">
                    {item.label}
                  </div>
                  <div className="font-medium text-neutral-80 text-xs/[12px]">
                    {item.description}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </SpaceBlackBox>

      <SpaceBlackBox>
        <div className="w-full text-sm text-foreground">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px]">
              <Clock width={28} height={28} />
              {t('timeline')}
            </div>
            {isEdit ? (
              <div
                className="cursor-pointer w-fit h-fit"
                onClick={() => {
                  popup
                    .open(
                      <SetSchedulePopup
                        startedAt={startedAt}
                        endedAt={endedAt}
                        onconfirm={(startDate: number, endDate: number) => {
                          setStartDate(Math.floor(startDate / 1000));
                          setEndDate(Math.floor(endDate / 1000));
                          popup.close();
                        }}
                      />,
                    )
                    .overflow(true);
                }}
              >
                <Settings
                  width={20}
                  height={20}
                  className="text-neutral-500 w-5 h-5"
                />
              </div>
            ) : (
              <></>
            )}
          </div>

          <div className="flex flex-col pl-3.25 gap-5">
            {[
              { label: t('created'), date: createdAt },
              { label: t('start'), date: startedAt },
              { label: t('end'), date: endedAt },
            ].map((item) => (
              <div className="flex flex-col gap-1" key={item.label}>
                <div className="font-medium text-foreground text-[15px]/[12px]">
                  {item.label}
                </div>
                <div className="font-medium text-neutral-80 text-xs/[12px]">
                  {getTimeWithFormat(item.date ?? 0)}
                </div>
              </div>
            ))}
          </div>
        </div>
      </SpaceBlackBox>
    </div>
  );
}
