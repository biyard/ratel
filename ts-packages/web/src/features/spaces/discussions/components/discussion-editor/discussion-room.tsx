import { useEffect, useRef, useState } from 'react';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';
import { format } from 'date-fns';
import discussionImg from '@/assets/images/discussion.png';
import { ArrowRight, Extra2 } from '@/components/icons';

export type DiscussionRoomProps = {
  discussion: SpaceDiscussionResponse;
  canEdit: boolean;
  isMember: boolean;
  isPublished: boolean;
  onclick: () => void;
  onupdate: () => void;
  ondelete: () => void;
};

export function DiscussionRoom({
  discussion,
  canEdit,
  isMember,
  isPublished,
  onclick,
  onupdate,
  ondelete,
}: DiscussionRoomProps) {
  const now = Math.floor(Date.now());
  const isLive = now >= discussion.started_at && now <= discussion.ended_at;
  const isUpcoming = now < discussion.started_at;
  const isFinished = now > discussion.ended_at;

  const formattedDate = `${format(
    new Date(discussion.started_at),
    'dd MMM, yyyy HH:mm',
  )} - ${format(new Date(discussion.ended_at), 'dd MMM, yyyy HH:mm')}`;

  const statusLabel = isUpcoming
    ? 'Upcoming Discussion'
    : isFinished
      ? 'Finished Discussion'
      : 'Ongoing Discussion';

  const [menuOpen, setMenuOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!menuOpen) return;
    const onOutside = (e: MouseEvent) => {
      if (!menuRef.current) return;
      if (!menuRef.current.contains(e.target as Node)) {
        setMenuOpen(false);
      }
    };
    const onEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setMenuOpen(false);
    };
    document.addEventListener('mousedown', onOutside);
    document.addEventListener('keydown', onEsc);
    return () => {
      document.removeEventListener('mousedown', onOutside);
      document.removeEventListener('keydown', onEsc);
    };
  }, [menuOpen]);

  return (
    <div className="flex flex-row w-full items-start justify-between max-tablet:flex-col gap-5">
      <div className="relative w-[240px] h-[150px] rounded-lg overflow-hidden max-tablet:w-[350px] max-mobile:w-full max-tablet:aspect-[16/9] max-tablet:h-auto">
        <img
          src={discussionImg}
          alt="Discussion Thumbnail"
          className="object-cover"
        />
        {isLive && (
          <div className="absolute top-[12px] left-[12px] bg-[rgba(255,0,0,0.5)] rounded-sm font-semibold text-sm text-white p-1">
            LIVE
          </div>
        )}
      </div>

      <div className="flex flex-col flex-1 h-full justify-between items-start w-full">
        <div className="flex flex-col flex-1 gap-1 w-full">
          <div className="flex w-full items-start justify-between">
            <div className="text-sm text-neutral-400 light:text-[#737373] font-normal">
              {statusLabel}
            </div>

            {canEdit && (
              <div className="relative w-fit h-fit" ref={menuRef}>
                <Extra2
                  className="cursor-pointer w-6 h-6"
                  onClick={(e) => {
                    e.stopPropagation();
                    setMenuOpen((v) => !v);
                  }}
                />
                {menuOpen && (
                  <div
                    className="absolute right-0 mt-2 w-25 bg-white text-black rounded shadow-lg text-sm z-50 overflow-hidden"
                    onClick={(e) => e.stopPropagation()}
                  >
                    <div
                      className="px-4 py-2 hover:bg-neutral-200 cursor-pointer whitespace-nowrap"
                      onClick={() => {
                        setMenuOpen(false);
                        onupdate();
                      }}
                    >
                      {'Update'}
                    </div>
                    <div
                      className="px-4 py-2 hover:bg-neutral-200 cursor-pointer whitespace-nowrap"
                      onClick={() => {
                        setMenuOpen(false);
                        ondelete();
                      }}
                    >
                      {'Delete'}
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>

          <div className="text-lg text-text-primary font-bold">
            {discussion.name}
          </div>
          <div className="text-sm text-[#6d6d6d] light:text-[#737373] font-normal">
            {formattedDate}
          </div>
          <div
            className="text-sm text-neutral-400 light:text-[#737373] font-normal overflow-hidden text-ellipsis"
            style={{
              display: '-webkit-box',
              WebkitLineClamp: 2,
              WebkitBoxOrient: 'vertical',
            }}
          >
            {discussion.description}
          </div>
        </div>

        {isLive && isMember && isPublished && (
          <div className="flex flex-row w-full justify-end">
            <JoinButton onClick={onclick} />
          </div>
        )}
      </div>
    </div>
  );
}

function JoinButton({ onClick }: { onClick: () => void }) {
  return (
    <div
      className="cursor-pointer flex flex-row items-center w-fit h-fit px-5 py-2.5 gap-2.5 bg-white light:bg-card-bg border border-card-border hover:bg-white/80 light:hover:bg-card-bg/50 rounded-lg"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-sm">Join</div>
      <ArrowRight className=" [&>path]:stroke-black stroke-3 w-[15px] h-[15px]" />
    </div>
  );
}
