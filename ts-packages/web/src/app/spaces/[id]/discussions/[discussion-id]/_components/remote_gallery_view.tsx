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
  u,
  focusedAttendeeId,
  setFocusedAttendeeId,
}: {
  meetingSession: DefaultMeetingSession;
  videoTiles: { tileId: number; attendeeId: string }[];
  participants: Participant[];
  u: DiscussionParticipant[];
  focusedAttendeeId: string | null;
  setFocusedAttendeeId: (attendeeId: string | null) => void;
}) {
  const users = u.filter(
    (user, index, self) =>
      index === self.findIndex((u) => u.participant_id === user.participant_id),
  );

  const containerRef = useRef<HTMLDivElement>(null);
  const rowRef = useRef<HTMLDivElement>(null);
  const [scrollIndex, setScrollIndex] = useState(0);

  const tileWidth = 220;
  const [visibleCount, setVisibleCount] = useState(1);

  // === [추가] 컨테이너 길이: 원하는 길이 vs 화면 가용폭 중 작은 값 ===
  const DESIRED_CONTAINER_PX = 1400; // “조금 더 긴” 목표 길이
  const MIN_CONTAINER_PX = 720;
  const SIDE_PADDING_PX = 48; // px-6 좌우
  const ARROWS_AREA_PX = 96; // 양 옆 버튼 영역 대략
  const [containerWidthPx, setContainerWidthPx] = useState(MIN_CONTAINER_PX);

  const resizeContainerWidth = () => {
    const vw =
      typeof window !== 'undefined' ? window.innerWidth : DESIRED_CONTAINER_PX;
    const available = Math.max(0, vw - SIDE_PADDING_PX - ARROWS_AREA_PX);
    const w = Math.min(
      DESIRED_CONTAINER_PX,
      Math.max(MIN_CONTAINER_PX, available),
    );
    setContainerWidthPx(w);
  };
  useEffect(() => {
    resizeContainerWidth();
    window.addEventListener('resize', resizeContainerWidth);
    return () => window.removeEventListener('resize', resizeContainerWidth);
  }, []);

  // === [추가] gap 포함한 stride 계산 ===
  const [colGap, setColGap] = useState(8); // Tailwind gap-2 기본 8px
  useEffect(() => {
    const readGap = () => {
      if (!rowRef.current) return;
      const cs = getComputedStyle(rowRef.current);
      const g = parseFloat(cs.columnGap || '0');
      if (!Number.isNaN(g)) setColGap(g || 0);
    };
    readGap();
    window.addEventListener('resize', readGap);
    return () => window.removeEventListener('resize', readGap);
  }, []);
  const stride = tileWidth + colGap;

  const selfAttendeeId = meetingSession.configuration.credentials?.attendeeId;
  const selfUser = users.find((u) => u.participant_id === selfAttendeeId);
  const selfUserId = selfUser?.user_id;

  const maxIndex = useMemo(() => {
    const pages = Math.ceil(
      Math.max(1, users.length) / Math.max(1, visibleCount),
    );
    return Math.max(0, pages - 1);
  }, [users.length, visibleCount]);

  const attendeeTileMap = useMemo(() => {
    return new Map(
      videoTiles.map(({ attendeeId, tileId }) => [attendeeId, tileId]),
    );
  }, [videoTiles]);

  const videoRefs = useRef<Map<string, HTMLVideoElement>>(new Map());

  // === [치환] 가시 카드 수 계산: stride / 컨테이너 실제 너비 기준 ===
  useEffect(() => {
    const updateVisibleCount = () => {
      const containerWidth =
        containerWidthPx ||
        (containerRef.current ? containerRef.current.offsetWidth : 0);
      const count = Math.floor((containerWidth + colGap) / Math.max(1, stride));
      setVisibleCount(Math.max(1, count));
    };
    updateVisibleCount();
    window.addEventListener('resize', updateVisibleCount);
    return () => window.removeEventListener('resize', updateVisibleCount);
  }, [containerWidthPx, stride, colGap]);

  const scrollTo = (index: number) => {
    const clampedIndex = Math.max(0, Math.min(index, maxIndex));
    setScrollIndex(clampedIndex);
    if (containerRef.current) {
      containerRef.current.scrollTo({
        left: clampedIndex * visibleCount * stride,
        behavior: 'smooth',
      });
    }
  };

  useEffect(() => {
    scrollTo(scrollIndex);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [users.length, visibleCount, stride, containerWidthPx]);

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
        className="overflow-x-hidden scroll-smooth no-scrollbar"
        style={{ width: `${containerWidthPx}px` }} // [추가] 동적 컨테이너 폭
      >
        <div
          ref={rowRef}
          className="flex flex-row gap-2"
          style={{ width: `${users.length * stride - colGap}px` }} // [치환] gap 포함
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
                <div className="w-[200px] h-[130px] bg-neutral-700 rounded-md overflow-hidden relative ring-2 ring-transparent">
                  {hasVideo ? (
                    <video
                      onClick={() => {
                        if (
                          typeof window !== 'undefined' &&
                          window.innerWidth <= 768
                        ) {
                          setFocusedAttendeeId(
                            focusedAttendeeId === attendeeId
                              ? null
                              : attendeeId,
                          );
                        }
                      }}
                      onDoubleClick={() =>
                        setFocusedAttendeeId(
                          focusedAttendeeId === attendeeId ? null : attendeeId,
                        )
                      }
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
