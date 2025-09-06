'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import { getTimeWithFormat } from '@/lib/time-utils';
import React, { useContext } from 'react';
import Clock from '@/assets/icons/clock.svg';
import { Discuss, PieChart1, File, Vote } from '@/components/icons';
import { CheckCircle, Settings } from 'lucide-react';
import { DeliberationTab } from '../types';
import { SpaceStatus } from '@/lib/api/models/spaces';
import {
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from '../provider.client';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { TeamContext } from '@/lib/contexts/team-context';
import { usePopup } from '@/lib/contexts/popup-service';
import SetSchedulePopup from '../../_components/modal/set-schedule';
import { useTranslations } from 'next-intl';

export default function SpaceSideMenu() {
  const t = useTranslations('DeliberationSpace');
  const popup = usePopup();
  const {
    isEdit,
    deliberation,
    selectedType,
    handleUpdateSelectedType,
    startedAt,
    endedAt,
    status,
    handleUpdateEndDate,
    handleUpdateStartDate,
  } = useDeliberationSpaceContext();
  const space = useDeliberationSpace();
  const { teams } = useContext(TeamContext);
  const authorId = space?.author[0].id;
  const discussions = deliberation.discussions;

  const deliberationEndedAt =
    discussions.length !== 0
      ? discussions
          .map((t) => t.ended_at)
          .reduce((latest, current) => (current > latest ? current : latest))
      : 0;

  const selectedTeam = teams.some((t) => t.id === authorId);

  const { data: userInfo } = useUserInfo();
  const userId = userInfo ? userInfo.id : 0;
  const createdAt = space.created_at;

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <BlackBox>
        <div className="flex flex-col gap-2.5 w-full">
          <div
            className={`cursor-pointer flex flex-row w-full gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.SUMMARY ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.SUMMARY);
            }}
          >
            <File className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-white text-sm">{t('summary')}</div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.DELIBERATION ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.DELIBERATION);
            }}
          >
            <Discuss className="w-5 h-5" />
            <div className="font-bold text-white text-sm">
              {t('deliberation')}
            </div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.POLL ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.POLL);
            }}
          >
            <Vote className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-white text-sm">{t('poll')}</div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.RECOMMANDATION ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              handleUpdateSelectedType(DeliberationTab.RECOMMANDATION);
            }}
          >
            <CheckCircle className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-white text-sm">
              {t('recommendation')}
            </div>
          </div>

          {(space.author.some((a) => a.id === userId) || selectedTeam) &&
            status == SpaceStatus.InProgress && (
              <div
                className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${
                  selectedType == DeliberationTab.ANALYZE
                    ? 'bg-neutral-800'
                    : ''
                }`}
                onClick={() => {
                  handleUpdateSelectedType(DeliberationTab.ANALYZE);
                }}
              >
                <PieChart1 className="[&>path]:stroke-neutral-80 w-5 h-5" />
                <div className="font-bold text-white text-sm">
                  {t('analyze')}
                </div>
              </div>
            )}
        </div>
      </BlackBox>
      <BlackBox>
        <div className="w-full text-sm text-white">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-1 text-neutral-400 font-semibold text-[14px]">
              <Clock width={20} height={20} />
              {t('timeline')}
            </div>
            {isEdit ? (
              <div
                className="cursor-pointer w-fit h-fit"
                onClick={() => {
                  popup
                    .open(
                      <SetSchedulePopup
                        startedAt={startedAt}
                        endedAt={endedAt}
                        onconfirm={(startDate: number, endDate: number) => {
                          handleUpdateStartDate(Math.floor(startDate / 1000));
                          handleUpdateEndDate(Math.floor(endDate / 1000));
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
            )}
          </div>

          <div className="flex flex-col pl-3.25 gap-5">
            {[
              { label: t('created'), date: createdAt },
              deliberationEndedAt
                ? { label: t('deliberation'), date: deliberationEndedAt }
                : null,
              { label: t('poll_open'), date: startedAt },
              { label: t('poll_close'), date: endedAt },
            ]
              .filter(
                (item): item is { label: string; date: number } =>
                  item !== null,
              )
              .map((item) => (
                <div className="flex flex-col gap-1" key={item.label}>
                  <div className="font-medium text-white text-[15px]/[12px]">
                    {item.label}
                  </div>
                  <div className="font-medium text-neutral-80 text-xs/[12px]">
                    {getTimeWithFormat(item.date ?? 0)}
                  </div>
                </div>
              ))}
          </div>
        </div>
      </BlackBox>
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
