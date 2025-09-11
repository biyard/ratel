'use client';
import React, { useEffect, useState } from 'react';
import { useNetwork } from '../_hooks/use-network';
import { logger } from '@/lib/logger';
import { Follower } from '@/lib/api/models/network';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { Add } from '@/components/icons';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { followRequest } from '@/lib/api/models/networks/follow';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { checkString } from '@/lib/string-filter-utils';
import Link from 'next/link';
import { route } from '@/route';

export default function MyNetwork() {
  const { post } = useApiCall();
  const data = useSuspenseUserInfo();
  const network = useNetwork();
  const networkData = network.data;

  // Refetch network once on mount
  useEffect(() => {
    const timer = setTimeout(() => {
      network.refetch();
    }, 0);
    return () => clearTimeout(timer);
  }, []);

  // handle API follow call
  const handleFollow = async (userId: number) => {
    await post(ratelApi.networks.follow(userId), followRequest());
  };

  logger.debug('query response of networks', networkData);

  return (
    <div className="flex flex-col w-full gap-3">
      <FollowingContents
        label="Suggested teams"
        users={
          networkData
            ? networkData.suggested_teams.filter(
                (team) => !checkString(team.username),
              )
            : []
        }
        follow={async (userId: number) => {
          logger.debug('follow button clicked user id: ', userId);
          try {
            await handleFollow(userId);
            data.refetch();
            network.refetch();
            showSuccessToast('Successfully followed user');
          } catch (err) {
            showErrorToast('Failed to follow user');
            logger.error('failed to follow user with error: ', err);
          }
        }}
      />

      <FollowingContents
        label="Suggested users"
        users={
          networkData
            ? networkData.suggested_users.filter(
                (user) => !checkString(user.username),
              )
            : []
        }
        follow={async (userId: number) => {
          logger.debug('follow button clicked user id: ', userId);
          try {
            await handleFollow(userId);
            data.refetch();
            network.refetch();
            showSuccessToast('Successfully followed user');
          } catch (err) {
            showErrorToast('Failed to follow user');
            logger.error('failed to follow user with error: ', err);
          }
        }}
      />
    </div>
  );
}

/* ---------------- FOLLOW BUTTON ---------------- */
function FollowButton({
  isFollowing,
  onClick,
}: {
  isFollowing: boolean;
  onClick: () => void;
}) {
  return (
    <div
      className={`cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] border rounded-[50px]
      ${
        isFollowing
          ? 'bg-gray-200 border-gray-300'
          : 'bg-white border-white hover:bg-gray-300'
      }`}
      onClick={onClick}
    >
      {!isFollowing && (
        <Add className="w-[15px] h-[15px] [&>path]:stroke-[#000203]" />
      )}
      <div className="font-bold text-[#000203] text-xs">
        {isFollowing ? 'Following' : 'Follow'}
      </div>
    </div>
  );
}

/* ---------------- FOLLOWING CONTENTS ---------------- */
function FollowingContents({
  label,
  users,
  follow,
}: {
  label: string;
  users: Follower[];
  follow: (userId: number) => Promise<void>;
}) {
  // Track which users have been followed in this session
  const [followedUsers, setFollowedUsers] = useState<number[]>([]);

  const handleFollowClick = async (userId: number) => {
    try {
      await follow(userId);
      setFollowedUsers((prev) => [...prev, userId]);
    } catch (err) {
      // error is already handled by parent follow()
    }
  };

  return (
    <div className="flex flex-col w-full rounded-lg bg-card-bg border border-card-border px-4 py-5 gap-2.5">
      <div className="font-semibold text-text-primary text-base/[20px]">
        {label}
      </div>
      <div className="flex flex-col">
        {users.map((user) => {
          const isFollowing = followedUsers.includes(user.id);
          return (
            <div
              key={user.id}
              className="flex flex-col w-full gap-[5px] px-2.5 py-5 border-b border-b-divider"
            >
              <div className="flex flex-row w-full justify-between items-start">
                {/* Profile section */}
                <div className="flex flex-row w-fit gap-2">
                  {user.user_type == UserType.Team ? (
                    user.profile_url ? (
                      <Image
                        width={32}
                        height={32}
                        src={user.profile_url}
                        alt="Profile"
                        className="w-8 h-8 rounded-lg object-cover"
                      />
                    ) : (
                      <div className="w-8 h-8 rounded-lg bg-neutral-500" />
                    )
                  ) : user.profile_url ? (
                    <Image
                      width={32}
                      height={32}
                      src={user.profile_url}
                      alt="Profile"
                      className="w-8 h-8 rounded-full object-cover"
                    />
                  ) : (
                    <div className="w-8 h-8 rounded-full bg-neutral-500" />
                  )}

                  <div className="flex flex-col">
                    <Link href={route.teamByUsername(user.username)}>
                      <div className="font-semibold text-text-primary text-sm/[20px]">
                        {user.nickname}
                      </div>
                    </Link>
                  </div>
                </div>

                {/* Follow button */}
                <FollowButton
                  isFollowing={isFollowing}
                  onClick={() => handleFollowClick(user.id)}
                />
              </div>

              {/* Description */}
              <div
                id="user-profile-description"
                className="font-medium text-[12px] text-desc-text line-clamp-3 overflow-hidden"
                dangerouslySetInnerHTML={{
                  __html: user.html_contents,
                }}
              />
            </div>
          );
        })}
      </div>
    </div>
  );
}


// function SelectedIndustry({ industries }: { industries: Industry[] }) {
//   const PAGE_SIZE = 5;
//   const [selectedIndustry, setSelectedIndustry] = useState('ALL');
//   const [page, setPage] = useState(0);

//   const totalPages = Math.ceil(industries.length / PAGE_SIZE);
//   const visibleItems = industries.slice(
//     page * PAGE_SIZE,
//     (page + 1) * PAGE_SIZE,
//   );

//   return (
//     <div className="flex items-center gap-2">
//       {page > 0 && (
//         <ChevronLeft
//           className="cursor-pointer stroke-neutral-500"
//           onClick={() => setPage((prev) => Math.max(prev - 1, 0))}
//         ></ChevronLeft>
//       )}

//       <div className="flex gap-2 flex-1 flex-wrap">
//         <IndustryLabel
//           name="ALL"
//           selected={selectedIndustry === 'ALL'}
//           setSelectedIndustry={() => setSelectedIndustry('ALL')}
//         />
//         {visibleItems.map((industry) => {
//           const name = industry.name.toUpperCase();
//           return (
//             <IndustryLabel
//               key={name}
//               name={name}
//               selected={selectedIndustry === name}
//               setSelectedIndustry={() => setSelectedIndustry(name)}
//             />
//           );
//         })}
//       </div>

//       {page < totalPages - 1 && (
//         <ChevronRight
//           className="cursor-pointer stroke-neutral-500"
//           onClick={() => setPage((prev) => Math.min(prev + 1, totalPages - 1))}
//         ></ChevronRight>
//       )}
//     </div>
//   );
// }

// function IndustryLabel({
//   name,
//   selected,
//   setSelectedIndustry,
// }: {
//   name: string;
//   selected: boolean;
//   setSelectedIndustry: (name: string) => void;
// }) {
//   return (
//     <div
//       className="cursor-pointer flex flex-row w-fit h-fit px-2.5 py-2 rounded-lg font-semibold text-white text-sm/[20px] whitespace-nowrap border border-neutral-700 bg-transparent hover:bg-neutral-700 aria-selected:border-none aria-selected:bg-neutral-700"
//       aria-selected={selected}
//       onClick={() => {
//         setSelectedIndustry(name);
//       }}
//     >
//       {name}
//     </div>
//   );
// }
