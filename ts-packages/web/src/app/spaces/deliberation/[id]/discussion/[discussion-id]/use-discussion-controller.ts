import { useRef, useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router';
import type { UseSuspenseQueryResult } from '@tanstack/react-query';
import {
  ConsoleLogger,
  DefaultDeviceController,
  DefaultMeetingSession,
  LogLevel,
  MeetingSessionConfiguration,
} from 'amazon-chime-sdk-js';

import { useApiCall } from '@/lib/api/use-send';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import useDiscussionById from '@/features/spaces/discussions/hooks/use-discussion';
import { useExitMeetingMutation } from '@/features/spaces/discussions/hooks/use-exit-meeting-mutation';
import { useStartMeetingMutation } from '@/features/spaces/discussions/hooks/use-start-meeting-mutation';
import { useParticipantMeetingMutation } from '@/features/spaces/discussions/hooks/use-participant-meeting-mutation';
import { useDiscussionMeetingMutation } from '@/features/spaces/discussions/hooks/use-discussion-meeting';
import {
  DeliberationDiscussionResponse,
  DiscussionParticipantResponse,
  DiscussionUser,
} from '@/features/discussion/utils/discussion.v3';

type ControllerDeps = ReturnType<typeof buildDeps>;

export class DiscussionMeetingController {
  private deps: ControllerDeps;
  constructor(deps: ControllerDeps) {
    this.deps = deps;
  }

  get spacePk() {
    return this.deps.spacePk;
  }
  get discussionPk() {
    return this.deps.discussionPk;
  }

  get tileMapRef() {
    return this.deps.refs.tileMapRef;
  }

  get isFirstClicked() {
    return this.deps.state.isFirstClicked;
  }
  changeIsFirstClicked = (v: boolean) => this.deps.setters.setIsFirstClicked(v);

  get isVideoOn() {
    return this.deps.state.isVideoOn;
  }
  changeIsVideoOn = (v: boolean) => this.deps.setters.setIsVideoOn(v);

  get isSharing() {
    return this.deps.state.isSharing;
  }
  changeIsSharing = (v: boolean) => this.deps.setters.setIsSharing(v);

  get isAudioOn() {
    return this.deps.state.isAudioOn;
  }
  changeIsAudioOn = (v: boolean) => this.deps.setters.setIsAudioOn(v);

  get isRecording() {
    return this.deps.state.isRecording;
  }
  changeIsRecording = (v: boolean) => this.deps.setters.setIsRecording(v);

  get videoTiles() {
    return this.deps.state.videoTiles;
  }
  changeVideoTiles = (v: { tileId: number; attendeeId: string }[]) =>
    this.deps.setters.setVideoTiles(v);

  get meetingSession() {
    return this.deps.state.meetingSession;
  }
  changeMeetingSession = (v: DefaultMeetingSession | null) =>
    this.deps.setters.setMeetingSession(v);

  get remoteContentTileOwner() {
    return this.deps.state.remoteContentTileOwner;
  }
  changeRemoteContentTileOwner = (v: string | null) =>
    this.deps.setters.setRemoteContentTileOwner(v);

  get micStates() {
    return this.deps.state.micStates;
  }
  changeMicStates = (v: Record<string, boolean>) =>
    this.deps.setters.setMicStates(v);

  get videoStates() {
    return this.deps.state.videoStates;
  }
  changeVideoStates = (v: Record<string, boolean>) =>
    this.deps.setters.setVideoStates(v);

  get messages() {
    return this.deps.state.messages;
  }
  changeMessages = (
    v: { senderId: string; text: string; timestamp: number }[],
  ) => this.deps.setters.setMessages(v);

  get activePanel() {
    return this.deps.state.activePanel;
  }
  changeActivePanel = (v: 'participants' | 'chat' | null | undefined) =>
    this.deps.setters.setActivePanel(v);

  get participants() {
    return this.deps.state.participants;
  }
  changeParticipants = (v: DiscussionUser[]) =>
    this.deps.setters.setParticipants(v);

  get focusedAttendeeId() {
    return this.deps.state.focusedAttendeeId;
  }
  changeFocusedAttendeeId = (v: string | null) =>
    this.deps.setters.setFocusedAttendeeId(v);

  get data(): UseSuspenseQueryResult<DeliberationDiscussionResponse> {
    return this.deps.data;
  }
  get discussion(): DeliberationDiscussionResponse {
    return this.deps.discussion;
  }
  get users(): DiscussionParticipantResponse[] {
    return this.deps.users;
  }

  sendMessage = (text: string) => this.deps.handlers.sendMessage(text);

  get focusedUser() {
    return this.deps.derived.focusedUser;
  }
  get focusedParticipant() {
    return this.deps.derived.focusedParticipant;
  }
  get focusedNickname() {
    return this.deps.derived.focusedNickname;
  }
}

export function useDiscussionMeetingController(
  spacePk: string,
  discussionPk: string,
) {
  const deps = buildDeps(spacePk, discussionPk);
  return useMemo(() => new DiscussionMeetingController(deps), [deps.token]);
}

function buildDeps(spacePk: string, discussionPk: string) {
  const discussionMeetingMutation = useDiscussionMeetingMutation();
  const exitMeetingMutation = useExitMeetingMutation();
  const startMeetingMutation = useStartMeetingMutation();
  const participantMeetingMutation = useParticipantMeetingMutation();

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
  const [participants, setParticipants] = useState<DiscussionUser[]>([]);
  const [focusedAttendeeId, setFocusedAttendeeId] = useState<string | null>(
    null,
  );

  const { post, get } = useApiCall();
  const navigate = useNavigate();

  const data = useDiscussionById(spacePk, discussionPk);

  const discussion = data.data;
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
        } catch (err) {}
        dc.destroy?.();
      }
      try {
        await exitMeetingMutation.mutateAsync({
          spacePk,
          discussionPk,
        });
      } catch (err) {}
      setRemoteContentTileOwner(null);
    };

    const handlePopState = async () => {
      await cleanupMeetingSession();
      navigate(route.deliberationSpaceById(spacePk));
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
  }, [spacePk, discussionPk, meetingSession, navigate, post]);

  useEffect(() => {
    async function startChime() {
      await startMeetingMutation.mutateAsync({
        spacePk,
        discussionPk,
      });

      await participantMeetingMutation.mutateAsync({
        spacePk,
        discussionPk,
      });

      data.refetch();

      const joinInfo = await discussionMeetingMutation.mutateAsync({
        spacePk,
        discussionPk,
      });

      setParticipants(joinInfo.participants);
      const chimeLogger = new ConsoleLogger('ChimeLogs', LogLevel.INFO);
      const deviceController = new DefaultDeviceController(chimeLogger);
      const configuration = new MeetingSessionConfiguration(
        joinInfo.meeting,
        joinInfo.attendee,
      );
      const session = new DefaultMeetingSession(
        configuration,
        chimeLogger,
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
      if (selfAttendeeId)
        setMicStates((prev) => ({ ...prev, [selfAttendeeId]: false }));
      setMeetingSession(session);
    }
    startChime();
  }, [spacePk, discussionPk, post, get]);

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
        const videoOn =
          attendeeId !== meetingSession.configuration.credentials?.attendeeId
            ? true
            : tileState.boundAttendeeId &&
              tileState.tileId !== null &&
              (tileState.active || tileState.boundVideoStream !== null);
        if (!tileState.isContent && tileState.tileId && boundAttendeeId) {
          setVideoTiles((prev) => {
            const exists = prev.some((t) => t.tileId === tileState.tileId);
            return exists
              ? prev
              : [
                  ...prev,
                  { tileId: tileState.tileId, attendeeId: boundAttendeeId },
                ];
          });
        }
        setVideoStates((prev) => ({ ...prev, [boundAttendeeId]: videoOn }));
      },
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      videoTileWasRemoved: (tileId: any) => {
        const attendeeId = tileMapRef.current[tileId];
        if (!attendeeId) return;
        setVideoStates((prev) => ({ ...prev, [attendeeId]: false }));
        setVideoTiles((prev) => prev.filter((t) => t.tileId !== tileId));
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
        av.realtimeSubscribeToVolumeIndicator(attendeeId, (_id, _v, muted) => {
          setMicStates((prev) =>
            typeof muted !== 'boolean'
              ? prev
              : { ...prev, [attendeeId]: !muted },
          );
        });
      } else {
        activeAttendeeIds.delete(attendeeId);
        data.refetch();
        setMicStates((prev) => {
          const cp = { ...prev };
          delete cp[attendeeId];
          return cp;
        });
      }
    });
    return () => {
      activeAttendeeIds.forEach((id) =>
        av.realtimeUnsubscribeFromVolumeIndicator(id),
      );
    };
  }, [meetingSession, data]);

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
    return () => av.realtimeUnsubscribeFromReceiveDataMessage(topic);
  }, [meetingSession]);

  const exitedAttendeesRef = useRef<Set<string>>(new Set());

  useEffect(() => {
    if (!meetingSession || !users || users.length === 0) return;
    const av = meetingSession.audioVideo;
    const handlePresenceChange = async (
      attendeeId: string,
      present: boolean,
    ) => {
      const selfId = meetingSession.configuration.credentials?.attendeeId;
      if (attendeeId === selfId && !present) return;
      if (present) exitedAttendeesRef.current.delete(attendeeId);
      else exitedAttendeesRef.current.add(attendeeId);
      try {
        const joinInfo = await discussionMeetingMutation.mutateAsync({
          spacePk,
          discussionPk,
        });

        setParticipants((prev) => {
          const incoming = new Set(
            joinInfo.participants.map((p: any) => p.user_pk),
          );
          return [
            ...prev.filter((p) => incoming.has(p.user_pk)),
            ...joinInfo.participants.filter(
              (p: any) => !prev.some((pp) => pp.user_pk === p.id),
            ),
          ];
        });
      } catch (err) {}
    };
    av.realtimeSubscribeToAttendeeIdPresence(handlePresenceChange);
    return () => {
      av.realtimeUnsubscribeToAttendeeIdPresence?.(handlePresenceChange);
    };
  }, [meetingSession, users, spacePk, discussionPk, get]);

  useEffect(() => {
    if (!meetingSession) return;
    const av = meetingSession.audioVideo;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const onRecordingMessage = (dataMessage: any) => {
      const text = new TextDecoder().decode(dataMessage.data);
      if (text === 'start') setIsRecording(true);
      else if (text === 'stop') setIsRecording(false);
    };
    av.realtimeSubscribeToReceiveDataMessage(
      'recording-status',
      onRecordingMessage,
    );
    return () =>
      av.realtimeUnsubscribeFromReceiveDataMessage('recording-status');
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

  const focusedUser = useMemo(
    () => users.find((u) => u.participant_id === focusedAttendeeId),
    [users, focusedAttendeeId],
  );
  const focusedParticipant = useMemo(
    () => participants.find((p) => p.user_pk === focusedUser?.user_pk),
    [participants, focusedUser],
  );
  const focusedNickname = useMemo(
    () =>
      focusedParticipant?.author_username ??
      focusedParticipant?.author_username ??
      focusedParticipant?.author_display_name,
    [focusedParticipant],
  );

  const token = [
    spacePk,
    discussionPk,
    isFirstClicked,
    isVideoOn,
    isSharing,
    isAudioOn,
    isRecording,
    videoTiles.length,
    Boolean(meetingSession),
    remoteContentTileOwner ?? '-',
    Object.keys(micStates).length,
    Object.keys(videoStates).length,
    messages.length,
    activePanel ?? '-',
    participants.length,
    focusedAttendeeId ?? '-',
    users.length,
  ].join('|');

  return {
    spacePk,
    discussionPk,
    refs: { tileMapRef },
    state: {
      isFirstClicked,
      isVideoOn,
      isSharing,
      isAudioOn,
      isRecording,
      videoTiles,
      meetingSession,
      remoteContentTileOwner,
      micStates,
      videoStates,
      messages,
      activePanel,
      participants,
      focusedAttendeeId,
    },
    setters: {
      setIsFirstClicked,
      setIsVideoOn,
      setIsSharing,
      setIsAudioOn,
      setIsRecording,
      setVideoTiles,
      setMeetingSession,
      setRemoteContentTileOwner,
      setMicStates,
      setVideoStates,
      setMessages,
      setActivePanel,
      setParticipants,
      setFocusedAttendeeId,
    },
    handlers: { sendMessage },
    data,
    discussion,
    users,
    derived: { focusedUser, focusedParticipant, focusedNickname },
    token,
  };
}
