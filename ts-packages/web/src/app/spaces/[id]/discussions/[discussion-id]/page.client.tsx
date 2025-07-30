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
    setIsVideoOn,
    isSharing,
    setIsSharing,
    isAudioOn,
    setIsAudioOn,
    isRecording,
    setIsRecording,
    videoTiles,
    meetingSession,
    remoteContentTileOwner,
    setRemoteContentTileOwner,
    micStates,
    videoStates,
    messages,
    activePanel,
    setActivePanel,
    participants,
    focusedAttendeeId,
    setFocusedAttendeeId,
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
              setRemoteContentTileOwner(null);
              router.replace(route.deliberationSpaceById(discussion.space_id));
            }}
          />

          {meetingSession && !isSharing && !remoteContentTileOwner && (
            <RemoteGalleryView
              meetingSession={meetingSession}
              videoTiles={videoTiles}
              participants={participants}
              u={users}
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
