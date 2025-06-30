'use client';

import React, { useEffect, useRef, useState } from 'react';

import { DefaultMeetingSession } from 'amazon-chime-sdk-js';

export default function LocalVideo({
  meetingSession,
  isVideoOn,
}: {
  meetingSession: DefaultMeetingSession;
  isVideoOn: boolean;
}) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [started, setStarted] = useState(false);

  useEffect(() => {
    const av = meetingSession.audioVideo;

    const init = async () => {
      try {
        const devices = await av.listVideoInputDevices();
        if (devices.length > 0) {
          await av.startVideoInput(devices[0].deviceId);
        } else {
          console.warn('No video input devices found.');
          return;
        }

        if (isVideoOn) {
          av.startLocalVideoTile();
        }

        if (videoRef.current) {
          videoRef.current.id = 'local-video-element';
        }
        setStarted(true);
      } catch (err) {
        console.error('Failed to init local video:', err);
      }
    };

    init();

    const observer = {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      videoTileDidUpdate: (tileState: any) => {
        if (tileState.localTile && videoRef.current) {
          av.bindVideoElement(tileState.tileId, videoRef.current);
        }
      },
    };

    av.addObserver(observer);

    return () => {
      av.removeObserver(observer);
      av.stopLocalVideoTile();
      av.stop();
    };
  }, [meetingSession]);

  useEffect(() => {
    const av = meetingSession.audioVideo;
    if (!started) return;

    if (isVideoOn) {
      av.startLocalVideoTile();
    } else {
      av.stopLocalVideoTile();
    }
  }, [isVideoOn]);

  return (
    <video
      ref={videoRef}
      id="local-video-element"
      className="absolute top-0 left-0 w-full h-full bg-black object-cover"
      autoPlay
      muted
      onLoadedMetadata={() => console.log('Video metadata loaded')}
      onCanPlay={() => console.log('Video can play')}
      onPlay={() => console.log('Video playing')}
      onError={(e) => console.error('Video error:', e)}
    />
  );
}
