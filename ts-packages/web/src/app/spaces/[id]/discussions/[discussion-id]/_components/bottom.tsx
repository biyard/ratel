'use client';

import {
  ZoomChat,
  ZoomClose,
  ZoomMicOff,
  ZoomMicOn,
  ZoomParticipants,
  ZoomRecord,
  ZoomShare,
  ZoomVideoOff,
  ZoomVideoOn,
} from '@/components/icons';
import React, { JSX } from 'react';

export default function Bottom({
  isVideoOn,
  isAudioOn,
  isRecording,

  onclose,
  onRecordClick,
  onParticipantsClick,
  onChatClick,
  onAudioToggle,
  onVideoToggle,
  onShareToggle,
}: {
  isVideoOn: boolean;
  isAudioOn: boolean;
  isSharing: boolean;
  isRecording: boolean;
  onRecordClick: () => void;
  onclose: () => void;
  onParticipantsClick: () => void;
  onChatClick: () => void;
  onAudioToggle: () => void;
  onVideoToggle: () => void;
  onShareToggle: () => void;
}) {
  return (
    <div className="flex flex-row w-full min-h-[70px] justify-between items-center bg-neutral-900 px-10 py-2.5 border-b border-neutral-800">
      <div className="flex flex-row gap-5 w-[80px]">
        <IconLabel
          icon={
            isAudioOn ? (
              <ZoomMicOn className="w-6 h-6" />
            ) : (
              <ZoomMicOff className="w-6 h-6" />
            )
          }
          label="Audio"
          onclick={() => {
            onAudioToggle();
          }}
        />
        <IconLabel
          icon={
            isVideoOn ? (
              <ZoomVideoOn className="w-6 h-6" />
            ) : (
              <ZoomVideoOff className="w-6 h-6" />
            )
          }
          label="Video"
          onclick={() => {
            onVideoToggle();
          }}
        />
      </div>

      <div className="flex flex-row w-fit gap-5">
        <IconLabel
          icon={<ZoomParticipants />}
          label="Participants"
          onclick={onParticipantsClick}
        />
        <IconLabel icon={<ZoomChat />} label="Chat" onclick={onChatClick} />
        <IconLabel
          icon={<ZoomShare className="w-6 h-6" />}
          label="Share"
          onclick={() => {
            onShareToggle();
          }}
        />
        <IconLabel
          icon={
            isRecording ? (
              <ZoomClose className="w-6 h-6" />
            ) : (
              <ZoomRecord className="w-6 h-6" />
            )
          }
          label="Record"
          onclick={() => {
            onRecordClick();
          }}
        />
      </div>

      <div className="flex flex-row w-[80px] gap-5">
        <IconLabel
          icon={<ZoomClose className="w-6 h-6" />}
          label="End"
          onclick={onclose}
        />
      </div>
    </div>
  );
}

function IconLabel({
  icon,
  label,
  onclick,
}: {
  icon: JSX.Element;
  label: string;
  onclick: () => void;
}) {
  return (
    <div
      className="cursor-pointer flex flex-col gap-1 w-fit h-fit justify-center items-center px-[10px] py-[4px]"
      onClick={onclick}
    >
      {icon}
      <div className="font-semibold text-white text-sm">{label}</div>
    </div>
  );
}
