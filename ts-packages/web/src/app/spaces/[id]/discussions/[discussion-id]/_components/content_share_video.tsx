'use client';

import React, { useEffect, useRef } from 'react';

import { DefaultMeetingSession } from 'amazon-chime-sdk-js';

export default function ContentShareVideo({
  meetingSession,
}: {
  meetingSession: DefaultMeetingSession;
}) {
  const contentRef = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    const av = meetingSession.audioVideo;

    const observer = {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      videoTileDidUpdate: (tileState: any) => {
        if (tileState.isContent && tileState.tileId && contentRef.current) {
          av.bindVideoElement(tileState.tileId, contentRef.current);
        }
      },
    };

    av.addObserver(observer);

    return () => {
      av.removeObserver(observer);
    };
  }, [meetingSession]);

  return (
    <video
      ref={contentRef}
      className="absolute top-0 left-0 w-full h-full object-contain bg-black z-0"
      autoPlay
      muted
    />
  );
}
