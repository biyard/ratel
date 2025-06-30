import React from 'react';
import ArrowLeft from '@/assets/icons/left.svg';
// import Shared from '@/assets/icons/share.svg';
// import Extra from '@/assets/icons/extra.svg';
// import Bookmark from '@/assets/icons/bookmark.svg';
import Badge from '@/assets/icons/badge.svg';
import { getTimeAgo } from '@/lib/time-utils';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { Input } from '@/components/ui/input';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { Play } from 'lucide-react';

export interface SpaceHeaderProps {
  title: string;
  status: SpaceStatus;
  userType: UserType;
  proposerImage: string;
  proposerName: string;
  createdAt: number;

  isEdit?: boolean;
  onback: () => void;
  setTitle?: (title: string) => void;
}

export default function SpaceHeader({
  title,
  status,
  userType,
  proposerImage,
  proposerName,
  createdAt,
  isEdit = false,
  onback = () => {},
  setTitle = () => {},
}: SpaceHeaderProps) {
  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row w-full justify-between items-center">
        <div
          className="cursor-pointer w-fit h-fit"
          onClick={() => {
            onback();
          }}
        >
          <ArrowLeft width={24} height={24} />
        </div>

        {/* <div className="flex flex-row w-fit h-fit items-center gap-5">
          <Shared width={24} height={24} />
          <Extra width={24} height={24} />
        </div> */}
      </div>

      <div className="flex flex-col gap-2.5">
        <div className="flex flex-row w-full justify-start items-center gap-2.5">
          <SpaceType />
          {status == SpaceStatus.InProgress ? <Onboard /> : <></>}
        </div>
        <div className="flex flex-row w-full justify-between items-center">
          {isEdit ? (
            <Input
              value={title}
              onChange={(e) => {
                setTitle(e.target.value);
              }}
            />
          ) : (
            <div className="font-bold text-white text-[20px]/[30px]">
              {title}
            </div>
          )}
          {/* <Bookmark width={20} height={20} /> */}
        </div>
      </div>

      <div className="flex flex-row w-full justify-between items-center">
        <div className="flex flex-row w-fit gap-2 justify-between items-center">
          <Image
            src={proposerImage || '/default-profile.png'}
            alt={proposerName}
            width={20}
            height={20}
            className={
              userType == UserType.Team
                ? 'rounded-[8px] object-cover object-top w-[25px] h-[25px]'
                : 'rounded-full object-cover object-top w-[25px] h-[25px]'
            }
          />
          <div className="font-semibold text-white text-sm/[20px]">
            {proposerName}
          </div>
          <Badge />
        </div>

        <div className="font-light text-white text-sm/[14px]">
          {getTimeAgo(createdAt)}
        </div>
      </div>
    </div>
  );
}

function Onboard() {
  return (
    <div className="flex flex-row items-center w-fit px-2 gap-1 border border-[#05df72] opacity-50 rounded-sm">
      <Play className="w-[10px] h-[10px] stroke-[#00d492] fill-[#00d492]" />
      <div className="font-semibold text-sm/[25px] text-[#00d492]">ONBOARD</div>
    </div>
  );
}

function SpaceType() {
  return (
    <div className="flex flex-row w-fit h-fit px-2 bg-transparent rounded-sm border border-c-wg-70 font-semibold text-white text-xs/[25px]">
      Crypto
    </div>
  );
}
