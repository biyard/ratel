'use client';

import Image from 'next/image';
import BlackBox from '@/app/(social)/_components/black-box';
import CustomCalendar from '@/components/calendar-picker/calendar-picker';
import TimeDropdown from '@/components/time-dropdown/time-dropdown';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Trash2 } from 'lucide-react';
import React, { useEffect, useMemo, useState } from 'react';
import { format } from 'date-fns';

import { v4 as uuidv4 } from 'uuid';
import discussionImg from '@/assets/images/discussion.png';
import { Discussion, Member } from '@/lib/api/models/discussion';
import { Add } from './add';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { ArrowRight } from 'lucide-react';
import { useRouter } from 'next/navigation';
import { route } from '@/route';
import { UserGroup } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import InviteMemberPopup from './invite_member';
import { DiscussionInfo } from '../types';
import { TotalUser } from '@/lib/api/models/user';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';

export interface SpaceDiscussionProps {
  isEdit?: boolean;
  status: SpaceStatus;
  discussions: DiscussionInfo[];
  discussionRaws: Discussion[];
  onremove?: (index: number) => void;
  onupdate?: (index: number, discussion: DiscussionInfo) => void;
  onadd?: (discussion: DiscussionInfo) => void;
}

export default function SpaceDiscussion({
  isEdit = false,
  status,
  discussions,
  discussionRaws,
  onadd = () => {},
  onremove = () => {},
  onupdate = () => {},
}: SpaceDiscussionProps) {
  const router = useRouter();
  const { data: userInfo } = useSuspenseUserInfo();
  const userId = userInfo?.id || 0;

  const moveDiscussion = (spaceId: number, discussionId: number) => {
    router.push(route.discussionById(spaceId, discussionId));
  };

  return (
    <div className="flex flex-col w-full">
      {isEdit ? (
        <EditableDiscussion
          discussions={discussions}
          onadd={() => {
            const started_at = Math.floor(Date.now() / 1000);
            const ended_at = Math.floor(Date.now() / 1000);
            const name = '';
            const description = '';

            onadd({
              started_at,
              ended_at,
              name,
              description,
              participants: [],
            });
          }}
          onupdate={(index: number, discussion: DiscussionInfo) => {
            onupdate(index, discussion);
          }}
          onremove={(index: number) => {
            onremove(index);
          }}
        />
      ) : (
        <ViewDiscussion
          userId={userId}
          discussions={discussionRaws}
          status={status}
          moveDiscussion={moveDiscussion}
        />
      )}
    </div>
  );
}

function ViewDiscussion({
  userId,
  discussions,
  status,
  moveDiscussion,
}: {
  userId: number;
  discussions: Discussion[];
  status: SpaceStatus;
  moveDiscussion: (spaceId: number, discussionId: number) => void;
}) {
  return (
    <div className="flex flex-col w-full gap-2.5">
      <DiscussionSchedules
        userId={userId}
        discussions={discussions}
        status={status}
        moveDiscussion={moveDiscussion}
      />
    </div>
  );
}

function DiscussionSchedules({
  userId,
  discussions,
  status,
  moveDiscussion,
}: {
  userId: number;
  discussions: Discussion[];
  status: SpaceStatus;
  moveDiscussion: (spaceId: number, discussionId: number) => void;
}) {
  return (
    <div className="flex flex-col gap-2.5">
      {/* <BlackBox>
        <div className="flex flex-col gap-3">
          <div className="font-bold text-white text-[15px]/[20px]">
            Schedule
          </div>
          <div className="flex flex-col gap-5">
            <div className="font-normal text-neutral-300 text-[15px]/[24px]">
              In order to improve feasibility of Digital asset basic act, we
              have scheduled a discussion and a lecture on it.
            </div>

            {discussions.map((discussion, index) => (
              <DiscussionTable
                key={index}
                startDate={discussion.started_at}
                endDate={discussion.ended_at}
                title={discussion.name}
                description={discussion.description}
              />
            ))}
          </div>
        </div>
      </BlackBox> */}

      <BlackBox>
        <div className="flex flex-col w-full gap-5">
          <div className="font-bold text-white text-[15px]/[20px]">
            Discussions
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
                  onclick={() => {
                    moveDiscussion(discussion.space_id, discussion.id);
                  }}
                />
                {index !== discussions.length - 1 ? (
                  <div className=" w-full h-0.25 gap-1 bg-neutral-800" />
                ) : (
                  <></>
                )}
              </React.Fragment>
            ))}
          </div>
        </div>
      </BlackBox>
    </div>
  );
}

export function DiscussionRoom({
  userId,
  status,
  startDate,
  endDate,
  title,
  description,
  members,
  onclick,
}: {
  userId: number;
  status: SpaceStatus;
  startDate: number;
  endDate: number;
  title: string;
  description: string;
  members: Member[];

  onclick: () => void;
}) {
  const now = Math.floor(Date.now() / 1000);

  const isLive = now >= startDate && now <= endDate;
  const isUpcoming = now < startDate;
  const isFinished = now > endDate;

  const formattedDate = `${format(new Date(startDate * 1000), 'dd MMM, yyyy HH:mm')} - ${format(new Date(endDate * 1000), 'dd MMM, yyyy HH:mm')}`;

  const statusLabel = isUpcoming
    ? 'Upcoming discussion'
    : isFinished
      ? 'Finished discussion'
      : 'On-going';

  const isMember = members.some((member) => member.id === userId);

  return (
    <div className="flex flex-row w-full items-start justify-between gap-5">
      <div className="relative w-[240px] h-[150px] rounded-lg overflow-hidden">
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

      <div className="flex flex-col flex-1 h-full justify-between items-start">
        <div className="flex flex-col flex-1 gap-1">
          <div className="text-sm text-neutral-400 font-normal">
            {statusLabel}
          </div>
          <div className="text-lg text-white font-bold">{title}</div>
          <div className="text-sm text-[#6d6d6d] font-normal">
            {formattedDate}
          </div>
          <div
            className="text-sm text-neutral-400 font-normal overflow-hidden text-ellipsis"
            style={{
              display: '-webkit-box',
              WebkitLineClamp: 2,
              WebkitBoxOrient: 'vertical',
            }}
          >
            {description}
          </div>
        </div>

        {isLive && status == SpaceStatus.InProgress && isMember && (
          <div className="flex flex-row w-full justify-end">
            <JoinButton
              onClick={() => {
                onclick();
              }}
            />
          </div>
        )}
      </div>
    </div>
  );
}

function JoinButton({ onClick }: { onClick: () => void }) {
  return (
    <div
      className="cursor-pointer flex flex-row items-center w-fit h-fit px-5 py-2.5 gap-2.5 bg-white hover:bg-neutral-300 rounded-lg"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-sm">Join</div>
      <ArrowRight className="stroke-black stroke-3 w-[15px] h-[15px]" />
    </div>
  );
}

function EditableDiscussion({
  discussions,
  onremove,
  onadd,
  onupdate,
}: {
  discussions: DiscussionInfo[];
  onremove: (index: number) => void;
  onadd: () => void;
  onupdate: (index: number, discussion: DiscussionInfo) => void;
}) {
  const stableKeys = useMemo(
    () => discussions.map(() => uuidv4()),
    [discussions.length],
  );

  return (
    <BlackBox>
      <div className="flex flex-col w-full gap-5">
        <div className="font-bold text-white text-[15px]/[20px]">
          Discussion
        </div>

        {discussions.map((discussion, index) => (
          <EditableDiscussionInfo
            key={stableKeys[index]}
            index={index}
            startedAt={discussion.started_at}
            endedAt={discussion.ended_at}
            name={discussion.name}
            description={discussion.description}
            participants={discussion.participants}
            onupdate={(index: number, discussion: DiscussionInfo) => {
              onupdate(index, discussion);
            }}
            onremove={(index: number) => {
              onremove(index);
            }}
          />
        ))}

        <AddDiscussion
          onadd={() => {
            onadd();
          }}
        />
      </div>
    </BlackBox>
  );
}

function AddDiscussion({ onadd }: { onadd: () => void }) {
  return (
    <div
      onClick={() => {
        onadd();
      }}
      className="bg-transparent border-2 border-dashed border-neutral-700 rounded-md h-50 flex flex-col items-center justify-center cursor-pointer hover:bg-neutral-800 transition gap-[10px]"
    >
      <div className="flex flex-row w-[45px] h-[45px] justify-center items-center rounded-full border border-neutral-500">
        <Add className="w-5 h-5 stroke-neutral-500 text-neutral-500" />
      </div>
      <span className=" text-neutral-500 font-medium text-base">
        Add Discussion
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
  startedAt: number;
  endedAt: number;
  name: string;
  description: string;
  participants: TotalUser[];
  onupdate: (index: number, discussion: DiscussionInfo) => void;
  onremove: (index: number) => void;
}) {
  const popup = usePopup();
  const [startTime, setStartTime] = useState<number>(startedAt);
  const [endTime, setEndTime] = useState<number>(endedAt);
  const [title, setTitle] = useState<string>(name);
  const [desc, setDesc] = useState<string>(description);
  const [users, setUsers] = useState<TotalUser[]>(participants);

  useEffect(() => {
    setUsers(participants);
  }, [participants]);

  const update = (
    newStart: number,
    newEnd: number,
    newTitle: string,
    newDesc: string,
    newParticipants: TotalUser[],
  ) => {
    onupdate(index, {
      started_at: newStart,
      ended_at: newEnd,
      name: newTitle,
      description: newDesc,
      participants: newParticipants,
    });
  };

  return (
    <div className="flex flex-col gap-5 border border-neutral-700 rounded-md p-4">
      <div className="flex flex-row w-full justify-end items-center gap-[10px]">
        <div
          className="cursor-pointer flex flex-row items-center px-[20px] py-[10px] bg-white hover:bg-neutral-300 rounded-lg gap-[10px]"
          onClick={() => {
            popup
              .open(
                <InviteMemberPopup
                  users={users}
                  onclick={(users: TotalUser[]) => {
                    update(startTime, endTime, title, desc, users);
                    popup.close();
                  }}
                />,
              )
              .withTitle('Invite to Discussion');
          }}
        >
          <UserGroup className="w-[18px] h-[18px] stroke-neutral-500" />
          <div className="font-bold text-sm text-[#000203]">Invite Member</div>
        </div>
        <div
          className="cursor-pointer w-fit h-fit"
          onClick={() => {
            onremove(index);
          }}
        >
          <Trash2 className="stroke-white w-[18px] h-[18px]" />
        </div>
      </div>
      <div className="flex flex-col w-full justify-start items-start gap-[10px]">
        <div className="font-medium text-neutral-300 text-[15px] w-[80px]">
          Title
        </div>
        <Input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onBlur={() => update(startTime, endTime, title, desc, users)}
        />
      </div>

      <div className="flex flex-wrap w-full justify-between items-center gap-[10px]">
        <div className="font-medium text-neutral-300 text-[15px] w-[80px]">
          Period
        </div>
        <div className="flex flex-row gap-[10px] items-center flex-wrap">
          <div className="flex flex-row gap-[10px]">
            <CustomCalendar
              value={startTime * 1000}
              onChange={(date) => {
                const newStart = Math.floor(date / 1000);
                setStartTime(newStart);
                update(newStart, endTime, title, desc, users);
              }}
            />
            <TimeDropdown
              value={startTime * 1000}
              onChange={(timestamp) => {
                const newStart = Math.floor(timestamp / 1000);
                setStartTime(newStart);
                update(newStart, endTime, title, desc, users);
              }}
            />
          </div>
          <div className="w-[20px] h-[1px] bg-neutral-500" />
          <div className="flex flex-row gap-[10px]">
            <CustomCalendar
              value={endTime * 1000}
              onChange={(date) => {
                const newEnd = Math.floor(date / 1000);
                setEndTime(newEnd);
                update(startTime, newEnd, title, desc, users);
              }}
            />
            <TimeDropdown
              value={endTime * 1000}
              onChange={(timestamp) => {
                const newEnd = Math.floor(timestamp / 1000);
                setEndTime(newEnd);
                update(startTime, newEnd, title, desc, users);
              }}
            />
          </div>
        </div>
      </div>

      <div className="flex flex-col w-full justify-start items-start gap-[10px]">
        <div className="font-medium text-neutral-300 text-[15px] w-[80px]">
          Description
        </div>
        <Textarea
          value={desc}
          onChange={(e) => setDesc(e.target.value)}
          onBlur={() => update(startTime, endTime, title, desc, users)}
        />
      </div>
    </div>
  );
}
