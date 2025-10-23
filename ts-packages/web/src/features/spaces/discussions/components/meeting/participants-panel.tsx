import {
  Clear,
  Logo,
  ZoomMicOff,
  ZoomMicOn,
  ZoomVideoOff,
  ZoomVideoOn,
} from '@/components/icons';
import { useEffect, useMemo, useState } from 'react';
import { DefaultMeetingSession } from 'amazon-chime-sdk-js';
import { SpaceDiscussionParticipantResponse } from '../../types/space-discussion-participant-response';

export type ParticipantsPanelProps = {
  micStates: Record<string, boolean>;
  videoStates: Record<string, boolean>;
  users: SpaceDiscussionParticipantResponse[];
  participants: SpaceDiscussionParticipantResponse[];
  setFocusedAttendeeId: (attendeeId: string | null) => void;
  meetingSession: DefaultMeetingSession;
  onClose: () => void;
};

export default function ParticipantsPanel({
  micStates,
  videoStates,
  users,
  participants,
  onClose,
}: ParticipantsPanelProps) {
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const id = setTimeout(() => setVisible(true), 10);
    return () => clearTimeout(id);
  }, []);

  const userIdToAttendeeId = useMemo(() => {
    const map = new Map<string, string>();
    users.forEach((u) => map.set(u.user_pk, u.participant_id));
    return map;
  }, [users]);

  const handleClose = () => {
    setVisible(false);
    setTimeout(onClose, 300);
  };

  return (
    <aside
      role="dialog"
      aria-label="Participants panel"
      className={[
        'fixed inset-y-0 right-0',
        'w-[min(320px,100vw)] max-mobile:w-full',
        'bg-[#2d2d2d] text-white shadow-lg border-l border-neutral-800',
        'transition-transform duration-300',
        'z-[200]',
        visible ? 'translate-x-0' : 'translate-x-full',
        'overflow-y-auto',
      ].join(' ')}
    >
      <div className="flex justify-between items-center px-4 py-3 border-b border-neutral-600">
        <div className="flex flex-row w-fit gap-2.5 items-center">
          <Logo width={25} height={25} />
          <div className="font-semibold text-sm">Participants</div>
        </div>
        <button
          type="button"
          aria-label="Close participants panel"
          onClick={handleClose}
          className="cursor-pointer w-[24px] h-[24px] [&>path]:stroke-[#bfc8d9]"
        >
          <Clear fill="white" />
        </button>
      </div>

      <div className="flex flex-col flex-1 px-[10px] py-[20px] gap-[20px]">
        {participants.map((participant, index) => {
          const attendeeId = userIdToAttendeeId.get(participant.user_pk);
          const isMicOn = micStates[attendeeId ?? ''] ?? false;
          const isVideoOn = videoStates[attendeeId ?? ''] ?? false;

          return (
            <div
              key={index}
              className="flex flex-row w-full justify-between items-center"
            >
              <div className="flex flex-row w-fit items-center gap-1 min-w-0">
                {participant.author_profile_url ? (
                  <img
                    src={participant.author_profile_url}
                    alt={`${participant.author_username}'s profile`}
                    className="w-7.5 h-7.5 object-cover rounded-full flex-shrink-0"
                  />
                ) : (
                  <div className="w-7.5 h-7.5 bg-neutral-500 rounded-full flex-shrink-0" />
                )}
                <div className="font-medium text-white text-sm truncate">
                  {participant.author_username}
                </div>
              </div>

              <div className="flex flex-row w-fit gap-2 flex-shrink-0">
                {isMicOn ? (
                  <ZoomMicOn className="w-[18px] h-[18px] stroke-white" />
                ) : (
                  <ZoomMicOff className="w-[18px] h-[18px] stroke-white" />
                )}
                {isVideoOn ? (
                  <ZoomVideoOn className="w-[18px] h-[18px] stroke-white" />
                ) : (
                  <ZoomVideoOff className="w-[18px] h-[18px] stroke-white" />
                )}
              </div>
            </div>
          );
        })}
      </div>
    </aside>
  );
}
