'use client';

import { getTimeWithFormat } from '@/lib/time-utils';
import React, { useContext } from 'react';
import Clock from '@/assets/icons/clock.svg';
import { PieChart1, Vote } from '@/components/icons';
// import { Settings } from 'lucide-react';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { TeamContext } from '@/lib/contexts/team-context';
// import { usePopup } from '@/lib/contexts/popup-service';
// import SetSchedulePopup from '../../_components/modal/set-schedule';
import { useTranslations } from 'next-intl';
import useSpaceById from '@/hooks/use-space-by-id';
// import { useEditCoordinatorStore } from '../../space-store';
import { Tab, usePollStore } from '../store';
import BorderSpaceCard from '@/app/(social)/_components/border-space-card';
import { usePermission } from '@/app/(social)/_hooks/use-permission';
import { GroupPermission } from '@/lib/api/models/group';

export default function SpaceSideMenu({ spaceId }: { spaceId: number }) {
  const t = useTranslations('PollSpace');
  // const popup = usePopup();
  const { data: space } = useSpaceById(spaceId);
  const { status } = space;
  // const { isEdit } = useEditCoordinatorStore();
  const started_at = space?.started_at || 0;
  const ended_at = space?.ended_at || 0;
  const { activeTab, changeTab } = usePollStore();
  const { teams } = useContext(TeamContext);
  const authorId = space?.author[0].id;

  const selectedTeam = teams.some((t) => t.id === authorId);

  const { data: userInfo } = useUserInfo();
  const userId = userInfo ? userInfo.id : 0;
  const createdAt = space.created_at;

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <BorderSpaceCard>
        <div className="flex flex-col gap-2.5 w-full">
          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${activeTab == Tab.Poll ? 'bg-neutral-800 light:bg-[#f5f5f5]' : ''}`}
            onClick={() => {
              changeTab(Tab.Poll);
            }}
          >
            <Vote className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-text-primary text-sm">
              {t('poll')}
            </div>
          </div>

          {(space.author.some((a) => a.id === userId) || selectedTeam) &&
            status == SpaceStatus.InProgress && (
              <div
                className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${
                  activeTab == Tab.Analyze
                    ? 'bg-neutral-800 light:bg-[#f5f5f5]'
                    : ''
                }`}
                onClick={() => {
                  changeTab(Tab.Analyze);
                }}
              >
                <PieChart1 className="[&>path]:stroke-neutral-80 w-5 h-5" />
                <div className="font-bold text-text-primary text-sm">
                  {t('analyze')}
                </div>
              </div>
            )}
        </div>
      </BorderSpaceCard>
      <BorderSpaceCard>
        <div className="w-full text-sm text-white">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 light:text-neutral-500 font-semibold text-[14px]">
              <Clock width={20} height={20} />
              {t('timeline')}
            </div>
            {/* {isEdit ? (
              <div
                className="cursor-pointer w-fit h-fit"
                onClick={() => {
                  popup
                    .open(
                      <SetSchedulePopup
                        startedAt={started_at}
                        endedAt={ended_at}
                        onconfirm={(startDate: number, endDate: number) => {
                          console.log('startDate, endDate', startDate, endDate);
                          // handleUpdateStartDate(Math.floor(startDate / 1000));
                          // handleUpdateEndDate(Math.floor(endDate / 1000));
                          popup.close();
                        }}
                      />,
                    )
                    .overflow(true);
                }}
              >
                <Settings
                  width={20}
                  height={20}
                  className="text-neutral-500 w-5 h-5"
                />
              </div>
            ) : (
              <></>
            )} */}
          </div>

          <div className="flex flex-col pl-3.25 gap-5">
            {[
              { label: t('created'), date: createdAt },
              { label: t('poll_open'), date: started_at },
              { label: t('poll_close'), date: ended_at },
            ]
              .filter(
                (item): item is { label: string; date: number } =>
                  item !== null,
              )
              .map((item) => (
                <div className="flex flex-col gap-1" key={item.label}>
                  <div className="font-medium text-text-primary text-[15px]/[12px]">
                    {item.label}
                  </div>
                  <div className="font-medium text-neutral-80 text-xs/[12px]">
                    {getTimeWithFormat(item.date ?? 0)}
                  </div>
                </div>
              ))}
          </div>
        </div>
      </BorderSpaceCard>
    </div>
  );
}

export function SpaceTabsMobile({ spaceId }: { spaceId: number }) {
  const t = useTranslations('PollSpace');

  const { data: space } = useSpaceById(spaceId);

  const activeTab = usePollStore((s) => s.activeTab);
  const setActiveTab = usePollStore((s) => s.changeTab);

  const { data: userInfo } = useUserInfo();
  const userId = userInfo?.id ?? 0;
  const { teams } = React.useContext(TeamContext);

  const authorId = space?.author?.[0]?.id;
  const selectedTeam = !!authorId && teams.some((t) => t.id === authorId);

  const writePostPermission =
    usePermission(authorId ?? 0, GroupPermission.WritePosts).data
      ?.has_permission ?? false;

  const showAnalyze =
    !!space &&
    (space.author.some((a) => a.id === userId) || selectedTeam) &&
    space.status !== SpaceStatus.Draft &&
    writePostPermission;

  const wrapRef = React.useRef<HTMLDivElement | null>(null);
  const pos = React.useRef({ isDown: false, startX: 0, scrollLeft: 0 });

  const onPointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el) return;

    const target = e.target as Element;
    if (target.closest('button')) return;

    el.setPointerCapture?.(e.pointerId);
    pos.current.isDown = true;
    pos.current.startX = e.clientX;
    pos.current.scrollLeft = el.scrollLeft;
  };

  const onPointerMove = (e: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el || !pos.current.isDown) return;
    const dx = e.clientX - pos.current.startX;
    el.scrollLeft = pos.current.scrollLeft - dx;
  };

  const endDrag = (e?: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el) return;
    pos.current.isDown = false;
    if (e) el.releasePointerCapture?.(e.pointerId);
  };

  const TabBtn = ({
    label,
    active,
    onClick,
  }: {
    label: string;
    active: boolean;
    onClick: () => void;
  }) => (
    <button
      onClick={onClick}
      className={[
        'shrink-0 px-3 py-2 rounded-[50px] text-sm font-bold transition-colors',
        active
          ? 'bg-neutral-500 light:bg-white text-text-primary border border-neutral-500 light:border-white'
          : 'bg-transparent border border-white/70 light:border-neutral-500 text-text-primary/80 hover:bg-white/5',
      ].join(' ')}
    >
      {label}
    </button>
  );

  if (!space) return null;

  return (
    <div className="max-tablet:flex hidden w-full">
      <div
        ref={wrapRef}
        className="w-full overflow-x-auto no-scrollbar momentum cursor-grab select-none"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={endDrag}
        onPointerCancel={endDrag}
        onPointerLeave={endDrag}
      >
        <div className="flex items-center gap-2 w-max pr-3">
          <TabBtn
            label={t('poll')}
            active={activeTab === Tab.Poll}
            onClick={() => setActiveTab(Tab.Poll)}
          />
          {showAnalyze && (
            <TabBtn
              label={t('analyze')}
              active={activeTab === Tab.Analyze}
              onClick={() => setActiveTab(Tab.Analyze)}
            />
          )}
        </div>
      </div>
    </div>
  );
}

// function EditSplitButton({
//   isEdit,
//   status,
//   postingSpace,
//   onedit,
//   onsave,
// }: {
//   isEdit: boolean;
//   status: SpaceStatus;
//   postingSpace: () => void;
//   onedit: () => void;
//   onsave: () => void;
// }) {
//   const [showPopup, setShowPopup] = useState(false);
//   const popupRef = useRef<HTMLDivElement>(null);

//   useEffect(() => {
//     function handleClickOutside(event: MouseEvent) {
//       if (
//         popupRef.current &&
//         !popupRef.current.contains(event.target as Node)
//       ) {
//         setShowPopup(false);
//       }
//     }

//     document.addEventListener('mousedown', handleClickOutside);
//     return () => {
//       document.removeEventListener('mousedown', handleClickOutside);
//     };
//   }, []);

//   return (
//     <div className="relative flex items-center w-full h-[46px] gap-2">
//       {/* Left "Edit" Button */}
//       {
//         <button
//           className={`flex items-center justify-start flex-row w-full bg-white hover:bg-neutral-300 text-black px-4 py-3 gap-1 ${status === SpaceStatus.Draft ? 'rounded-l-[100px] rounded-r-[4px]' : 'rounded-l-[100px] rounded-r-[100px] w-full'}'}`}
//           onClick={() => {
//             if (isEdit) {
//               onsave();
//             } else {
//               onedit();
//             }
//           }}
//         >
//           <Edit1 className="w-[18px] h-[18px]" />
//           <span className="font-bold text-neutral-900 text-base/[22px]">
//             {isEdit ? 'Save' : 'Edit'}
//           </span>
//         </button>
//       }

//       {/* Right Dropdown Toggle */}
//       {status != SpaceStatus.InProgress ? (
//         <div className="relative h-full" ref={popupRef}>
//           <button
//             className="w-[48px] h-full flex items-center justify-center bg-neutral-500 rounded-r-[100px] rounded-l-[4px]"
//             onClick={() => setShowPopup((prev) => !prev)}
//           >
//             <BottomTriangle />
//           </button>

//           {/* Pop-up Menu */}
//           {showPopup && (
//             <div
//               className="absolute top-full right-0 mt-2 px-4 py-2 min-w-[150px] bg-white hover:bg-neutral-300 text-black rounded shadow-lg text-sm cursor-pointer whitespace-nowrap z-50"
//               onClick={() => {
//                 postingSpace();
//                 setShowPopup(false);
//               }}
//             >
//               Posting
//             </div>
//           )}
//         </div>
//       ) : (
//         <></>
//       )}
//     </div>
//   );
// }
