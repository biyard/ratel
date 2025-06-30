'use client';

import { Clear, Logo } from '@/components/icons';
import { Send } from 'lucide-react';
import React, { useEffect, useRef, useState } from 'react';
import dayjs from 'dayjs';
import { Input } from '@/components/ui/input';
import { DiscussionParticipant } from '@/lib/api/models/discussion';
import { Participant } from '@/lib/api/models/meeting';
import Image from 'next/image';

export default function ChatPanel({
  onClose,
  messages,
  onSend,
  users,
  participants,
  myAttendeeId,
}: {
  onClose: () => void;
  users: DiscussionParticipant[];
  participants: Participant[];
  messages: { senderId: string; text: string; timestamp: number }[];
  onSend: (text: string) => void;
  myAttendeeId: string;
}) {
  const [visible, setVisible] = useState(false);
  const [input, setInput] = useState('');
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setTimeout(() => setVisible(true), 10);
  }, []);

  useEffect(() => {
    scrollRef.current?.scrollTo(0, scrollRef.current.scrollHeight);
  }, [messages]);

  const handleClose = () => {
    setVisible(false);
    setTimeout(onClose, 300);
  };

  const handleSend = () => {
    if (!input.trim()) return;
    onSend(input.trim());
    setInput('');
  };

  return (
    <div
      className={`fixed top-0 right-0 h-full w-[320px] bg-[#1e1e1e] z-40 border-l border-neutral-800 transform transition-all duration-300 ${
        visible ? 'translate-x-0' : 'translate-x-full'
      } flex flex-col`}
    >
      {/* Header */}
      <div className="flex justify-between items-center px-4 py-3 border-b border-neutral-700 flex-none">
        <div className="flex flex-row w-fit gap-2.5">
          <Logo width={24} height={24} />
          <div className="font-semibold text-sm text-white">Chat</div>
        </div>
        <Clear
          className="cursor-pointer w-[22px] h-[22px] [&>path]:stroke-[#bfc8d9]"
          onClick={handleClose}
        />
      </div>

      {/* Messages */}
      <div
        ref={scrollRef}
        className="flex-1 overflow-y-auto px-3 py-4 space-y-3 text-sm"
      >
        {messages.map((msg, i) => {
          const isMe = msg.senderId === myAttendeeId;
          const showDateHeader =
            i === 0 ||
            !dayjs(msg.timestamp).isSame(messages[i - 1].timestamp, 'day');
          const senderInfo = getParticipantInfo(
            msg.senderId,
            users,
            participants,
          );

          return (
            <div key={i}>
              {showDateHeader && (
                <div className="text-xs text-center text-neutral-400 my-2">
                  {dayjs(msg.timestamp).format('YYYY. MM. DD.')}
                </div>
              )}

              <div
                className={`flex flex-row ${isMe ? 'justify-end' : 'justify-start'}`}
              >
                <div className="flex flex-row gap-[5px] justify-end items-end">
                  {!isMe ? (
                    senderInfo?.profile_url ? (
                      <Image
                        width={30}
                        height={30}
                        src={senderInfo.profile_url || '/default-profile.png'}
                        alt={`${senderInfo.username}'s profile`}
                        className="w-[30px] h-[30px] object-cover rounded-full"
                      />
                    ) : (
                      <div className="w-8 h-8 bg-neutral-500 rounded-full" />
                    )
                  ) : (
                    <></>
                  )}
                  {isMe ? (
                    <div className="text-[10px] text-neutral-400 mt-1 text-right">
                      {dayjs(msg.timestamp).format('A h:mm')}
                    </div>
                  ) : (
                    <></>
                  )}
                  <div
                    className={`max-w-[80%] px-3 py-2 rounded-xl relative ${
                      isMe
                        ? 'bg-[#3f3f3f] text-white rounded-br-none'
                        : 'bg-neutral-700 text-white rounded-bl-none'
                    }`}
                  >
                    <div className="whitespace-pre-wrap">{msg.text}</div>
                  </div>
                  {!isMe ? (
                    <div className="text-[10px] text-neutral-400 mt-1 text-right">
                      {dayjs(msg.timestamp).format('A h:mm')}
                    </div>
                  ) : (
                    <></>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Input */}
      <div className="border-t border-neutral-700 px-3 py-2 flex-none">
        <div className="flex items-center gap-2">
          <Input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.nativeEvent.isComposing) return;
              if (e.key === 'Enter') {
                e.preventDefault();
                e.stopPropagation();
                handleSend();
              }
            }}
            placeholder="Type message here"
            className="flex-1 rounded-full px-4 py-2 text-sm bg-[#2a2a2a] text-white border border-neutral-600 outline-none"
          />
          <button
            onClick={handleSend}
            className="cursor-pointer p-2 rounded-full bg-blue-600 hover:bg-blue-700"
          >
            <Send className="w-4 h-4 text-white" />
          </button>
        </div>
      </div>
    </div>
  );
}

function getParticipantInfo(
  participantId: string,
  users: DiscussionParticipant[],
  participants: Participant[],
) {
  const user = users.find((u) => u.participant_id === participantId);
  if (!user) return null;

  const participant = participants.find((p) => p.id === user.user_id);
  return participant || null;
}
