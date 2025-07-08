'use client';

import { useDiscussionById } from '@/app/(social)/_hooks/use-discussion';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import { useParams, useRouter } from 'next/navigation';
import React, { useEffect, useRef, useState } from 'react';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  endRecordingRequest,
  exitMeetingRequest,
  participantMeetingRequest,
  startMeetingRequest,
  startRecordingRequest,
} from '@/lib/api/models/discussion';

import {
  DefaultDeviceController,
  DefaultMeetingSession,
  ConsoleLogger,
  LogLevel,
  MeetingSessionConfiguration,
} from 'amazon-chime-sdk-js';
import { Participant } from '@/lib/api/models/meeting';
import ParticipantsPanel from './_components/participants_panel';
import ChatPanel from './_components/chat_panel';
import Bottom from './_components/bottom';
import { Header } from './_components/header';
import LocalVideo from './_components/local_video';
import ContentShareVideo from './_components/content_share_video';
import RemoteContentShareVideo from './_components/remote_content_share_video';
import RemoteGalleryView from './_components/remote_gallery_view';

export default function DiscussionByIdPage() {
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
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [focusedAttendeeId, setFocusedAttendeeId] = useState<string | null>(
    null,
  );

  const { post, get } = useApiCall();
  const router = useRouter();
  const params = useParams();
  const spaceId = Number(params['id']);
  const discussionId = Number(params['discussion-id']);

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
    <div className="fixed top-0 left-0 flex flex-row w-full h-full">
      <div className="flex flex-1 h-full justify-center items-center">
        <div className="flex flex-1 bg-black h-full w-full max-w-[1300px] flex-col justify-center items-center">
          <Header
            name={discussion.name}
            onclose={async () => {
              await post(
                ratelApi.discussions.actDiscussionById(spaceId, discussionId),
                exitMeetingRequest(),
              );
              setRemoteContentTileOwner(null);
              router.replace(route.deliberationSpaceById(discussion.space_id));
            }}
          />

          {meetingSession && (
            <RemoteGalleryView
              meetingSession={meetingSession}
              videoTiles={videoTiles}
              participants={participants}
              users={users}
              focusedAttendeeId={focusedAttendeeId}
              setFocusedAttendeeId={setFocusedAttendeeId}
            />
          )}

          <div className="relative w-full h-full">
            <div className="flex flex-col w-full justify-start items-start">
              <>
                {meetingSession && (
                  <RemoteContentShareVideo
                    meetingSession={meetingSession}
                    onRemoteContentTileUpdate={(tileState) => {
                      if (!tileState) {
                        setRemoteContentTileOwner(null);
                        return;
                      }

                      const attendeeId = tileState.boundAttendeeId;
                      if (
                        attendeeId &&
                        attendeeId !==
                          meetingSession.configuration.credentials?.attendeeId
                      ) {
                        setRemoteContentTileOwner(attendeeId);
                      } else {
                        setRemoteContentTileOwner(null);
                      }
                    }}
                  />
                )}

                {meetingSession && isSharing && (
                  <ContentShareVideo meetingSession={meetingSession} />
                )}

                {focusedAttendeeId && meetingSession && (
                  <div className="w-full h-full z-100 bg-black border-4 border-white rounded-xl ">
                    <video
                      className="absolute top-0 left-0 w-full h-full bg-black object-cover z-50"
                      ref={(el) => {
                        if (el) {
                          const tile = videoTiles.find(
                            (t) => t.attendeeId === focusedAttendeeId,
                          );
                          if (tile) {
                            meetingSession.audioVideo.bindVideoElement(
                              tile.tileId,
                              el,
                            );
                          }
                        }
                      }}
                      autoPlay
                      muted={false}
                    />

                    <div className="absolute bottom-2 right-2 z-50 w-fit max-w-[100px] h-fit px-[10px] py-[5px] bg-neutral-800 text-white text-sm rounded-lg overflow-hidden text-ellipsis whitespace-nowrap">
                      {focusedNickname}
                    </div>
                  </div>
                )}

                {meetingSession && (
                  <div
                    className={
                      isSharing || remoteContentTileOwner
                        ? 'absolute bottom-4 right-4 w-[180px] h-[130px] z-10'
                        : 'w-full h-full'
                    }
                  >
                    <LocalVideo
                      meetingSession={meetingSession}
                      isVideoOn={isVideoOn}
                    />
                  </div>
                )}
              </>
            </div>
          </div>

          <Bottom
            isVideoOn={isVideoOn}
            isAudioOn={isAudioOn}
            isSharing={isSharing}
            isRecording={isRecording}
            onclose={async () => {
              await post(
                ratelApi.discussions.actDiscussionById(spaceId, discussionId),
                exitMeetingRequest(),
              );
              setRemoteContentTileOwner(null);
              router.replace(route.deliberationSpaceById(discussion.space_id));
            }}
            onRecordClick={async () => {
              if (!isRecording) {
                await post(
                  ratelApi.discussions.actDiscussionById(spaceId, discussionId),
                  startRecordingRequest(),
                );
              } else {
                await post(
                  ratelApi.discussions.actDiscussionById(spaceId, discussionId),
                  endRecordingRequest(),
                );
              }
              setIsRecording(!isRecording);
            }}
            onParticipantsClick={() => {
              setActivePanel((prev) =>
                prev === 'participants' ? null : 'participants',
              );
            }}
            onChatClick={() => {
              setActivePanel((prev) => (prev === 'chat' ? null : 'chat'));
            }}
            onVideoToggle={() => {
              setIsVideoOn((prev) => !prev);
              setFocusedAttendeeId(null);
            }}
            onShareToggle={async () => {
              if (!meetingSession) return;

              const av = meetingSession.audioVideo;

              if (!isSharing) {
                try {
                  await av.startContentShareFromScreenCapture();

                  setIsSharing(true);
                } catch (err) {
                  logger.error('Failed to share video with error: ', err);
                }
              } else {
                av.stopContentShare();
                setIsSharing(false);
              }
            }}
            onAudioToggle={() => {
              if (!meetingSession) return;

              const av = meetingSession.audioVideo;

              if (isAudioOn) {
                av.realtimeMuteLocalAudio();
              } else {
                av.realtimeUnmuteLocalAudio();
              }

              setIsAudioOn((prev) => !prev);
            }}
          />
        </div>
      </div>

      {activePanel === 'participants' && (
        <ParticipantsPanel
          micStates={micStates}
          videoStates={videoStates}
          users={users}
          participants={participants}
          setFocusedAttendeeId={(attendeeId: string | null) => {
            setFocusedAttendeeId(attendeeId);
          }}
          meetingSession={meetingSession!}
          onClose={() => setActivePanel(null)}
        />
      )}
      {activePanel === 'chat' && (
        <ChatPanel
          onClose={() => setActivePanel(null)}
          messages={messages}
          users={users}
          participants={participants}
          onSend={(text: string) => {
            sendMessage(text);
          }}
          myAttendeeId={
            meetingSession?.configuration.credentials?.attendeeId ?? ''
          }
        />
      )}
    </div>
  );
}
