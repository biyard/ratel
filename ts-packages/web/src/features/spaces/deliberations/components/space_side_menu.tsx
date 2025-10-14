import * as React from 'react';

import { getTimeWithFormat } from '@/lib/time-utils';
import Clock from '@/assets/icons/clock.svg?react';
import { Discuss, PieChart1, File, Vote } from '@/components/icons';
import { CheckCircle, Settings } from 'lucide-react';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation } from 'react-i18next';
import BorderSpaceCard from '@/app/(social)/_components/border-space-card';
import SetSchedulePopup from '@/app/spaces/[id]/_components/modal/set-schedule';
import { SpacePublishState } from '@/features/deliberation-space/utils/deliberation.spaces.v3';
import { useUserInfo } from '@/hooks/use-user-info';
import { DeliberationSpaceResponse } from '@/features/deliberation-space/utils/deliberation.spaces.v3';
import { useSpaceHeaderStore } from '@/app/spaces/_components/header/store';
import { TFunction } from 'i18next';
import { Deliberation } from '../types/deliberation-type';
import {
  DeliberationTab,
  DeliberationTabType,
} from '../types/deliberation-tab';

export type SpaceSideMenuProps = {
  t: TFunction<'DeliberationSpace', undefined>;
  space: DeliberationSpaceResponse;
  deliberation: Deliberation;
  selectedType: DeliberationTabType;
  handleUpdateSelectedType: (type: DeliberationTabType) => void;
  startedAt: number;
  endedAt: number;
  handleUpdateStartDate: (startDate: number) => void;
  handleUpdateEndDate: (endDate: number) => void;
};

export default function SpaceSideMenu({
  t,
  space,
  deliberation,
  selectedType,
  handleUpdateSelectedType,
  startedAt,
  endedAt,
  handleUpdateEndDate,
  handleUpdateStartDate,
}: SpaceSideMenuProps) {
  const store = useSpaceHeaderStore();
  const isEdit = store.isEditingMode;

  const popup = usePopup();
  const authorPk = space.user_pk;

  const discussions = deliberation.discussions;

  const deliberationEndedAt =
    discussions.length !== 0
      ? discussions
          .map((t) => t.ended_at)
          .reduce((latest, current) => (current > latest ? current : latest))
      : 0;

  // const selectedTeam = teams.some((t) => t.id === authorId);

  const { data: userInfo } = useUserInfo();
  const userPk = userInfo ? userInfo.pk : '';
  const createdAt = space.created_at;

  //   const writePostPermission = usePermission(
  //     space.author[0]?.id ?? 0,
  //     GroupPermission.WritePosts,
  //   ).data.has_permission;

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <BorderSpaceCard>
        <div className="flex flex-col gap-2.5 w-full">
          <div
            className={`cursor-pointer flex flex-row w-full gap-1 items-center px-1 py-2 rounded-sm ${
              selectedType == DeliberationTab.SUMMARY
                ? 'bg-neutral-800 light:bg-[#f5f5f5]'
                : ''
            }`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.SUMMARY);
            }}
          >
            <File className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-text-primary text-sm">
              {t('summary')}
            </div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${
              selectedType == DeliberationTab.DELIBERATION
                ? 'bg-neutral-800 light:bg-[#f5f5f5]'
                : ''
            }`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.DELIBERATION);
            }}
          >
            <Discuss className="w-5 h-5" />
            <div className="font-bold text-text-primary text-sm">
              {t('deliberation')}
            </div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${
              selectedType == DeliberationTab.POLL
                ? 'bg-neutral-800 light:bg-[#f5f5f5]'
                : ''
            }`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.POLL);
            }}
          >
            <Vote className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-text-primary text-sm">
              {t('poll')}
            </div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${
              selectedType == DeliberationTab.RECOMMANDATION
                ? 'bg-neutral-800 light:bg-[#f5f5f5]'
                : ''
            }`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.RECOMMANDATION);
            }}
          >
            <CheckCircle className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-text-primary text-sm">
              {t('recommendation')}
            </div>
          </div>

          {authorPk === userPk &&
            space.publish_state !== SpacePublishState.Draft.toUpperCase() && (
              <div
                className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${
                  selectedType === DeliberationTab.ANALYZE
                    ? 'bg-neutral-800 light:bg-[#f5f5f5]'
                    : ''
                }`}
                onClick={() =>
                  handleUpdateSelectedType(DeliberationTab.ANALYZE)
                }
              >
                <PieChart1 className="[&>path]:stroke-neutral-80 w-5 h-5" />
                <div className="font-bold text-text-primary text-sm">
                  {t('analyze')}
                </div>
              </div>
            )}
        </div>
      </BorderSpaceCard>
      <BorderSpaceCard>
        <div className="w-full text-sm text-white">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 light:text-neutral-800 font-semibold text-[14px]">
              <Clock width={20} height={20} />
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
                          handleUpdateStartDate(Math.floor(startDate / 1000));
                          handleUpdateEndDate(Math.floor(endDate / 1000));
                          store.onModifyContent();
                          popup.close();
                        }}
                      />,
                    )
                    .overflow(true);
                }}
              >
                <Settings
                  id="timeline-setting"
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
              { label: t('created'), date: createdAt / 1000 },
              deliberationEndedAt
                ? { label: t('deliberation'), date: deliberationEndedAt }
                : null,
              { label: t('poll_open'), date: startedAt },
              { label: t('poll_close'), date: endedAt },
            ]
              .filter(
                (item): item is { label: string; date: number } =>
                  item !== null,
              )
              .map((item) => (
                <div className="flex flex-col gap-1" key={item.label}>
                  <div className="font-medium text-text-primary text-[15px]/[12px]">
                    {item.label}
                  </div>
                  <div className="font-medium text-neutral-80 text-xs/[12px]">
                    {getTimeWithFormat(item.date ?? 0)}
                  </div>
                </div>
              ))}
          </div>
        </div>
      </BorderSpaceCard>
    </div>
  );
}

export function SpaceTabsMobile({
  space,
  selectedType,
  handleUpdateSelectedType,
}: {
  space: DeliberationSpaceResponse;
  selectedType: DeliberationTabType;
  handleUpdateSelectedType: (type: DeliberationTabType) => void;
}) {
  const { t } = useTranslation('DeliberationSpace');

  const { data: userInfo } = useUserInfo();
  const userPk = userInfo ? userInfo.pk : '';
  const authorPk = space.user_pk;
  //   const { teams } = useContext(TeamContext);
  //   const selectedTeam = teams.some((t) => t.id === authorId);
  //   const writePostPermission = usePermission(
  //     space.author[0]?.id ?? 0,
  //     GroupPermission.WritePosts,
  //   ).data.has_permission;

  const showAnalyze =
    authorPk == userPk &&
    space.publish_state !== SpacePublishState.Draft.toUpperCase();

  const wrapRef = React.useRef<HTMLDivElement | null>(null);
  const pos = React.useRef({ isDown: false, startX: 0, scrollLeft: 0 });

  const onPointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el) return;
    const target = e.target as Element;
    if (target.closest('button')) return;
    el.setPointerCapture(e.pointerId);
    pos.current.isDown = true;
    pos.current.startX = e.clientX;
    pos.current.scrollLeft = el.scrollLeft;
  };

  const onPointerMove = (e: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el || !pos.current.isDown) return;
    const dx = e.clientX - pos.current.startX;
    el.scrollLeft = pos.current.scrollLeft - dx;
  };

  const endDrag = (e?: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el) return;
    pos.current.isDown = false;
    if (e) el.releasePointerCapture(e.pointerId);
  };

  const Tab = ({
    label,
    active,
    onClick,
  }: {
    label: string;
    active: boolean;
    onClick: () => void;
  }) => (
    <button
      onClick={onClick}
      className={[
        'shrink-0 px-3 py-2 rounded-[50px] text-sm font-bold',
        active
          ? 'bg-neutral-500 light:bg-white text-text-primary border border-neutral-500 light:border-white'
          : 'bg-transparent border border-white light:border-neutral-500 text-text-primary/80',
      ].join(' ')}
    >
      {label}
    </button>
  );

  return (
    <div className="max-tablet:flex hidden w-full">
      <div
        ref={wrapRef}
        className="w-full overflow-x-auto no-scrollbar momentum cursor-grab select-none"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={endDrag}
        onPointerCancel={endDrag}
        onPointerLeave={endDrag}
      >
        <div className="flex items-center gap-2 w-max pr-3">
          <Tab
            label={t('summary')}
            active={selectedType === DeliberationTab.SUMMARY}
            onClick={() => handleUpdateSelectedType(DeliberationTab.SUMMARY)}
          />
          <Tab
            label={t('deliberation')}
            active={selectedType === DeliberationTab.DELIBERATION}
            onClick={() =>
              handleUpdateSelectedType(DeliberationTab.DELIBERATION)
            }
          />
          <Tab
            label={t('poll')}
            active={selectedType === DeliberationTab.POLL}
            onClick={() => handleUpdateSelectedType(DeliberationTab.POLL)}
          />
          <Tab
            label={t('recommendation')}
            active={selectedType === DeliberationTab.RECOMMANDATION}
            onClick={() =>
              handleUpdateSelectedType(DeliberationTab.RECOMMANDATION)
            }
          />
          {showAnalyze && (
            <Tab
              label={t('analyze')}
              active={selectedType === DeliberationTab.ANALYZE}
              onClick={() => handleUpdateSelectedType(DeliberationTab.ANALYZE)}
            />
          )}
        </div>
      </div>
    </div>
  );
}
