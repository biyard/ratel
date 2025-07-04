'use client';

import { DiscussionParticipant } from '@/lib/api/models/discussion';
import { Participant } from '@/lib/api/models/meeting';
import { DefaultMeetingSession } from 'amazon-chime-sdk-js';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import React, { useRef, useState, useEffect, useMemo } from 'react';

export default function RemoteGalleryView({
  meetingSession,
  videoTiles,
  participants,
  users,
}: {
  meetingSession: DefaultMeetingSession;
  videoTiles: { tileId: number; attendeeId: string }[];
  participants: Participant[];
  users: DiscussionParticipant[];
}) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollIndex, setScrollIndex] = useState(0);
  const tileWidth = 220;
  const [visibleCount, setVisibleCount] = useState(1);

  const selfAttendeeId = meetingSession.configuration.credentials?.attendeeId;
  const selfUser = users.find((u) => u.participant_id === selfAttendeeId);
  const selfUserId = selfUser?.user_id;

  const maxIndex = useMemo(() => {
    return Math.max(0, Math.ceil(users.length / visibleCount) - 1);
  }, [users.length, visibleCount]);

  const attendeeTileMap = useMemo(() => {
    return new Map(
      videoTiles.map(({ attendeeId, tileId }) => [attendeeId, tileId]),
    );
  }, [videoTiles]);

  const videoRefs = useRef<Map<string, HTMLVideoElement>>(new Map());

  useEffect(() => {
    const updateVisibleCount = () => {
      if (containerRef.current) {
        const containerWidth = containerRef.current.offsetWidth;
        const count = Math.floor(containerWidth / tileWidth);
        setVisibleCount(Math.max(1, count));
      }
    };
    updateVisibleCount();
    window.addEventListener('resize', updateVisibleCount);
    return () => window.removeEventListener('resize', updateVisibleCount);
  }, []);

  const scrollTo = (index: number) => {
    const clampedIndex = Math.max(0, Math.min(index, maxIndex));
    setScrollIndex(clampedIndex);
    if (containerRef.current) {
      containerRef.current.scrollTo({
        left: clampedIndex * visibleCount * tileWidth,
        behavior: 'smooth',
      });
    }
  };

  useEffect(() => {
    scrollTo(scrollIndex);
  }, [users.length, visibleCount]);

  const sortedParticipants = useMemo(() => {
    if (!selfUserId) return participants;
    const self = participants.find((p) => p.id === selfUserId);
    const others = participants.filter((p) => p.id !== selfUserId);
    return self ? [self, ...others] : others;
  }, [participants, selfUserId]);

  useEffect(() => {
    videoTiles.forEach(({ attendeeId, tileId }) => {
      if (attendeeId === selfAttendeeId) return;

      const el = videoRefs.current.get(attendeeId);
      if (el) {
        meetingSession.audioVideo.bindVideoElement(tileId, el);
      }
    });

    return () => {
      videoTiles.forEach(({ tileId }) => {
        meetingSession.audioVideo.unbindVideoElement(tileId);
      });
    };
  }, [videoTiles]);

  return (
    <div className="flex flex-row w-full items-center justify-center bg-neutral-800 px-6 py-4">
      {scrollIndex > 0 ? (
        <button
          onClick={() => scrollTo(scrollIndex - 1)}
          className="flex flex-row w-fit h-fit"
        >
          <ChevronLeft width={24} height={24} />
        </button>
      ) : (
        <div className="w-[24px]" />
      )}

      <div
        ref={containerRef}
        className="w-[1100px] overflow-x-hidden scroll-smooth no-scrollbar"
      >
        <div
          className="flex flex-row gap-2"
          style={{ width: `${users.length * tileWidth}px` }}
        >
          {sortedParticipants.map((p) => {
            const user = users.find((u) => u.user_id === p.id);
            const attendeeId = user?.participant_id;

            if (!attendeeId || attendeeId === selfAttendeeId) return null;

            const tileId = attendeeTileMap.get(attendeeId);
            const hasVideo = tileId !== undefined;
            const nickname = p.nickname ?? p.username;

            return (
              <div
                key={p.id}
                className="w-[220px] h-[130px] shrink-0 flex items-center justify-center relative"
              >
                <div className="w-[200px] h-[130px] bg-neutral-700 rounded-md overflow-hidden relative">
                  {hasVideo ? (
                    <video
                      ref={(el) => {
                        if (el) {
                          videoRefs.current.set(attendeeId, el);
                        }
                      }}
                      autoPlay
                      muted={false}
                      className="w-full h-full object-cover rounded-md bg-neutral-700"
                    />
                  ) : null}
                  <div className="absolute bottom-2 left-2 text-sm text-white w-fit max-w-[100px] h-fit px-[10px] py-[5px] bg-neutral-800 rounded-lg overflow-hidden text-ellipsis whitespace-nowrap">
                    {nickname}
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {scrollIndex < maxIndex ? (
        <button
          onClick={() => scrollTo(scrollIndex + 1)}
          className="flex flex-row w-fit h-fit"
        >
          <ChevronRight width={24} height={24} />
        </button>
      ) : (
        <div className="w-[24px]" />
      )}
    </div>
  );
}
