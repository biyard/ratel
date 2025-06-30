'use client';

import { logger } from '@/lib/logger';
import React, { useEffect, useRef } from 'react';

import { DefaultMeetingSession } from 'amazon-chime-sdk-js';

export default function RemoteContentShareVideo({
  meetingSession,
  onRemoteContentTileUpdate,
}: {
  meetingSession: DefaultMeetingSession;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  onRemoteContentTileUpdate: (tileState: any) => void;
}) {
  const remoteContentRef = useRef<HTMLVideoElement>(null);
  const boundContentTileIdRef = useRef<number | null>(null);

  useEffect(() => {
    const av = meetingSession.audioVideo;

    if (!av) {
      return;
    }

    const observer = {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      videoTileDidUpdate: (tileState: any) => {
        const isContent = tileState.isContent && !tileState.localTile;
        const alreadyBound = boundContentTileIdRef.current === tileState.tileId;

        if (isContent && tileState.tileId) {
          onRemoteContentTileUpdate(tileState);
        }

        if (
          isContent &&
          tileState.tileId &&
          remoteContentRef.current &&
          !alreadyBound
        ) {
          av.bindVideoElement(tileState.tileId, remoteContentRef.current);
          boundContentTileIdRef.current = tileState.tileId;
        }
      },

      videoTileWasRemoved: (tileId: number) => {
        logger.debug('[DEBUG] Tile removed:', tileId);
        if (remoteContentRef.current) {
          remoteContentRef.current.srcObject = null;
        }

        onRemoteContentTileUpdate(null);
      },
    };

    av.addObserver(observer);

    const tiles = av.getAllVideoTiles();

    tiles.forEach((tile) => {
      const state = tile.state();
      logger.debug('[FORCE CHECK] existing tile:', {
        tileId: state.tileId,
        isContent: state.isContent,
        localTile: state.localTile,
        active: state.active,
        attendeeId: state.boundAttendeeId,
        boundVideoElement: state.boundVideoElement,
      });
    });

    av.realtimeSubscribeToAttendeeIdPresence((attendeeId, present) => {
      if (attendeeId.includes('#content')) {
        logger.debug('[CHECK] #content stream presence:', attendeeId, present);
      }
    });

    return () => {
      av.removeObserver(observer);
    };
  }, [meetingSession]);

  return (
    <video
      id="remote-content-video"
      ref={remoteContentRef}
      className="absolute top-0 left-0 w-full h-full object-contain z-0"
      autoPlay
      muted
    />
  );
}
