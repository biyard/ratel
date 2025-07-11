'use client';
import { Internet } from '@/components/icons';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { usePopup } from '@/lib/contexts/popup-service';
import React, { useState } from 'react';
import InviteMemberPopup from './invite_member';
import { DiscussionInfo } from '../../types';
import TimeDropdown from '@/components/time-dropdown';
import CalendarDropdown from '@/components/calendar-dropdown';

export default function NewDiscussion({
  discussion,
  onadd,
}: {
  discussion: DiscussionInfo;
  onadd: (discussion: DiscussionInfo) => void;
}) {
  const popup = usePopup();
  const [title, setTitle] = useState(discussion.name);
  const [description, setDescription] = useState(discussion.description);
  const [reminderEnabled] = useState(false);

  const [startTime, setStartTime] = useState<number>(discussion.started_at);
  const [endTime, setEndTime] = useState<number>(discussion.ended_at);
  return (
    <div className="max-w-[900px] w-full">
      <div className="flex flex-col py-2.5 gap-[5px]">
        <label className="flex flex-row justify-start items-center text-[15px]/[28px] text-neutral-400 font-bold  gap-1">
          Title <span className="text-error">*</span>
        </label>
        <Input
          className="px-5 py-[10.5px] bg-transparent border border-btn-o font-medium text-[15px]/[22.5px] placeholder:text-neutral-600 text-white"
          placeholder="Input your discussion name."
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          maxLength={100}
        />
        <div className="text-right text-[15px]/[22.5px] font-medium text-neutral-600">
          {title.length}/100
        </div>
      </div>

      <div className="flex flex-col py-2.5 gap-[5px]">
        <label className="text-[15px]/[28px] text-neutral-400 font-bold">
          Description
        </label>
        <Textarea
          className="px-5 py-[10.5px] bg-transparent border border-btn-o font-normal text-sm placeholder:text-neutral-600 text-white max-h-[100px] overflow-y-auto"
          placeholder="What is the purpose of your discussion?"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          maxLength={100}
        />
        <div className="text-right text-[15px]/[22.5px] font-medium text-neutral-600">
          {description.length}/100
        </div>
      </div>

      <div className="flex flex-col py-2.5 gap-[5px]">
        <label className="flex flex-row justify-start items-center text-[15px]/[28px] text-neutral-400 font-bold  gap-1">
          Date <span className="text-error">*</span>
        </label>
        <div className="flex flex-row gap-2.5 items-center">
          <CalendarDropdown
            value={startTime}
            onChange={(date) => {
              const newStart = Math.floor(date);
              setStartTime(newStart);
            }}
          />
          <TimeDropdown
            value={startTime}
            onChange={(timestamp) => {
              const newStart = Math.floor(timestamp);
              setStartTime(newStart);
            }}
          />
          <div className="w-[15px] h-0.25 bg-neutral-600" />
          <CalendarDropdown
            value={endTime}
            onChange={(date) => {
              const newEnd = Math.floor(date);
              setEndTime(newEnd);
            }}
          />
          <TimeDropdown
            value={endTime}
            onChange={(timestamp) => {
              const newEnd = Math.floor(timestamp);
              setEndTime(newEnd);
            }}
          />
          <div className="flex flex-row items-center w-fit border border-c-wg-70 rounded-lg px-5 py-[10.5px] gap-2.5">
            <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
              Pacific Time
            </div>
            <Internet
              className="w-5 h-5 [&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500"
              width="20"
              height="20"
            />
          </div>
        </div>
      </div>

      {/* <div className="flex flex-row w-full py-5 items-start gap-2.5">
        <CustomCheckbox
          checked={reminderEnabled}
          onChange={() => setReminderEnabled(!reminderEnabled)}
          disabled={false}
        />
        <div className="text-[15px]/[24px]">
          <div className="font-medium text-white">Reminder Notification</div>
          <div className="font-normal text-neutral-300">
            A reminder email will be sent 10 minutes prior to the discussion.
          </div>
        </div>
      </div> */}

      <div className="flex justify-end mt-2.5">
        <button
          className="w-fit px-10 py-[14.5px] rounded-[10px] bg-primary hover:bg-hover text-black text-bold text-base hover:text-black cursor-pointer"
          onClick={() => {
            popup
              .open(
                <InviteMemberPopup
                  title={title}
                  description={description}
                  startTime={startTime / 1000}
                  endTime={endTime / 1000}
                  reminderEnabled={reminderEnabled}
                  users={discussion.participants}
                  onadd={(discussion: DiscussionInfo) => {
                    onadd(discussion);
                    popup.close();
                  }}
                />,
              )
              .withTitle('New Discussion')
              .withoutBackdropClose();
          }}
        >
          {'Continue'}
        </button>
      </div>
    </div>
  );
}
