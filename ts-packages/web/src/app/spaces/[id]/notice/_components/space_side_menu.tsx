'use client';

import BlackBox from '@/app/(social)/_components/black-box';
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
import { Settings } from 'lucide-react';
// import { File, Mega } from '@/components/icons';
// import { NoticeTab } from '../types';
import { useNoticeSpace, useNoticeSpaceContext } from '../provider.client';
import { useSpaceByIdContext } from '../../providers.client';
import { useLatestQuizAttempt, useQuizAttempts } from '@/lib/api/ratel_api';
import { usePopup } from '@/lib/contexts/popup-service';
import SetSchedulePopup from './modal/set_schedule';
import { useQuizDataRefresh } from '@/hooks/use-quiz-updates';

export default function SpaceSideMenu() {
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

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <BlackBox>
        <div className="w-full text-sm text-white">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px]">
              <History width={28} height={28} />
              Reward
              {latestAttempt && attemptsData && (
                <span className="text-xs font-normal text-gray-500 ml-2">
                  (#{attemptsData.total_count} -{' '}
                  {latestAttempt.is_successful ? 'Correct' : 'Wrong'})
                </span>
              )}
            </div>
          </div>

          {/* Booster reward line - only show if booster type is not 'none' */}
          {(() => {
            const boosterType = currentSpace?.booster_type || 'none';
            if (boosterType === 'none') return null;

            // Map booster type values to their display names
            const boosterTypeMap: { [key: string]: string } = {
              '1': 'NoBoost',
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
                  Boosting
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
                      style={{ color: 'var(--color-error)' }}
                    />
                    <span
                      className="font-bold text-[13px]"
                      style={{ color: 'var(--color-error)' }}
                    >
                      X 0.5
                    </span>
                  </div>
                  <span
                    className="font-bold text-[13px]"
                    style={{ color: 'var(--color-error)' }}
                  >
                    Penalty
                  </span>
                </div>,
              );
            }

            return penaltyLines;
          })()}

          <div className="flex flex-col pl-1 gap-3">
            {[
              {
                label: '+10,000 P',
                description: 'Correct',
                icon: Check,
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
                  <div className="font-medium text-white text-[15px]/[12px]">
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
      </BlackBox>

      <BlackBox>
        <div className="w-full text-sm text-white">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px]">
              <Info width={28} height={28} />
              Scoring Rules
            </div>
          </div>

          <div className="flex flex-col pl-1 gap-3">
            {[
              {
                label: '+10,000 P',
                description: 'Correct Attempt',
                icon: Check,
              },
              {
                label: '-50%',
                description: 'Wrong Attempt',
                icon: Clear,
              },
              {
                label: 'x2',
                description: 'Boosting',
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
                  <div className="font-medium text-white text-[15px]/[12px]">
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
      </BlackBox>

      <BlackBox>
        <div className="w-full text-sm text-white">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px]">
              <Clock width={28} height={28} />
              Timeline
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
              { label: 'Created', date: createdAt },
              { label: 'Start', date: startedAt },
              { label: 'End', date: endedAt },
            ].map((item) => (
              <div className="flex flex-col gap-1" key={item.label}>
                <div className="font-medium text-white text-[15px]/[12px]">
                  {item.label}
                </div>
                <div className="font-medium text-neutral-80 text-xs/[12px]">
                  {getTimeWithFormat(item.date ?? 0)}
                </div>
              </div>
            ))}
          </div>
        </div>
      </BlackBox>
    </div>
  );
}
