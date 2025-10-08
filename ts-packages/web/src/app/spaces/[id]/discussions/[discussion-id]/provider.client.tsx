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
import type { UseSuspenseQueryResult } from '@tanstack/react-query';
import {
  ConsoleLogger,
  DefaultDeviceController,
  DefaultMeetingSession,
  LogLevel,
  MeetingSessionConfiguration,
} from 'amazon-chime-sdk-js';
import { useNavigate } from 'react-router';
import {
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
  changeIsFirstClicked: (clicked: boolean) => void;
  isVideoOn: boolean;
  changeIsVideoOn: (videoOn: boolean) => void;
  isSharing: boolean;
  changeIsSharing: (sharing: boolean) => void;
  isAudioOn: boolean;
  changeIsAudioOn: (audioOn: boolean) => void;
  isRecording: boolean;
  changeIsRecording: (recording: boolean) => void;
  videoTiles: { tileId: number; attendeeId: string }[];
  changeVideoTiles: (tiles: { tileId: number; attendeeId: string }[]) => void;
  meetingSession: DefaultMeetingSession | null;
  changeMeetingSession: (session: DefaultMeetingSession | null) => void;
  remoteContentTileOwner: string | null;
  changeRemoteContentTileOwner: (owner: string | null) => void;
  micStates: Record<string, boolean>;
  changeMicStates: (states: Record<string, boolean>) => void;
  videoStates: Record<string, boolean>;
  changeVideoStates: (states: Record<string, boolean>) => void;
  messages: { senderId: string; text: string; timestamp: number }[];
  changeMessages: (
    msgs: { senderId: string; text: string; timestamp: number }[],
  ) => void;
  activePanel: 'participants' | 'chat' | null | undefined;
  changeActivePanel: (
    panel: 'participants' | 'chat' | null | undefined,
  ) => void;
  participants: Participant[];
  changeParticipants: (newParticipants: Participant[]) => void;
  focusedAttendeeId: string | null;
  changeFocusedAttendeeId: (attendeeId: string | null) => void;

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
  const navigate = useNavigate();

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
      navigate(route.deliberationSpaceById(spaceId));
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
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

      session.audioVideo.setDeviceLabelTrigger(async () => {
        const stream = await navigator.mediaDevices.getUserMedia({
          audio: true,
          video: true,
        });
        return stream;
      });
      const selfAttendeeId = configuration.credentials?.attendeeId;
      if (selfAttendeeId) {
        setMicStates((prev) => ({ ...prev, [selfAttendeeId]: false }));
      }

      setMeetingSession(session);
    }

    startChime();
    // eslint-disable-next-line react-hooks/exhaustive-deps
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [meetingSession, users]);

  useEffect(() => {
    if (!meetingSession) return;

    const av = meetingSession.audioVideo;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const onRecordingMessage = (dataMessage: any) => {
      const text = new TextDecoder().decode(dataMessage.data);
      if (text === 'start') {
        changeIsRecording(true);
      } else if (text === 'stop') {
        changeIsRecording(false);
      }
    };

    av.realtimeSubscribeToReceiveDataMessage(
      'recording-status',
      onRecordingMessage,
    );

    return () => {
      av.realtimeUnsubscribeFromReceiveDataMessage('recording-status');
    };
  }, [meetingSession]);

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

  const changeIsFirstClicked = (clicked: boolean) => {
    setIsFirstClicked(clicked);
  };

  const changeIsVideoOn = (videoOn: boolean) => {
    setIsVideoOn(videoOn);
  };

  const changeIsSharing = (sharing: boolean) => {
    setIsSharing(sharing);
  };

  const changeIsAudioOn = (audioOn: boolean) => {
    setIsAudioOn(audioOn);
  };

  const changeIsRecording = (recording: boolean) => {
    setIsRecording(recording);
  };

  const changeVideoTiles = (
    tiles: { tileId: number; attendeeId: string }[],
  ) => {
    setVideoTiles(tiles);
  };

  const changeMeetingSession = (session: DefaultMeetingSession | null) => {
    setMeetingSession(session);
  };

  const changeRemoteContentTileOwner = (owner: string | null) => {
    setRemoteContentTileOwner(owner);
  };

  const changeMicStates = (states: Record<string, boolean>) => {
    setMicStates(states);
  };

  const changeVideoStates = (states: Record<string, boolean>) => {
    setVideoStates(states);
  };

  const changeMessages = (
    msgs: { senderId: string; text: string; timestamp: number }[],
  ) => {
    setMessages(msgs);
  };

  const changeActivePanel = (
    panel: 'participants' | 'chat' | null | undefined,
  ) => {
    setActivePanel(panel);
  };

  const changeParticipants = (newParticipants: Participant[]) => {
    setParticipants(newParticipants);
  };

  const changeFocusedAttendeeId = (attendeeId: string | null) => {
    setFocusedAttendeeId(attendeeId);
  };

  return (
    <Context.Provider
      value={{
        spaceId,
        discussionId,
        tileMapRef,
        isFirstClicked,
        changeIsFirstClicked,
        isVideoOn,
        changeIsVideoOn,
        isSharing,
        changeIsSharing,
        isAudioOn,
        changeIsAudioOn,
        isRecording,
        changeIsRecording,
        videoTiles,
        changeVideoTiles,
        meetingSession,
        changeMeetingSession,
        remoteContentTileOwner,
        changeRemoteContentTileOwner,
        micStates,
        changeMicStates,
        videoStates,
        changeVideoStates,
        messages,
        changeMessages,
        activePanel,
        changeActivePanel,
        participants,
        changeParticipants,
        focusedAttendeeId,
        changeFocusedAttendeeId,
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
