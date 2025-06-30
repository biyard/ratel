'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import { getTimeWithFormat } from '@/lib/time-utils';
import React, { useEffect, useRef, useState } from 'react';
import Clock from '@/assets/icons/clock.svg';
import { BottomTriangle, Discuss, Edit1 } from '@/components/icons';
import { File, Vote, CheckCircle } from 'lucide-react';
import { DeliberationTab } from '../types';
import { SpaceStatus } from '@/lib/api/models/spaces';
import {
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from '../provider.client';
import { useUserInfo } from '@/app/(social)/_hooks/user';

export default function SpaceSideMenu() {
  const {
    selectedType,
    setSelectedType,
    isEdit,
    status,
    handlePostingSpace,
    handleEdit: onedit,
    handleSave: onsave,
  } = useDeliberationSpaceContext();
  const space = useDeliberationSpace();

  const { data: userInfo } = useUserInfo();
  const userId = userInfo ? userInfo.id : 0;
  const created_at = space.created_at;

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      {space.author.some((a) => a.id === userId) && (
        <EditSplitButton
          status={status}
          isEdit={isEdit}
          postingSpace={handlePostingSpace}
          onedit={onedit}
          onsave={onsave}
        />
      )}
      <BlackBox>
        <div className="flex flex-col gap-2.5 w-full">
          <div
            className={`cursor-pointer flex flex-row w-full gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.SUMMARY ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              setSelectedType(DeliberationTab.SUMMARY);
            }}
          >
            <File className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-white text-sm">Summary</div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.DELIBERATION ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              setSelectedType(DeliberationTab.DELIBERATION);
            }}
          >
            <Discuss className="w-5 h-5" />
            <div className="font-bold text-white text-sm">Deliberation</div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.POLL ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              setSelectedType(DeliberationTab.POLL);
            }}
          >
            <Vote className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-white text-sm">Poll</div>
          </div>

          <div
            className={`cursor-pointer flex flex-row gap-1 items-center px-1 py-2 rounded-sm ${selectedType == DeliberationTab.RECOMMANDATION ? 'bg-neutral-800' : ''}`}
            onClick={() => {
              setSelectedType(DeliberationTab.RECOMMANDATION);
            }}
          >
            <CheckCircle className="[&>path]:stroke-neutral-80 w-5 h-5" />
            <div className="font-bold text-white text-sm">Recommandation</div>
          </div>
        </div>
      </BlackBox>
      <BlackBox>
        <div className="flex flex-col gap-5 w-full">
          <div className="flex flex-row gap-1 items-center">
            <Clock width={20} height={20} />
            <div className="font-bold text-neutral-500 text-sm/[14px]">
              Created
            </div>
          </div>

          <div className="flex flex-col w-full gap-[6px]">
            <div className="font-medium text-neutral-80 text-xs/[12px]">
              {getTimeWithFormat(created_at)}
            </div>
          </div>
        </div>
      </BlackBox>
    </div>
  );
}

function EditSplitButton({
  isEdit,
  status,
  postingSpace,
  onedit,
  onsave,
}: {
  isEdit: boolean;
  status: SpaceStatus;
  postingSpace: () => void;
  onedit: () => void;
  onsave: () => void;
}) {
  const [showPopup, setShowPopup] = useState(false);
  const popupRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        popupRef.current &&
        !popupRef.current.contains(event.target as Node)
      ) {
        setShowPopup(false);
      }
    }

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  return (
    <div className="relative flex items-center w-full h-[46px] gap-2">
      {/* Left "Edit" Button */}
      {status != SpaceStatus.InProgress ? (
        <button
          className={`flex items-center justify-start flex-row w-full bg-white hover:bg-neutral-300 text-black px-4 py-3 gap-1 rounded-l-[100px] rounded-r-[4px]'}`}
          onClick={() => {
            if (isEdit) {
              onsave();
            } else {
              onedit();
            }
          }}
        >
          <Edit1 className="w-[18px] h-[18px]" />
          <span className="font-bold text-neutral-900 text-base/[22px]">
            {isEdit ? 'Save' : 'Edit'}
          </span>
        </button>
      ) : (
        <></>
      )}

      {/* Right Dropdown Toggle */}
      {status != SpaceStatus.InProgress ? (
        <div className="relative h-full" ref={popupRef}>
          <button
            className="w-[48px] h-full flex items-center justify-center bg-neutral-500 rounded-r-[100px] rounded-l-[4px]"
            onClick={() => setShowPopup((prev) => !prev)}
          >
            <BottomTriangle />
          </button>

          {/* Pop-up Menu */}
          {showPopup && (
            <div
              className="absolute top-full right-0 mt-2 px-4 py-2 min-w-[150px] bg-white hover:bg-neutral-300 text-black rounded shadow-lg text-sm cursor-pointer whitespace-nowrap z-50"
              onClick={() => {
                postingSpace();
                setShowPopup(false);
              }}
            >
              Posting
            </div>
          )}
        </div>
      ) : (
        <></>
      )}
    </div>
  );
}
