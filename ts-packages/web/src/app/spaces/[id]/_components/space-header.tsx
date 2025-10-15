// import Shared from '@/assets/icons/share.svg?react';
// import Extra from '@/assets/icons/extra.svg?react';
// import Bookmark from '@/assets/icons/bookmark.svg?react';
import Badge from '@/assets/icons/badge.svg?react';
import { UserType } from '@/lib/api/models/user';

import { Input } from '@/components/ui/input';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { Play } from 'lucide-react';
import TimeAgo from './time-ago-wrapper';

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

/**
 * @deprecated
 * Use `SpaceHeader` from features/spaces/components/header instead.
 */
export default function SpaceHeader({
  title,
  status,
  userType,
  proposerImage,
  proposerName,
  createdAt,
  isEdit = false,
  setTitle = () => {},
}: SpaceHeaderProps) {
  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-col gap-2.5">
        <div className="flex flex-row w-full justify-start items-center gap-2.5">
          {/* <SpaceType /> */}
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
          {proposerImage && proposerImage !== '' ? (
            <img
              src={proposerImage}
              alt={proposerName}
              className={
                userType == UserType.Team
                  ? 'rounded-lg object-cover object-top w-6.25 h-6.25'
                  : 'rounded-full object-cover object-top w-6.25 h-6.25'
              }
            />
          ) : (
            <div className="w-6.25 h-6.25 rounded-full border border-neutral-500 bg-neutral-600" />
          )}
          <div className="font-semibold text-white text-sm/[20px]">
            {proposerName}
          </div>
          <Badge />
        </div>

        <TimeAgo timestamp={createdAt} />
      </div>
    </div>
  );
}

function Onboard() {
  return (
    <div className="flex flex-row items-center w-fit px-2 gap-1 border border-[#05df72] opacity-50 rounded-sm">
      <Play className="w-[10px] h-[10px] stroke-[#00d492]-[#00d492]" />
      <div className="font-semibold text-sm/[25px] text-[#00d492]">ONBOARD</div>
    </div>
  );
}

// function SpaceType() {
//   return (
//     <div className="flex flex-row w-fit h-fit px-2 bg-transparent rounded-sm border border-c-wg-70 font-semibold text-white text-xs/[25px]">
//       Crypto
//     </div>
//   );
// }
