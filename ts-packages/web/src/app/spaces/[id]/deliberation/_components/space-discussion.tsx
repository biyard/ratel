'use client';

import Image from 'next/image';
import React, { useEffect, useMemo, useRef, useState } from 'react';
import { format } from 'date-fns';

import { v4 as uuidv4 } from 'uuid';
import discussionImg from '@/assets/images/discussion.png';
import { Member } from '@/lib/api/models/discussion';
import { Add } from './add';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { ArrowRight } from 'lucide-react';
import { useRouter } from 'next/navigation';
import { route } from '@/route';
import { Extra2 } from '@/components/icons';
import { DiscussionInfo } from '../types';
import { TotalUser } from '@/lib/api/models/user';
import { usePopup } from '@/lib/contexts/popup-service';
import NewDiscussion from './modal/new-discussion';
import {
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from '../provider.client';
import { useTranslations } from 'next-intl';
import BorderSpaceCard from '@/app/(social)/_components/border-space-card';

export default function SpaceDiscussion() {
  const { isEdit } = useDeliberationSpaceContext();

  return (
    <div className="flex flex-col w-full">
      {isEdit ? <EditableDiscussion /> : <ViewDiscussion />}
    </div>
  );
}

function ViewDiscussion() {
  return (
    <div className="flex flex-col w-full gap-2.5">
      <DiscussionSchedules />
    </div>
  );
}

function DiscussionSchedules() {
  const t = useTranslations('DeliberationSpace');
  const { status, handleViewRecord } = useDeliberationSpaceContext();
  // TODO: Update to use v3 user API with string pk instead of numeric id
  const userId = 0;

  const discussions = useDeliberationSpace().discussions;

  const router = useRouter();

  const handleMoveDiscussion = (spaceId: number, discussionId: number) => {
    router.push(route.discussionById(spaceId, discussionId));
  };

  return (
    <div className="flex flex-col gap-2.5">
      <BorderSpaceCard>
        <div className="flex flex-col w-full gap-5">
          <div className="font-bold text-text-primary text-[15px]/[20px]">
            {t('discussions')}
          </div>
          <div className="flex flex-col w-full gap-2.5">
            {discussions.map((discussion, index) => (
              <React.Fragment key={index}>
                <DiscussionRoom
                  userId={userId}
                  status={status}
                  startDate={discussion.started_at}
                  endDate={discussion.ended_at}
                  title={discussion.name}
                  description={discussion.description}
                  members={discussion.members}
                  record={discussion.record}
                  onclick={() => {
                    handleMoveDiscussion(discussion.space_id, discussion.id);
                  }}
                  viewRecordClick={() => {
                    handleViewRecord(discussion.id, discussion.record ?? '');
                  }}
                />
                {index !== discussions.length - 1 ? (
                  <div className=" w-full h-0.25 gap-1 bg-divider" />
                ) : (
                  <></>
                )}
              </React.Fragment>
            ))}
          </div>
        </div>
      </BorderSpaceCard>
    </div>
  );
}

export function DiscussionRoom({
  userId,
  startDate,
  endDate,
  title,
  status,
  description,
  members,
  record,
  onclick,
  viewRecordClick,
}: {
  userId: number;
  status: SpaceStatus;
  startDate: number;
  endDate: number;
  title: string;
  description: string;
  record?: string;
  members: Member[];

  onclick: () => void;
  viewRecordClick: () => void;
}) {
  const t = useTranslations('DeliberationSpace');
  const now = Math.floor(Date.now() / 1000);

  const isLive = now >= startDate && now <= endDate;
  const isUpcoming = now < startDate;
  const isFinished = now > endDate;

  const formattedDate = `${format(new Date(startDate * 1000), 'dd MMM, yyyy HH:mm')} - ${format(new Date(endDate * 1000), 'dd MMM, yyyy HH:mm')}`;

  const statusLabel = isUpcoming
    ? t('upcoming_discussion')
    : isFinished
      ? t('finished_discussion')
      : t('ongoing_discussion');

  const isMember = members.some((member) => member.id === userId);

  return (
    <div className="flex flex-row w-full items-start justify-between max-tablet:flex-col gap-5">
      <div className="relative w-[240px] h-[150px] rounded-lg overflow-hidden max-tablet:w-[350px] max-mobile:w-full max-tablet:aspect-[16/9] max-tablet:h-auto">
        <Image
          src={discussionImg}
          alt="Discussion Thumbnail"
          fill
          className="object-cover"
        />
        {isLive && (
          <div className="absolute top-[12px] left-[12px] bg-[rgba(255,0,0,0.5)] rounded-sm font-semibold text-sm text-white p-1">
            LIVE
          </div>
        )}
      </div>

      <div className="flex flex-col flex-1 h-full justify-between items-start w-full">
        <div className="flex flex-col flex-1 gap-1 w-full">
          <div className="flex w-full items-start justify-between">
            <div className="text-sm text-neutral-400 light:text-[#737373] font-normal">
              {statusLabel}
            </div>
            <div className="relative w-fit h-fit hidden max-tablet:block">
              <Extra2 className="cursor-pointer w-6 h-6" onClick={() => {}} />
            </div>
          </div>
          <div className="text-lg text-text-primary font-bold">{title}</div>
          <div className="text-sm text-[#6d6d6d] light:text-[#737373] font-normal">
            {formattedDate}
          </div>
          <div
            className="text-sm text-neutral-400 light:text-[#737373] font-normal overflow-hidden text-ellipsis"
            style={{
              display: '-webkit-box',
              WebkitLineClamp: 2,
              WebkitBoxOrient: 'vertical',
            }}
          >
            {description}
          </div>
        </div>

        {isLive && isMember && status !== SpaceStatus.Draft && (
          <div className="flex flex-row w-full justify-end">
            <JoinButton
              onClick={() => {
                onclick();
              }}
            />
          </div>
        )}

        {isFinished && isMember && record && (
          <div className="flex flex-row w-full justify-end items-end">
            <ViewRecord
              onClick={() => {
                viewRecordClick();
              }}
            />
          </div>
        )}
      </div>
    </div>
  );
}

function ViewRecord({ onClick }: { onClick: () => void }) {
  const t = useTranslations('DeliberationSpace');
  return (
    <div
      className="cursor-pointer flex flex-row items-center w-fit h-fit px-5 py-2.5 gap-2.5 bg-white light:bg-card-bg border border-card-border hover:bg-white/80 light:hover:bg-card-bg/50 rounded-lg"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-sm">{t('view_record')}</div>
      <ArrowRight className="stroke-black stroke-3 w-[15px] h-[15px]" />
    </div>
  );
}

function JoinButton({ onClick }: { onClick: () => void }) {
  const t = useTranslations('DeliberationSpace');
  return (
    <div
      className="cursor-pointer flex flex-row items-center w-fit h-fit px-5 py-2.5 gap-2.5 bg-white light:bg-card-bg border border-card-border hover:bg-white/80 light:hover:bg-card-bg/50 rounded-lg"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-sm">{t('join')}</div>
      <ArrowRight className="stroke-black stroke-3 w-[15px] h-[15px]" />
    </div>
  );
}

function EditableDiscussion() {
  const t = useTranslations('DeliberationSpace');
  const { deliberation, handleUpdateDeliberation } =
    useDeliberationSpaceContext();
  const discussions = deliberation.discussions;
  const popup = usePopup();
  const stableKeys = useMemo(
    () => discussions.map(() => uuidv4()),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [discussions.length],
  );

  const handleAddDiscussion = (discussion: DiscussionInfo) => {
    handleUpdateDeliberation({
      ...deliberation,
      discussions: [...deliberation.discussions, discussion],
    });
  };

  const handleRemoveDiscussion = (index: number) => {
    const updated = deliberation.discussions.filter((_, i) => i !== index);

    handleUpdateDeliberation({
      ...deliberation,
      discussions: updated,
    });
  };

  const handleUpdateDiscussion = (
    index: number,
    discussion: DiscussionInfo,
  ) => {
    const updated = [...deliberation.discussions];
    updated[index] = discussion;

    handleUpdateDeliberation({
      ...deliberation,
      discussions: updated,
    });
  };

  return (
    <BorderSpaceCard>
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-row w-full justify-between items-center">
          <div className="font-bold text-text-primary text-[15px]/[20px]">
            {t('discussions')}
          </div>

          <AddDiscussion
            onadd={() => {
              popup
                .open(
                  <NewDiscussion
                    discussion={{
                      started_at: Math.floor(Date.now()),
                      ended_at: Math.floor(Date.now() + 3600 * 1000),
                      name: '',
                      description: '',
                      participants: [],
                    }}
                    onadd={(discussion: DiscussionInfo) => {
                      handleAddDiscussion(discussion);
                    }}
                  />,
                )
                .withTitle(t('new_discussion'))
                .overflow(true)
                .withoutBackdropClose();
            }}
          />
        </div>

        {discussions.map((discussion, index) => (
          <EditableDiscussionInfo
            key={stableKeys[index]}
            index={index}
            discussionId={discussion.discussion_id}
            startedAt={discussion.started_at}
            endedAt={discussion.ended_at}
            name={discussion.name}
            description={discussion.description}
            participants={discussion.participants}
            onupdate={(index: number, discussion: DiscussionInfo) => {
              handleUpdateDiscussion(index, discussion);
            }}
            onremove={(index: number) => {
              handleRemoveDiscussion(index);
            }}
          />
        ))}
      </div>
    </BorderSpaceCard>
  );
}

function AddDiscussion({ onadd }: { onadd: () => void }) {
  const t = useTranslations('DeliberationSpace');
  return (
    <div
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

function EditableDiscussionInfo({
  index,
  startedAt,
  endedAt,
  name,
  description,
  participants,
  onupdate,
  onremove,
}: {
  index: number;
  discussionId?: number;
  startedAt: number;
  endedAt: number;
  name: string;
  description: string;
  participants: TotalUser[];
  onupdate: (index: number, discussion: DiscussionInfo) => void;
  onremove: (index: number) => void;
}) {
  const t = useTranslations('DeliberationSpace');
  const now = Math.floor(Date.now() / 1000);

  const popup = usePopup();
  const [startTime, setStartTime] = useState<number>(startedAt);
  const [endTime, setEndTime] = useState<number>(endedAt);
  const [title, setTitle] = useState<string>(name);
  const [desc, setDesc] = useState<string>(description);
  const [users, setUsers] = useState<TotalUser[]>(participants);
  const [menuOpen, setMenuOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const mobileMenuRef = useRef<HTMLDivElement>(null);

  const isLive = now >= startTime && now <= endTime;
  const isUpcoming = now < startTime;
  const isFinished = now > endTime;

  const formattedDate = `${format(new Date(startTime * 1000), 'dd MMM, yyyy HH:mm')} - ${format(new Date(endTime * 1000), 'dd MMM, yyyy HH:mm')}`;

  const statusLabel = isUpcoming
    ? t('upcoming_discussion')
    : isFinished
      ? t('finished_discussion')
      : t('ongoing_discussion');

  useEffect(() => {
    setTitle(name);
    setDesc(description);
    setStartTime(startedAt);
    setEndTime(endedAt);
    setUsers(participants);
  }, [name, description, startedAt, endedAt, participants]);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      const target = e.target as Node;
      if (
        menuRef.current &&
        !menuRef.current.contains(target) &&
        mobileMenuRef.current &&
        !mobileMenuRef.current.contains(target)
      ) {
        setMenuOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  return (
    <div className="w-full flex flex-col gap-4 relative">
      <div className="flex flex-row w-full items-start justify-between max-tablet:flex-col gap-5">
        <div className="relative w-[240px] h-[150px] rounded-lg overflow-hidden max-tablet:w-[350px] max-mobile:w-full max-tablet:aspect-[16/9] max-tablet:h-auto">
          <Image
            src={discussionImg}
            alt="Discussion Thumbnail"
            fill
            className="object-cover"
          />
          {isLive && (
            <div className="absolute top-[12px] left-[12px] bg-[rgba(255,0,0,0.5)] rounded-sm font-semibold text-sm text-white p-1">
              LIVE
            </div>
          )}
        </div>

        <div className="flex flex-col flex-1 h-full justify-between items-start w-full">
          <div className="flex flex-col flex-1 gap-1 w-full">
            <div className="flex w-full items-start justify-between">
              <div className="text-sm text-neutral-400 light:text-[#737373] font-normal">
                {statusLabel}
              </div>
              <div
                className="relative w-fit h-fit hidden max-tablet:block"
                ref={mobileMenuRef}
              >
                <Extra2
                  className="cursor-pointer w-6 h-6"
                  onClick={() => setMenuOpen(!menuOpen)}
                />
                {menuOpen && (
                  <div className="absolute right-0 mt-2 w-25 bg-white text-black rounded shadow-lg text-sm z-50 overflow-hidden">
                    <div
                      className="px-4 py-2 hover:bg-neutral-200 cursor-pointer whitespace-nowrap"
                      onClick={() => {
                        popup
                          .open(
                            <NewDiscussion
                              discussion={{
                                started_at: Math.floor(startedAt * 1000),
                                ended_at: Math.floor(endedAt * 1000),
                                name,
                                description,
                                participants: users,
                              }}
                              onadd={(discussion: DiscussionInfo) => {
                                onupdate(index, discussion);
                              }}
                            />,
                          )
                          .withTitle('New Discussion')
                          .overflow(true)
                          .withoutBackdropClose();
                        setMenuOpen(false);
                      }}
                    >
                      {t('update')}
                    </div>
                    <div
                      className="px-4 py-2 hover:bg-neutral-200 cursor-pointer whitespace-nowrap"
                      onClick={() => {
                        onremove(index);
                        setMenuOpen(false);
                      }}
                    >
                      {t('delete')}
                    </div>
                  </div>
                )}
              </div>
            </div>
            <div className="text-lg text-text-primary font-bold">{title}</div>
            <div className="text-sm text-[#6d6d6d] light:text-[#737373] font-normal">
              {formattedDate}
            </div>
            <div
              className="text-sm text-neutral-400 light:text-[#737373] font-normal overflow-hidden text-ellipsis"
              style={{
                display: '-webkit-box',
                WebkitLineClamp: 2,
                WebkitBoxOrient: 'vertical',
              }}
            >
              {desc}
            </div>
          </div>
        </div>

        <div className="relative w-fit h-fit max-tablet:hidden" ref={menuRef}>
          <Extra2
            className="cursor-pointer w-6 h-6"
            onClick={() => setMenuOpen(!menuOpen)}
          />
          {menuOpen && (
            <div className="absolute right-0 mt-2 w-25 bg-white text-black rounded shadow-lg text-sm z-50 overflow-hidden">
              <div
                className="px-4 py-2 hover:bg-neutral-200 cursor-pointer whitespace-nowrap"
                onClick={() => {
                  popup
                    .open(
                      <NewDiscussion
                        discussion={{
                          started_at: Math.floor(startedAt * 1000),
                          ended_at: Math.floor(endedAt * 1000),
                          name,
                          description,
                          participants: users,
                        }}
                        onadd={(discussion: DiscussionInfo) => {
                          onupdate(index, discussion);
                        }}
                      />,
                    )
                    .withTitle('New Discussion')
                    .overflow(true)
                    .withoutBackdropClose();
                  setMenuOpen(false);
                }}
              >
                {t('update')}
              </div>
              <div
                className="px-4 py-2 hover:bg-neutral-200 cursor-pointer whitespace-nowrap"
                onClick={() => {
                  onremove(index);
                  setMenuOpen(false);
                }}
              >
                {t('delete')}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
