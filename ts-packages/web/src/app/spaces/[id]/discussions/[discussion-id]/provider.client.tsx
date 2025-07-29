'use client';

import { useDiscussionById } from '@/app/(social)/_hooks/use-discussion';
import {
  Discussion,
  DiscussionParticipant,
  exitMeetingRequest,
  participantMeetingRequest,
  startMeetingRequest,
} from '@/lib/api/models/discussion';
import { Participant } from '@/lib/api/models/meeting';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import { StateSetter } from '@/types';
import { UseSuspenseQueryResult } from '@tanstack/react-query';
import {
  ConsoleLogger,
  DefaultDeviceController,
  DefaultMeetingSession,
  LogLevel,
  MeetingSessionConfiguration,
} from 'amazon-chime-sdk-js';
import { useRouter } from 'next/navigation';
import React, {
  useContext,
  createContext,
  useRef,
  useState,
  useEffect,
} from 'react';

type ContextType = {
  spaceId: number;
  discussionId: number;

  tileMapRef: React.RefObject<Record<number, string>>;

  isFirstClicked: boolean;
  setIsFirstClicked: StateSetter<boolean>;
  isVideoOn: boolean;
  setIsVideoOn: StateSetter<boolean>;
  isSharing: boolean;
  setIsSharing: StateSetter<boolean>;
  isAudioOn: boolean;
  setIsAudioOn: StateSetter<boolean>;
  isRecording: boolean;
  setIsRecording: StateSetter<boolean>;
  videoTiles: { tileId: number; attendeeId: string }[];
  setVideoTiles: StateSetter<{ tileId: number; attendeeId: string }[]>;
  meetingSession: DefaultMeetingSession | null;
  setMeetingSession: StateSetter<DefaultMeetingSession | null>;
  remoteContentTileOwner: string | null;
  setRemoteContentTileOwner: StateSetter<string | null>;
  micStates: Record<string, boolean>;
  setMicStates: StateSetter<Record<string, boolean>>;
  videoStates: Record<string, boolean>;
  setVideoStates: StateSetter<Record<string, boolean>>;
  messages: { senderId: string; text: string; timestamp: number }[];
  setMessages: StateSetter<
    { senderId: string; text: string; timestamp: number }[]
  >;
  activePanel: 'participants' | 'chat' | null | undefined;
  setActivePanel: StateSetter<'participants' | 'chat' | null | undefined>;
  participants: Participant[];
  setParticipants: StateSetter<Participant[]>;
  focusedAttendeeId: string | null;
  setFocusedAttendeeId: StateSetter<string | null>;

  data: UseSuspenseQueryResult<Discussion>;
  discussion: Discussion;
  users: DiscussionParticipant[];

  sendMessage: (text: string) => void;
  focusedUser: DiscussionParticipant | undefined;
  focusedParticipant: Participant | undefined;
  focusedNickname: string;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  spaceId,
  discussionId,
}: {
  children: React.ReactNode;
  spaceId: number;
  discussionId: number;
}) {
  const tileMapRef = useRef<Record<number, string>>({});

  const [isFirstClicked, setIsFirstClicked] = useState(false);
  const [isVideoOn, setIsVideoOn] = useState(false);
  const [isSharing, setIsSharing] = useState(false);
  const [isAudioOn, setIsAudioOn] = useState(false);
  const [isRecording, setIsRecording] = useState(false);

  const [videoTiles, setVideoTiles] = useState<
    { tileId: number; attendeeId: string }[]
  >([]);

  const [meetingSession, setMeetingSession] =
    useState<DefaultMeetingSession | null>(null);

  const [remoteContentTileOwner, setRemoteContentTileOwner] = useState<
    string | null
  >(null);
  const [micStates, setMicStates] = useState<Record<string, boolean>>({});
  const [videoStates, setVideoStates] = useState<Record<string, boolean>>({});
  const [messages, setMessages] = useState<
    { senderId: string; text: string; timestamp: number }[]
  >([]);
  const [activePanel, setActivePanel] = useState<
    'participants' | 'chat' | null
  >();
  const [participants, setParticipants] = useState<Participant[]>([]);

  const [focusedAttendeeId, setFocusedAttendeeId] = useState<string | null>(
    null,
  );

  const { post, get } = useApiCall();
  const router = useRouter();

  const data = useDiscussionById(spaceId, discussionId);
  const discussion = data.data;
  logger.debug('params: ', spaceId, discussionId);
  logger.debug('discussion: ', discussion);

  const users = discussion.participants;

  useEffect(() => {
    if (!isFirstClicked) {
      setIsFirstClicked(true);
      history.pushState(null, '', location.href);
    }
  }, [isFirstClicked]);

  useEffect(() => {
    const cleanupMeetingSession = async () => {
      if (meetingSession) {
        const av = meetingSession.audioVideo;
        const dc = meetingSession.deviceController;

        av.stopLocalVideoTile();
        av.stop();

        try {
          const videoDevices = await dc.listVideoInputDevices();
          for (const device of videoDevices) {
            const stream = await navigator.mediaDevices.getUserMedia({
              video: { deviceId: { exact: device.deviceId } },
            });
            stream.getTracks().forEach((track) => track.stop());
          }
        } catch (err) {
          console.warn('[CLEANUP] Failed to stop video input stream', err);
        }

        dc.destroy?.();
      }

      try {
        await post(
          ratelApi.discussions.actDiscussionById(spaceId, discussionId),
          exitMeetingRequest(),
        );
      } catch (err) {
        console.error('[EXIT] Failed to send exit request', err);
      }

      setRemoteContentTileOwner(null);
    };

    const handlePopState = async () => {
      await cleanupMeetingSession();
      router.replace(route.deliberationSpaceById(spaceId));
    };

    const handleBeforeUnload = async (e: BeforeUnloadEvent) => {
      e.preventDefault();
      await cleanupMeetingSession();
    };

    const handleUnload = async () => {
      await cleanupMeetingSession();
    };

    window.addEventListener('popstate', handlePopState);
    window.addEventListener('beforeunload', handleBeforeUnload);
    window.addEventListener('unload', handleUnload);

    return () => {
      window.removeEventListener('popstate', handlePopState);
      window.removeEventListener('beforeunload', handleBeforeUnload);
      window.removeEventListener('unload', handleUnload);
    };
  }, [spaceId, discussionId]);

  useEffect(() => {
    async function startChime() {
      await post(
        ratelApi.discussions.actDiscussionById(spaceId, discussionId),
        startMeetingRequest(),
      );

      await post(
        ratelApi.discussions.actDiscussionById(spaceId, discussionId),
        participantMeetingRequest(),
      );

      const joinInfo = await get(
        ratelApi.meeting.getMeetingById(spaceId, discussionId),
      );

      setParticipants(joinInfo.participants);

      const logger = new ConsoleLogger('ChimeLogs', LogLevel.INFO);
      const deviceController = new DefaultDeviceController(logger);

      const configuration = new MeetingSessionConfiguration(
        joinInfo.meeting,
        joinInfo.attendee,
      );

      const session = new DefaultMeetingSession(
        configuration,
        logger,
        deviceController,
      );

      const audioElement = new Audio();
      audioElement.autoplay = true;
      session.audioVideo.bindAudioElement(audioElement);

      const audioInputs =
        await session.deviceController.listAudioInputDevices();
      if (audioInputs.length > 0) {
        await session.deviceController.startAudioInput(audioInputs[0].deviceId);
        session.audioVideo.realtimeMuteLocalAudio();
        const selfAttendeeId = configuration.credentials?.attendeeId;
        if (selfAttendeeId) {
          setMicStates((prev) => ({
            ...prev,
            [selfAttendeeId]: false,
          }));
        }
      }

      setMeetingSession(session);
    }

    startChime();
  }, []);

  useEffect(() => {
    if (!meetingSession) return;
    const av = meetingSession.audioVideo;
    av.start();

    const observer = {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      videoTileDidUpdate: (tileState: any) => {
        const { tileId, boundAttendeeId } = tileState;
        if (!tileId || !boundAttendeeId) return;

        tileMapRef.current[tileId] = boundAttendeeId;
        const attendeeId = tileState.boundAttendeeId;

        const isVideoOn =
          attendeeId !== meetingSession.configuration.credentials?.attendeeId
            ? true
            : tileState.boundAttendeeId &&
              tileState.tileId !== null &&
              (tileState.active || tileState.boundVideoStream !== null);

        if (!tileState.isContent && tileState.tileId && boundAttendeeId) {
          setVideoTiles((prev) => {
            const exists = prev.some(
              (tile) => tile.tileId === tileState.tileId,
            );
            if (exists) return prev;
            return [
              ...prev,
              { tileId: tileState.tileId, attendeeId: boundAttendeeId },
            ];
          });
        }

        setVideoStates((prev) => ({
          ...prev,
          [boundAttendeeId]: isVideoOn,
        }));
      },
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      videoTileWasRemoved: (tileId: any) => {
        const attendeeId = tileMapRef.current[tileId];
        if (!attendeeId) return;

        setVideoStates((prev) => ({
          ...prev,
          [attendeeId]: false,
        }));

        setVideoTiles((prev) => prev.filter((tile) => tile.tileId !== tileId));

        delete tileMapRef.current[tileId];
      },
    };

    av.addObserver(observer);
    return () => av.removeObserver(observer);
  }, [meetingSession]);

  useEffect(() => {
    if (!meetingSession) return;
    const av = meetingSession.audioVideo;

    const activeAttendeeIds = new Set<string>();

    av.realtimeSubscribeToAttendeeIdPresence((attendeeId, present) => {
      if (present) {
        activeAttendeeIds.add(attendeeId);
        data.refetch();
        av.realtimeSubscribeToVolumeIndicator(
          attendeeId,
          (_attendeeId, _volume, muted) => {
            setMicStates((prev) => {
              if (typeof muted !== 'boolean') return prev;
              return {
                ...prev,
                [attendeeId]: !muted,
              };
            });
          },
        );
      } else {
        activeAttendeeIds.delete(attendeeId);
        data.refetch();
        setMicStates((prev) => {
          const copy = { ...prev };
          delete copy[attendeeId];
          return copy;
        });
      }
    });

    return () => {
      activeAttendeeIds.forEach((id) => {
        av.realtimeUnsubscribeFromVolumeIndicator(id);
      });
    };
  }, [meetingSession]);

  useEffect(() => {
    if (!meetingSession) return;
    const av = meetingSession.audioVideo;

    const topic = 'chat';
    const chatSound = new Audio('/sounds/chat.wav');
    chatSound.volume = 0.5;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const onMessageReceived = (dataMessage: any) => {
      const senderId = dataMessage.senderAttendeeId;
      const text = new TextDecoder('utf-8').decode(dataMessage.data);
      const timestamp = Date.now();

      setMessages((prev) => [...prev, { senderId, text, timestamp }]);

      chatSound.play();
    };

    av.realtimeSubscribeToReceiveDataMessage(topic, onMessageReceived);

    return () => {
      av.realtimeUnsubscribeFromReceiveDataMessage(topic);
    };
  }, [meetingSession]);

  const exitedAttendeesRef = useRef<Set<string>>(new Set());

  useEffect(() => {
    if (!meetingSession || !users || users.length === 0) return;
    const av = meetingSession.audioVideo;

    const handlePresenceChange = async (
      attendeeId: string,
      present: boolean,
    ) => {
      const selfAttendeeId =
        meetingSession.configuration.credentials?.attendeeId;

      if (attendeeId === selfAttendeeId && !present) {
        return;
      }

      if (present) {
        exitedAttendeesRef.current.delete(attendeeId);
      } else {
        exitedAttendeesRef.current.add(attendeeId);
      }

      try {
        const joinInfo = await get(
          ratelApi.meeting.getMeetingById(spaceId, discussionId),
        );

        setParticipants((prev) => {
          const incomingIds = new Set(
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            joinInfo.participants.map((p: any) => p.id),
          );
          return [
            ...prev.filter((p) => incomingIds.has(p.id)),
            ...joinInfo.participants.filter(
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
              (p: any) => !prev.some((pp) => pp.id === p.id),
            ),
          ];
        });
      } catch (err) {
        console.error('Failed to update participants:', err);
      }
    };

    av.realtimeSubscribeToAttendeeIdPresence(handlePresenceChange);
    return () => {
      av.realtimeUnsubscribeToAttendeeIdPresence(handlePresenceChange);
    };
  }, [meetingSession, users]);

  const sendMessage = (text: string) => {
    if (!meetingSession || !text.trim()) return;

    setMessages((prev) => [
      ...prev,
      {
        senderId: meetingSession.configuration.credentials?.attendeeId ?? '',
        text: text.trim(),
        timestamp: Date.now(),
      },
    ]);

    try {
      const av = meetingSession.audioVideo;
      const topic = 'chat';
      const data = new TextEncoder().encode(text.trim());

      av.realtimeSendDataMessage(topic, data, 10000);
    } catch (err) {
      logger.error('[SEND] failed to send message:', err);
    }
  };

  const focusedUser = users.find((u) => u.participant_id === focusedAttendeeId);
  const focusedParticipant = participants.find(
    (p) => p.id === focusedUser?.user_id,
  );
  const focusedNickname =
    focusedParticipant?.nickname ?? focusedParticipant?.username ?? '';

  return (
    <Context.Provider
      value={{
        spaceId,
        discussionId,
        tileMapRef,
        isFirstClicked,
        setIsFirstClicked,
        isVideoOn,
        setIsVideoOn,
        isSharing,
        setIsSharing,
        isAudioOn,
        setIsAudioOn,
        isRecording,
        setIsRecording,
        videoTiles,
        setVideoTiles,
        meetingSession,
        setMeetingSession,
        remoteContentTileOwner,
        setRemoteContentTileOwner,
        micStates,
        setMicStates,
        videoStates,
        setVideoStates,
        messages,
        setMessages,
        activePanel,
        setActivePanel,
        participants,
        setParticipants,
        focusedAttendeeId,
        setFocusedAttendeeId,
        data,
        discussion,
        users,
        sendMessage,
        focusedUser,
        focusedParticipant,
        focusedNickname,
      }}
    >
      {children}
    </Context.Provider>
  );
}

export function useDiscussionContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context does not be provided. Please wrap your component with ClientProviders.',
    );
  return context;
}
