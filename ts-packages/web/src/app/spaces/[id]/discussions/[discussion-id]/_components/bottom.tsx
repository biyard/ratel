'use client';

import {
  Extra,
  ZoomChat,
  ZoomClose,
  ZoomMicOff,
  ZoomMicOn,
  ZoomParticipants,
  // ZoomRecord,
  ZoomShare,
  ZoomVideoOff,
  ZoomVideoOn,
} from '@/components/icons';
import React, { JSX, useEffect, useRef, useState } from 'react';

export default function Bottom({
  isVideoOn,
  isAudioOn,
  // isRecording,
  onclose,
  // onRecordClick,
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
  const [showOptions, setShowOptions] = useState(false);
  const optionsBtnRef = useRef<HTMLDivElement | null>(null);
  const menuRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!showOptions) return;
    const onDocClick = (e: MouseEvent) => {
      const target = e.target as Node;
      if (
        menuRef.current &&
        !menuRef.current.contains(target) &&
        optionsBtnRef.current &&
        !optionsBtnRef.current.contains(target)
      ) {
        setShowOptions(false);
      }
    };
    document.addEventListener('mousedown', onDocClick);
    return () => document.removeEventListener('mousedown', onDocClick);
  }, [showOptions]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setShowOptions(false);
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, []);

  return (
    <div className="relative z-[150] w-full">
      <div className="flex flex-row w-full min-h-[70px] justify-between items-center bg-neutral-900 light:bg-neutral-800 px-10 py-2.5 border-b border-neutral-800">
        <div className="flex flex-row gap-5 flex-1 justify-start">
          <IconLabel
            icon={
              isAudioOn ? (
                <ZoomMicOn className="w-6 h-6" />
              ) : (
                <ZoomMicOff className="w-6 h-6" />
              )
            }
            label="Audio"
            onclick={onAudioToggle}
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
            onclick={onVideoToggle}
          />

          <div className="flex mobile:hidden justify-center items-center">
            <div ref={optionsBtnRef}>
              <IconLabel
                icon={<Extra />}
                label="Options"
                onclick={() => {
                  setShowOptions((s) => !s);
                }}
              />
            </div>
          </div>
        </div>

        <div className="hidden mobile:flex flex-row flex-1 gap-5 justify-center">
          <IconLabel
            icon={<ZoomParticipants />}
            label="Participants"
            onclick={onParticipantsClick}
          />
          <IconLabel icon={<ZoomChat />} label="Chat" onclick={onChatClick} />
          <IconLabel
            icon={<ZoomShare className="w-6 h-6" />}
            label="Share"
            onclick={onShareToggle}
          />
        </div>

        <div className="flex flex-row flex-1 gap-5 justify-end">
          <IconLabel
            icon={<ZoomClose className="w-6 h-6" />}
            label="End"
            onclick={onclose}
          />
        </div>
      </div>

      {showOptions && (
        <div
          ref={menuRef}
          className="tablet:hidden absolute bottom-[78px] left-1/2 -translate-x-1/2 w-[92%] max-w-[420px] rounded-xl bg-neutral-800 text-white shadow-2xl border border-neutral-700 p-2 z-[300]"
        >
          <div className="grid grid-cols-3 gap-1">
            <OptionItem
              icon={<ZoomParticipants />}
              label="Participants"
              onClick={() => {
                setShowOptions(false);
                onParticipantsClick();
              }}
            />
            <OptionItem
              icon={<ZoomChat />}
              label="Chat"
              onClick={() => {
                setShowOptions(false);
                onChatClick();
              }}
            />
            <OptionItem
              icon={<ZoomShare className="w-6 h-6" />}
              label="Share"
              onClick={() => {
                setShowOptions(false);
                onShareToggle();
              }}
            />
          </div>
        </div>
      )}
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
    <button
      type="button"
      className="cursor-pointer flex flex-col gap-1 w-fit h-fit justify-center items-center px-[10px] py-[4px] select-none"
      onClick={onclick}
      aria-label={label}
    >
      {icon}
      <div className="font-semibold text-white text-sm">{label}</div>
    </button>
  );
}

function OptionItem({
  icon,
  label,
  onClick,
}: {
  icon: JSX.Element;
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="flex flex-col items-center justify-center gap-1 py-3 rounded-lg hover:bg-neutral-700 active:bg-neutral-600"
    >
      <div className="w-6 h-6">{icon}</div>
      <div className="text-sm font-medium">{label}</div>
    </button>
  );
}
