'use client';

import { logger } from '@/lib/logger';
import { route } from '@/route';
import { useParams, useRouter } from 'next/navigation';
import React from 'react';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  endRecordingRequest,
  exitMeetingRequest,
  startRecordingRequest,
} from '@/lib/api/models/discussion';

import ParticipantsPanel from './_components/participants_panel';
import ChatPanel from './_components/chat_panel';
import Bottom from './_components/bottom';
import { Header } from './_components/header';
import LocalVideo from './_components/local_video';
import ContentShareVideo from './_components/content_share_video';
import RemoteContentShareVideo from './_components/remote_content_share_video';
import RemoteGalleryView from './_components/remote_gallery_view';
import ClientProviders, { useDiscussionContext } from './provider.client';

export default function DiscussionByIdPage() {
  const params = useParams();
  const spaceId = Number(params['id']);
  const discussionId = Number(params['discussion-id']);

  return (
    <ClientProviders spaceId={spaceId} discussionId={discussionId}>
      <Page />
    </ClientProviders>
  );
}

function Page() {
  const {
    spaceId,
    discussionId,
    isVideoOn,
    changeIsVideoOn,
    isSharing,
    changeIsSharing,
    isAudioOn,
    changeIsAudioOn,
    isRecording,
    changeIsRecording,
    videoTiles,
    meetingSession,
    remoteContentTileOwner,
    changeRemoteContentTileOwner,
    micStates,
    videoStates,
    messages,
    activePanel,
    changeActivePanel,
    participants,
    focusedAttendeeId,
    changeFocusedAttendeeId,
    discussion,
    users,
    sendMessage,
    focusedNickname,
  } = useDiscussionContext();

  const { post } = useApiCall();
  const router = useRouter();

  return (
    <div className="fixed top-0 left-0 flex flex-row w-full h-full">
      <div className="flex flex-1 h-full justify-center items-center">
        <div className="flex flex-1 bg-black h-full w-full max-w-full flex-col justify-center items-center">
          <Header
            name={discussion.name}
            onclose={async () => {
              await post(
                ratelApi.discussions.actDiscussionById(spaceId, discussionId),
                exitMeetingRequest(),
              );
              changeRemoteContentTileOwner(null);
              router.replace(route.deliberationSpaceById(discussion.space_id));
            }}
          />

          <div className="relative w-full h-full">
            <div className="flex flex-col w-full justify-start items-start">
              <>
                {meetingSession && (
                  <RemoteContentShareVideo
                    meetingSession={meetingSession}
                    onRemoteContentTileUpdate={(tileState) => {
                      if (!tileState) {
                        changeRemoteContentTileOwner(null);
                        return;
                      }
                      const attendeeId = tileState.boundAttendeeId;
                      if (
                        attendeeId &&
                        attendeeId !==
                          meetingSession.configuration.credentials?.attendeeId
                      ) {
                        changeRemoteContentTileOwner(attendeeId);
                      } else {
                        changeRemoteContentTileOwner(null);
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

          {meetingSession && !isSharing && !remoteContentTileOwner && (
            <RemoteGalleryView
              meetingSession={meetingSession}
              videoTiles={videoTiles}
              participants={participants}
              u={users}
              focusedAttendeeId={focusedAttendeeId}
              setFocusedAttendeeId={changeFocusedAttendeeId}
            />
          )}

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
              changeRemoteContentTileOwner(null);
              router.replace(route.deliberationSpaceById(discussion.space_id));
            }}
            onRecordClick={async () => {
              if (!meetingSession) return;
              const av = meetingSession.audioVideo;
              if (!isRecording) {
                await post(
                  ratelApi.discussions.actDiscussionById(spaceId, discussionId),
                  startRecordingRequest(),
                );
                av.realtimeSendDataMessage(
                  'recording-status',
                  new TextEncoder().encode('start'),
                  10000,
                );
              } else {
                await post(
                  ratelApi.discussions.actDiscussionById(spaceId, discussionId),
                  endRecordingRequest(),
                );
                av.realtimeSendDataMessage(
                  'recording-status',
                  new TextEncoder().encode('stop'),
                  10000,
                );
              }
              changeIsRecording(!isRecording);
            }}
            onParticipantsClick={() => {
              changeActivePanel(
                activePanel === 'participants' ? null : 'participants',
              );
            }}
            onChatClick={() => {
              changeActivePanel(activePanel === 'chat' ? null : 'chat');
            }}
            onVideoToggle={() => {
              changeIsVideoOn(!isVideoOn);
              changeFocusedAttendeeId(null);
            }}
            onShareToggle={async () => {
              if (!meetingSession) return;
              const av = meetingSession.audioVideo;
              if (!isSharing) {
                try {
                  await av.startContentShareFromScreenCapture();
                  changeIsSharing(true);
                } catch (err) {
                  logger.error('Failed to share video with error: ', err);
                }
              } else {
                av.stopContentShare();
                changeIsSharing(false);
              }
            }}
            onAudioToggle={async () => {
              if (!meetingSession) return;
              const av = meetingSession.audioVideo;

              if (isAudioOn) {
                av.realtimeMuteLocalAudio();
                changeIsAudioOn(false);
                return;
              }

              try {
                const dc = meetingSession.deviceController;

                let inputs = await dc.listAudioInputDevices();

                if (!inputs.length) {
                  try {
                    const stream = await navigator.mediaDevices.getUserMedia({
                      audio: true,
                    });
                    stream.getTracks().forEach((t) => t.stop());
                    inputs = await dc.listAudioInputDevices();
                  } catch (permErr) {
                    console.warn(
                      '[Audio] permission denied or no device:',
                      permErr,
                    );
                    return;
                  }
                }

                if (!inputs.length) {
                  console.warn(
                    '[Audio] no input devices found after permission',
                  );
                  return;
                }

                await dc.startAudioInput(inputs[0].deviceId);
                av.realtimeUnmuteLocalAudio();
                changeIsAudioOn(true);
              } catch (err) {
                console.warn('[Audio] enable failed:', err);
              }
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
            changeFocusedAttendeeId(attendeeId);
          }}
          meetingSession={meetingSession!}
          onClose={() => changeActivePanel(null)}
        />
      )}
      {activePanel === 'chat' && (
        <ChatPanel
          onClose={() => changeActivePanel(null)}
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
