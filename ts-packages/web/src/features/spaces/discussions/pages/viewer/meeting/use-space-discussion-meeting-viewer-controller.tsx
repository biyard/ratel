import { useRef, useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router';
import {
  ConsoleLogger,
  DefaultDeviceController,
  DefaultMeetingSession,
  LogLevel,
  MeetingSessionConfiguration,
} from 'amazon-chime-sdk-js';

import { logger } from '@/lib/logger';
import { route } from '@/route';
import { useExitMeetingMutation } from '../../../hooks/use-exit-meeting-mutation';
import { useDiscussionMeetingMutation } from '../../../hooks/use-discussion-meeting-mutation';
import useDiscussionParticipantSpace from '../../../hooks/use-discussion-participant-space';
import useDiscussion from '../../../hooks/use-discussion';
import { DiscussionResponse } from '../../../types/get-discussion-response';
import { SpaceDiscussionParticipantResponse } from '../../../types/space-discussion-participant-response';

type ControllerDeps = ReturnType<typeof useBuildDeps>;

export class SpaceDiscussionMeetingViewerController {
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
  changeParticipants = (v: SpaceDiscussionParticipantResponse[]) =>
    this.deps.setters.setParticipants(v);

  get focusedAttendeeId() {
    return this.deps.state.focusedAttendeeId;
  }
  changeFocusedAttendeeId = (v: string | null) =>
    this.deps.setters.setFocusedAttendeeId(v);

  get discussion(): DiscussionResponse {
    return this.deps.discussion;
  }
  get users(): SpaceDiscussionParticipantResponse[] {
    return this.deps.users;
  }

  sendMessage = (text: string) => this.deps.handlers.sendMessage(text);

  cleanUpMeetingSession = () => this.deps.handlers.cleanupMeetingSession();

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

export function useSpaceDiscussionMeetingViewerController(
  spacePk: string,
  discussionPk: string,
) {
  const deps = useBuildDeps(spacePk, discussionPk);
  return useMemo(
    () => new SpaceDiscussionMeetingViewerController(deps),
    [deps.token],
  );
}

function useBuildDeps(spacePk: string, discussionPk: string) {
  const discussionMeetingMutation = useDiscussionMeetingMutation();
  const exitMeetingMutation = useExitMeetingMutation();
  const data = useDiscussionParticipantSpace(spacePk, discussionPk);
  const { refetch: refetchParticipants } = data;

  const discussionParticipant = data.data;

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
  const [participants, setParticipants] = useState<
    SpaceDiscussionParticipantResponse[]
  >([]);
  const [focusedAttendeeId, setFocusedAttendeeId] = useState<string | null>(
    null,
  );

  const navigate = useNavigate();

  const { data: discussion } = useDiscussion(spacePk, discussionPk);

  const users = discussionParticipant.participants;

  const startedRef = useRef(false);

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
          logger.debug('cleanup meeting error: ', err);
        }
        dc.destroy?.();
      }
      try {
        await exitMeetingMutation.mutateAsync({
          spacePk,
          discussionPk,
        });
      } catch (err) {
        logger.debug('exit meeting error: ', err);
      }
      setRemoteContentTileOwner(null);
    };

    const handlePopState = async () => {
      await cleanupMeetingSession();
      data.refetch();
      navigate(route.discussionByPk(spacePk, discussionPk));
    };

    const handleBeforeUnload = async (e: BeforeUnloadEvent) => {
      e.preventDefault();
      await cleanupMeetingSession();
      data.refetch();
    };

    const handleUnload = async () => {
      await cleanupMeetingSession();
      data.refetch();
    };

    window.addEventListener('popstate', handlePopState);
    window.addEventListener('beforeunload', handleBeforeUnload);
    window.addEventListener('unload', handleUnload);
    return () => {
      window.removeEventListener('popstate', handlePopState);
      window.removeEventListener('beforeunload', handleBeforeUnload);
      window.removeEventListener('unload', handleUnload);
    };
  }, [spacePk, discussionPk, meetingSession, navigate]);

  useEffect(() => {
    if (startedRef.current) return;
    startedRef.current = true;

    async function startChime() {
      const joinInfo = await discussionMeetingMutation.mutateAsync({
        spacePk,
        discussionPk,
      });

      setParticipants(discussionParticipant.participants);
      const chimeLogger = new ConsoleLogger('ChimeLogs', LogLevel.INFO);
      const deviceController = new DefaultDeviceController(chimeLogger);

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const raw = joinInfo as any;

      const meeting =
        raw?.meeting ??
        raw?.Meeting ??
        raw?.data?.meeting ??
        raw?.data?.Meeting;

      const attendee =
        raw?.attendee ??
        raw?.Attendee ??
        raw?.data?.attendee ??
        raw?.data?.Attendee;

      const participants =
        raw?.participants ??
        raw?.Participants ??
        raw?.data?.participants ??
        raw?.data?.Participants;

      if (!meeting || !attendee) {
        console.error('[startChime] invalid joinInfo shape:', { raw });
        throw new Error('Invalid joinInfo: meeting/attendee missing');
      }

      if (participants) setParticipants(participants);
      const meetingForChime = {
        Meeting: {
          MeetingId: meeting.meeting_id ?? meeting.MeetingId,
          MediaRegion: meeting.media_region ?? meeting.MediaRegion,
          MediaPlacement: {
            AudioHostUrl:
              meeting.media_placement?.audio_host_url ??
              meeting.MediaPlacement?.AudioHostUrl,
            AudioFallbackUrl:
              meeting.media_placement?.audio_fallback_url ??
              meeting.MediaPlacement?.AudioFallbackUrl,
            ScreenDataUrl:
              meeting.media_placement?.screen_data_url ??
              meeting.MediaPlacement?.ScreenDataUrl,
            ScreenSharingUrl:
              meeting.media_placement?.screen_sharing_url ??
              meeting.MediaPlacement?.ScreenSharingUrl,
            ScreenViewingUrl:
              meeting.media_placement?.screen_viewing_url ??
              meeting.MediaPlacement?.ScreenViewingUrl,
            SignalingUrl:
              meeting.media_placement?.signaling_url ??
              meeting.MediaPlacement?.SignalingUrl,
            TurnControlUrl:
              meeting.media_placement?.turn_control_url ??
              meeting.MediaPlacement?.TurnControlUrl,
          },
        },
      };

      const attendeeForChime = {
        Attendee: {
          AttendeeId: attendee.attendee_id ?? attendee.AttendeeId,
          ExternalUserId: attendee.external_user_id ?? attendee.ExternalUserId,
          JoinToken: attendee.join_token ?? attendee.JoinToken,
        },
      };

      const configuration = new MeetingSessionConfiguration(
        meetingForChime,
        attendeeForChime,
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
  }, [spacePk, discussionPk]);

  useEffect(() => {
    if (!meetingSession) return;
    const av = meetingSession.audioVideo;
    av.start();
    const observer = {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      videoTileDidUpdate: (tileState: any) => {
        const { tileId, boundAttendeeId } = tileState;
        if (!tileId || !boundAttendeeId || tileState.isContent) return;
        tileMapRef.current[tileId] = boundAttendeeId;
        const videoOn = !!(tileState.active || tileState.boundVideoStream);
        setVideoTiles((prev) => {
          const exists = prev.some((t) => t.tileId === tileId);
          return exists
            ? prev
            : [...prev, { tileId, attendeeId: boundAttendeeId }];
        });
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
      if (attendeeId.includes('#content')) return;
      if (present) {
        activeAttendeeIds.add(attendeeId);
        void refetchParticipants();
        av.realtimeSubscribeToVolumeIndicator(attendeeId, (_id, _v, muted) => {
          if (typeof muted === 'boolean') {
            setMicStates((prev) => ({ ...prev, [attendeeId]: !muted }));
          }
        });
      } else {
        activeAttendeeIds.delete(attendeeId);
        void refetchParticipants();
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
  }, [meetingSession, refetchParticipants]);

  const exitedAttendeesRef = useRef<Set<string>>(new Set());

  useEffect(() => {
    if (!meetingSession || !users || users.length === 0) return;
    const av = meetingSession.audioVideo;

    const byAttendee = new Map((users ?? []).map((u) => [u.participant_id, u]));
    const byUserPk = new Map(
      (discussionParticipant?.participants ?? []).map((p) => [p.user_pk, p]),
    );

    const handlePresenceChange = (
      attendeeId: string,
      present: boolean,
      _externalUserId?: string,
    ) => {
      if (attendeeId.includes('#content')) return;
      const selfId = meetingSession.configuration.credentials?.attendeeId;
      if (attendeeId === selfId && !present) return;

      const u = byAttendee.get(attendeeId);
      const userPk = u?.user_pk;

      if (!userPk) {
        void refetchParticipants();
        return;
      }

      if (present) exitedAttendeesRef.current.delete(attendeeId);
      else exitedAttendeesRef.current.add(attendeeId);

      setParticipants((prev) => {
        const exists = prev.some((p) => p.user_pk === userPk);
        if (present) {
          if (exists) return prev;
          const full =
            byUserPk.get(userPk) ??
            ({
              user_pk: u.user_pk,
              author_username: u.author_username,
              author_display_name: u.author_display_name,
              author_profile_url: u.author_profile_url,
            } as SpaceDiscussionParticipantResponse);
          return [...prev, full];
        } else {
          if (!exists) return prev;
          return prev.filter((p) => p.user_pk !== userPk);
        }
      });
    };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    av.realtimeSubscribeToAttendeeIdPresence(handlePresenceChange as any);
    return () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      av.realtimeUnsubscribeToAttendeeIdPresence?.(handlePresenceChange as any);
    };
  }, [
    meetingSession,
    users,
    discussionParticipant?.participants,
    refetchParticipants,
  ]);

  useEffect(() => {
    setParticipants(discussionParticipant?.participants ?? []);
  }, [discussionParticipant?.participants]);

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
        logger.debug('cleanup meeting session failed: ', err);
      }
      dc.destroy?.();
    }
    try {
      await exitMeetingMutation.mutateAsync({
        spacePk,
        discussionPk,
      });
    } catch (err) {
      logger.debug('exit meeting failed: ', err);
    }
    setRemoteContentTileOwner(null);
  };

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
    handlers: { sendMessage, cleanupMeetingSession },
    discussion,
    users,
    derived: { focusedUser, focusedParticipant, focusedNickname },
    token,
  };
}
