import { logger } from '@/lib/logger';
import { SpaceDiscussionPathProps } from '../../space-discussion-path-props';
import { useSpaceDiscussionMeetingViewerController } from './use-space-discussion-meeting-viewer-controller';
import Bottom from '../../../components/meeting/bottom';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { SpaceType } from '@/features/spaces/types/space-type';
import { Header } from '../../../components/meeting/header';
import LocalVideo from '../../../components/meeting/local-video';
import RemoteContentShareVideo from '../../../components/meeting/remote-content-share-video';
import ChatPanel from '../../../components/meeting/chat-panel';
import ParticipantsPanel from '../../../components/meeting/participants-panel';
import RemoteGalleryView from '../../../components/meeting/remote-gallery-view';

export function SpaceDiscussionMeetingViewerPage({
  spacePk,
  discussionPk,
}: SpaceDiscussionPathProps) {
  logger.debug(
    `SpaceDiscussionMeetingViewerPage: spacePk=${spacePk} discussionPk=${discussionPk}`,
  );

  const ctrl = useSpaceDiscussionMeetingViewerController(spacePk, discussionPk);
  const navigate = useNavigate();

  return (
    <div className="fixed top-0 left-0 flex flex-row w-full h-full">
      <div className="flex flex-1 h-full justify-center items-center">
        <div className="flex flex-1 bg-black h-full w-full max-w-full flex-col justify-center items-center">
          <Header
            name={ctrl.discussion.discussion.name}
            onclose={async () => {
              await ctrl.cleanUpMeetingSession();
              navigate(route.spaceByType(SpaceType.Deliberation, spacePk));
            }}
          />

          <div className="relative w-full h-full">
            <div className="flex flex-col w-full justify-start items-start">
              <>
                {ctrl.meetingSession && (
                  <RemoteContentShareVideo
                    meetingSession={ctrl.meetingSession}
                    onRemoteContentTileUpdate={(tileState) => {
                      if (!tileState) {
                        ctrl.changeRemoteContentTileOwner(null);
                        return;
                      }
                      const attendeeId = tileState.boundAttendeeId;
                      if (
                        attendeeId &&
                        attendeeId !==
                          ctrl.meetingSession.configuration.credentials
                            ?.attendeeId
                      ) {
                        ctrl.changeRemoteContentTileOwner(attendeeId);
                      } else {
                        ctrl.changeRemoteContentTileOwner(null);
                      }
                    }}
                  />
                )}

                {ctrl.focusedAttendeeId && ctrl.meetingSession && (
                  <div className="w-full h-full z-100 bg-black border-4 border-white rounded-xl ">
                    <video
                      className="absolute top-0 left-0 w-full h-full bg-black object-cover z-50"
                      ref={(el) => {
                        if (el) {
                          const tile = ctrl.videoTiles.find(
                            (t) => t.attendeeId === ctrl.focusedAttendeeId,
                          );
                          if (tile) {
                            ctrl.meetingSession.audioVideo.bindVideoElement(
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
                      {ctrl.focusedNickname}
                    </div>
                  </div>
                )}

                {ctrl.meetingSession && (
                  <div
                    className={
                      ctrl.isSharing || ctrl.remoteContentTileOwner
                        ? 'absolute bottom-4 right-4 w-[180px] h-[130px] z-10'
                        : 'w-full h-full'
                    }
                  >
                    <LocalVideo
                      meetingSession={ctrl.meetingSession}
                      isVideoOn={ctrl.isVideoOn}
                    />
                  </div>
                )}
              </>
            </div>
          </div>

          <div className="flex flex-row w-full max-tablet:hidden">
            {ctrl.meetingSession &&
              !ctrl.isSharing &&
              !ctrl.remoteContentTileOwner && (
                <RemoteGalleryView
                  meetingSession={ctrl.meetingSession}
                  videoTiles={ctrl.videoTiles}
                  participants={ctrl.participants}
                  u={ctrl.users}
                  focusedAttendeeId={ctrl.focusedAttendeeId}
                  setFocusedAttendeeId={ctrl.changeFocusedAttendeeId}
                />
              )}
          </div>

          <Bottom
            isVideoOn={ctrl.isVideoOn}
            isAudioOn={ctrl.isAudioOn}
            isSharing={ctrl.isSharing}
            isRecording={ctrl.isRecording}
            onclose={async () => {
              await ctrl.cleanUpMeetingSession();
              navigate(route.spaceByType(SpaceType.Deliberation, spacePk));
            }}
            onRecordClick={async () => {
              if (!ctrl.meetingSession) return;
              ctrl.changeIsRecording(!ctrl.isRecording);
            }}
            onParticipantsClick={() => {
              ctrl.changeActivePanel(
                ctrl.activePanel === 'participants' ? null : 'participants',
              );
            }}
            onChatClick={() => {
              ctrl.changeActivePanel(
                ctrl.activePanel === 'chat' ? null : 'chat',
              );
            }}
            onVideoToggle={() => {
              ctrl.changeIsVideoOn(!ctrl.isVideoOn);
              ctrl.changeFocusedAttendeeId(null);
            }}
            onShareToggle={async () => {
              if (!ctrl.meetingSession) return;
              const av = ctrl.meetingSession.audioVideo;
              if (!ctrl.isSharing) {
                try {
                  await av.startContentShareFromScreenCapture();
                  ctrl.changeIsSharing(true);
                } catch (err) {
                  logger.error('Failed to share video with error: ', err);
                }
              } else {
                av.stopContentShare();
                ctrl.changeIsSharing(false);
              }
            }}
            onAudioToggle={async () => {
              if (!ctrl.meetingSession) return;
              const av = ctrl.meetingSession.audioVideo;

              if (ctrl.isAudioOn) {
                av.realtimeMuteLocalAudio();
                ctrl.changeIsAudioOn(false);
                return;
              }

              try {
                const dc = ctrl.meetingSession.deviceController;

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
                ctrl.changeIsAudioOn(true);
              } catch (err) {
                console.warn('[Audio] enable failed:', err);
              }
            }}
          />
        </div>
      </div>

      {ctrl.activePanel === 'participants' && (
        <ParticipantsPanel
          micStates={ctrl.micStates}
          videoStates={ctrl.videoStates}
          users={ctrl.users}
          participants={ctrl.participants}
          setFocusedAttendeeId={(attendeeId: string | null) => {
            ctrl.changeFocusedAttendeeId(attendeeId);
          }}
          meetingSession={ctrl.meetingSession!}
          onClose={() => ctrl.changeActivePanel(null)}
        />
      )}
      {ctrl.activePanel === 'chat' && (
        <ChatPanel
          onClose={() => ctrl.changeActivePanel(null)}
          messages={ctrl.messages}
          users={ctrl.users}
          participants={ctrl.participants}
          onSend={(text: string) => {
            ctrl.sendMessage(text);
          }}
          myAttendeeId={
            ctrl.meetingSession?.configuration.credentials?.attendeeId ?? ''
          }
        />
      )}
    </div>
  );
}
