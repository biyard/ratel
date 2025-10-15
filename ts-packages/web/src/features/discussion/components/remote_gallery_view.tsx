import { DefaultMeetingSession } from 'amazon-chime-sdk-js';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { useRef, useState, useEffect, useMemo } from 'react';
import {
  DiscussionParticipantResponse,
  DiscussionUser,
} from '../utils/discussion.v3';

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
  participants: DiscussionUser[];
  u: DiscussionParticipantResponse[];
  focusedAttendeeId: string | null;
  setFocusedAttendeeId: (attendeeId: string | null) => void;
}) {
  const users = useMemo(() => {
    const seen = new Set<string>();
    return u.filter((x) => {
      const k = x.participant_id;
      if (!k || seen.has(k)) return false;
      seen.add(k);
      return true;
    });
  }, [u]);

  const containerRef = useRef<HTMLDivElement>(null);
  const rowRef = useRef<HTMLDivElement>(null);
  const [scrollIndex, setScrollIndex] = useState(0);

  const tileWidth = 220;
  const [visibleCount, setVisibleCount] = useState(1);

  const DESIRED_CONTAINER_PX = 1400;
  const MIN_CONTAINER_PX = 720;
  const SIDE_PADDING_PX = 48;
  const ARROWS_AREA_PX = 96;
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

  const [colGap, setColGap] = useState(8);
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
  const userByUserPk = useMemo(() => {
    const m = new Map<string, DiscussionParticipantResponse>();
    users.forEach((uu) => m.set(uu.user_pk, uu));
    return m;
  }, [users]);

  const attendeeTileMap = useMemo(() => {
    const m = new Map<string, number>();
    videoTiles.forEach(({ attendeeId, tileId }) => m.set(attendeeId, tileId));
    return m;
  }, [videoTiles]);

  const uniqueParticipantsByUser = useMemo(() => {
    const seen = new Set<string>();
    const out: DiscussionUser[] = [];
    participants.forEach((p) => {
      const k = p.user_pk;
      if (!k || seen.has(k)) return;
      seen.add(k);
      out.push(p);
    });
    return out;
  }, [participants]);

  const sortedParticipants = useMemo(() => {
    const selfUserPk = users.find(
      (uu) => uu.participant_id === selfAttendeeId,
    )?.user_pk;
    if (!selfUserPk) return uniqueParticipantsByUser;
    const self = uniqueParticipantsByUser.find((p) => p.user_pk === selfUserPk);
    const others = uniqueParticipantsByUser.filter(
      (p) => p.user_pk !== selfUserPk,
    );
    return self ? [self, ...others] : others;
  }, [uniqueParticipantsByUser, users, selfAttendeeId]);

  const galleryItems = useMemo(() => {
    const items = sortedParticipants
      .map((p) => {
        const attendeeId = userByUserPk.get(p.user_pk)?.participant_id;
        return attendeeId && attendeeId !== selfAttendeeId
          ? { p, attendeeId }
          : null;
      })
      .filter(Boolean) as { p: DiscussionUser; attendeeId: string }[];
    const seen = new Set<string>();
    const out: { p: DiscussionUser; attendeeId: string }[] = [];
    for (const it of items) {
      if (seen.has(it.attendeeId)) continue;
      seen.add(it.attendeeId);
      out.push(it);
    }
    return out;
  }, [sortedParticipants, userByUserPk, selfAttendeeId]);

  const videoRefs = useRef<Map<string, HTMLVideoElement>>(new Map());

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

  const maxIndex = useMemo(() => {
    const pages = Math.ceil(
      Math.max(1, galleryItems.length) / Math.max(1, visibleCount),
    );
    return Math.max(0, pages - 1);
  }, [galleryItems.length, visibleCount]);

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
  }, [galleryItems.length, visibleCount, stride, containerWidthPx]); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    galleryItems.forEach(({ attendeeId }) => {
      const tileId = attendeeTileMap.get(attendeeId);
      if (!tileId) return;
      const el = videoRefs.current.get(attendeeId);
      if (el) meetingSession.audioVideo.bindVideoElement(tileId, el);
    });
    return () => {
      galleryItems.forEach(({ attendeeId }) => {
        const tileId = attendeeTileMap.get(attendeeId);
        if (tileId) meetingSession.audioVideo.unbindVideoElement(tileId);
      });
    };
  }, [galleryItems, attendeeTileMap, meetingSession.audioVideo]); // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <div className="flex flex-row w-full items-center justify-center bg-neutral-800 border-b border-neutral-700 px-6 py-4">
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
        style={{ width: `${containerWidthPx}px` }}
      >
        <div
          ref={rowRef}
          className="flex flex-row gap-2"
          style={{
            width: `${Math.max(1, galleryItems.length) * stride - colGap}px`,
          }}
        >
          {galleryItems.map(({ p, attendeeId }) => {
            const tileId = attendeeTileMap.get(attendeeId);
            const hasVideo = tileId !== undefined;
            const nickname = p.author_username ?? p.author_username;
            return (
              <div
                key={attendeeId}
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
                        if (el) videoRefs.current.set(attendeeId, el);
                        else videoRefs.current.delete(attendeeId);
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
