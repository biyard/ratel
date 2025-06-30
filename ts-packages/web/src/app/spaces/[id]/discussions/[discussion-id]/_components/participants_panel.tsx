'use client';

import {
  Clear,
  Logo,
  ZoomMicOff,
  ZoomMicOn,
  ZoomVideoOff,
  ZoomVideoOn,
} from '@/components/icons';
import React, { useEffect, useMemo, useState } from 'react';
import Image from 'next/image';
import { DiscussionParticipant } from '@/lib/api/models/discussion';

import { DefaultMeetingSession } from 'amazon-chime-sdk-js';
import { Participant } from '@/lib/api/models/meeting';

export default function ParticipantsPanel({
  micStates,
  videoStates,
  users,
  participants,
  onClose,
}: {
  micStates: Record<string, boolean>;
  videoStates: Record<string, boolean>;
  users: DiscussionParticipant[];
  participants: Participant[];
  setFocusedAttendeeId: (attendeeId: string | null) => void;
  meetingSession: DefaultMeetingSession;
  onClose: () => void;
}) {
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    setTimeout(() => setVisible(true), 10);
  }, []);

  const userIdToAttendeeId = useMemo(() => {
    const map = new Map<number, string>();
    users.forEach((u) => {
      map.set(u.user_id, u.participant_id);
    });
    return map;
  }, [users]);

  const handleClose = () => {
    setVisible(false);
    setTimeout(onClose, 300);
  };

  return (
    <div
      className={`fixed top-0 right-0 h-full w-[280px] z-30 bg-[#2d2d2d] shadow-lg border-l border-neutral-800 transform transition-all duration-300 ${
        visible ? 'translate-x-0' : 'translate-x-full'
      }`}
    >
      <div className="flex justify-between items-center px-4 py-3 border-b border-neutral-600">
        <div className="flex flex-row w-fit gap-2.5">
          <Logo width={25} height={25} />
          <div className="font-semibold text-sm text-white">Participants</div>
        </div>
        <Clear
          className="cursor-pointer w-[24px] h-[24px] [&>path]:stroke-[#bfc8d9]"
          onClick={handleClose}
          fill="white"
        />
      </div>

      <div className="flex flex-col flex-1 overflow-y-auto px-[10px] py-[20px] gap-[20px]">
        {participants.map((participant, index) => {
          const attendeeId = userIdToAttendeeId.get(participant.id);

          const isMicOn = micStates[attendeeId ?? ''] ?? false;
          const isVideoOn = videoStates[attendeeId ?? ''] ?? false;

          return (
            <div
              key={index}
              onDoubleClick={() => {}}
              className="flex flex-row w-full justify-between items-center"
            >
              <div className="flex flex-row w-fit items-center gap-1">
                {participant.profile_url ? (
                  <Image
                    width={30}
                    height={30}
                    src={participant.profile_url || '/default-profile.png'}
                    alt={`${participant.username}'s profile`}
                    className="w-[30px] h-[30px] object-cover rounded-full"
                  />
                ) : (
                  <div className="w-8 h-8 bg-neutral-500 rounded-full" />
                )}
                <div className="font-medium text-white text-sm">
                  {participant.username}
                </div>
              </div>

              <div className="flex flex-row w-fit gap-2">
                {isMicOn ? (
                  <ZoomMicOn className="w-[18px] h-[18px] stroke-white" />
                ) : (
                  <ZoomMicOff className="w-[18px] h-[18px] stroke-white" />
                )}
                {isVideoOn ? (
                  <ZoomVideoOn className="w-[18px] h-[18px] stroke-white" />
                ) : (
                  <ZoomVideoOff className="w-[18px] h-[18px] stroke-white" />
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
