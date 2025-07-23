'use client';

// import CalendarPicker from '@/components/calendar-picker/calendar-picker';
// import TimeDropdown from '@/components/time-dropdown/time-dropdown';
import React from 'react';
import { DiscussionInfo } from '../types';
import SpaceDiscussion from './space_discussion';
import SpaceElearning from './space_elearning';
import { FileInfo } from '@/lib/api/models/feeds';
import {
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from '../provider.client';

export default function DeliberationPage() {
  const { isEdit, deliberation, setDeliberation, handleViewRecord, status } =
    useDeliberationSpaceContext();
  const space = useDeliberationSpace();
  const discussions = space?.discussions ?? [];

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceDiscussion
          isEdit={isEdit}
          status={status}
          discussions={deliberation.discussions}
          discussionRaws={discussions}
          viewRecord={handleViewRecord}
          onadd={(discussion: DiscussionInfo) => {
            setDeliberation({
              ...deliberation,
              discussions: [...deliberation.discussions, discussion],
            });
          }}
          onupdate={(index: number, discussion: DiscussionInfo) => {
            const updated = [...deliberation.discussions];
            updated[index] = discussion;
            setDeliberation({
              ...deliberation,
              discussions: updated,
            });
          }}
          onremove={(index: number) => {
            const updated = deliberation.discussions.filter(
              (_, i) => i !== index,
            );
            setDeliberation({
              ...deliberation,
              discussions: updated,
            });
          }}
        />
        <SpaceElearning
          isEdit={isEdit}
          elearnings={deliberation.elearnings}
          onremove={(index: number) => {
            const updated = deliberation.elearnings.filter(
              (_, i) => i !== index,
            );
            setDeliberation({
              ...deliberation,
              elearnings: updated,
            });
          }}
          onadd={(file: FileInfo) => {
            setDeliberation({
              ...deliberation,
              elearnings: [...deliberation.elearnings, { files: [file] }],
            });
          }}
        />
      </div>
      {/* <CalendarPicker
        value={1750321970 * 1000}
        onChange={(newTimestamp) => {
          console.log(
            'Selected date:',
            new Date(newTimestamp).toLocaleString(),
          );
        }}
      /> */}
      {/* <TimeDropdown
        value={1750321970 * 1000}
        onChange={function (newTimestamp: number): void {
          throw new Error('Function not implemented.');
        }}
      /> */}
    </div>
  );
}
