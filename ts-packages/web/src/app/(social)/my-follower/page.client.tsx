'use client';
import { Add, ArrowLeft } from '@/components/icons';
import { Follower } from '@/lib/api/models/network';
import { useRouter } from 'next/navigation';
import React, { useState } from 'react';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { usePopup } from '@/lib/contexts/popup-service';
import UnFollowPopup from '../_popups/unfollow-popup';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  followRequest,
  unfollowRequest,
} from '@/lib/api/models/networks/follow';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { RelationType } from '@/types/relation-type';

const FollowTab = {
  FOLLOWERS: 'Followers',
  FOLLOWINGS: 'Followings',
} as const;

type FollowTabType = (typeof FollowTab)[keyof typeof FollowTab];

export default function MyFollower({ type }: { type: RelationType }) {
  const { post } = useApiCall();
  const popup = usePopup();
  const data = useSuspenseUserInfo();
  const router = useRouter();
  let initTab: FollowTabType = FollowTab.FOLLOWERS;

  if (type === RelationType.FOLLOWING) {
    initTab = FollowTab.FOLLOWINGS;
  }
  const [selectedType, setSelectedType] = useState<FollowTabType>(initTab);

  const userInfo = data.data;

  const followers = userInfo.followers;
  const followings = userInfo.followings;
  const handleUnFollow = async (userId: number) => {
    await post(ratelApi.networks.unfollow(userId), unfollowRequest());
  };

  const handleFollow = async (userId: number) => {
    await post(ratelApi.networks.follow(userId), followRequest());
  };

  return (
    <div className="flex flex-col w-full gap-5">
      <div className="flex flex-row w-full justify-start items-center gap-2.5">
        <div
          className="cursor-pointer w-fit h-fit"
          onClick={() => {
            router.back();
          }}
        >
          <ArrowLeft width={24} height={24} />
        </div>

        <div className="font-semibold text-white text-[20px]">My Network</div>
      </div>

      <SelectedType
        selectedType={selectedType}
        setSelectedType={setSelectedType}
      />

      {selectedType == FollowTab.FOLLOWERS ? (
        <FollowingInfo
          users={followers}
          selectedType={selectedType}
          followings={followings.map((f) => f.id)}
          unfollow={() => {}}
          follow={async (index: number) => {
            try {
              await handleFollow(followers[index].id);
              data.refetch();

              showSuccessToast('success to follow user');
              popup.close();
            } catch (err) {
              showErrorToast('failed to follow user');
              logger.error('failed to follow user with error: ', err);
              popup.close();
            }
          }}
        />
      ) : (
        <FollowingInfo
          users={followings}
          selectedType={selectedType}
          follow={() => {}}
          unfollow={(index: number) => {
            popup
              .open(
                <UnFollowPopup
                  username={followings[index].username}
                  email={followings[index].email}
                  oncancel={() => {
                    popup.close();
                  }}
                  unfollow={async () => {
                    try {
                      await handleUnFollow(followings[index].id);
                      data.refetch();

                      showSuccessToast('success to unfollow user');
                      popup.close();
                    } catch (err) {
                      showErrorToast('failed to unfollow user');
                      logger.error('failed to unfollow user with error: ', err);
                      popup.close();
                    }
                  }}
                />,
              )
              .withTitle('UnFollow');
          }}
        />
      )}
    </div>
  );
}

function FollowingInfo({
  users,
  selectedType,
  followings = [],
  unfollow,
  follow,
}: {
  users: Follower[];
  selectedType: FollowTabType;
  unfollow: (index: number) => void;
  follow: (index: number) => void;
  followings?: number[];
}) {
  return (
    <div className="flex flex-col w-full rounded-lg bg-[#191919] px-4 py-5 gap-2.5">
      {users.length != 0 ? (
        <div className="flex flex-col">
          {users.map((user, index) => (
            <div
              key={user.id}
              className="flex flex-col w-full gap-[5px] px-2.5 py-5 border-b border-b-neutral-800"
            >
              <div className="flex flex-row w-full justify-between items-start">
                <div className="flex flex-row w-fit gap-2">
                  {user.user_type == UserType.Team ? (
                    user.profile_url ? (
                      <Image
                        width={32}
                        height={32}
                        src={user.profile_url || '/default-profile.png'}
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
                      src={user.profile_url || '/default-profile.png'}
                      alt="Profile"
                      className="w-8 h-8 rounded-full object-cover"
                    />
                  ) : (
                    <div className="w-8 h-8 rounded-full bg-neutral-500" />
                  )}

                  <div className="flex flex-col">
                    <div className="font-semibold text-white text-sm/[20px]">
                      {user.nickname}
                    </div>
                    <div className="font-medium text-neutral-500 text-[12px]">
                      @{user.username}
                    </div>
                  </div>
                </div>

                {selectedType == FollowTab.FOLLOWINGS ? (
                  <UnFollowButton
                    onClick={() => {
                      unfollow(index);
                    }}
                  />
                ) : (
                  !followings.includes(user.id) && (
                    <FollowButton
                      onClick={() => {
                        follow(index);
                      }}
                    />
                  )
                )}
              </div>

              <div
                id="user-profile-description"
                className="font-medium text-[12px] text-neutral-300"
                dangerouslySetInnerHTML={{ __html: user.html_contents }}
              />
            </div>
          ))}
        </div>
      ) : (
        <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
          Following data is empty
        </div>
      )}
    </div>
  );
}

function FollowButton({ onClick }: { onClick: () => void }) {
  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] bg-white hover:bg-gray-300 rounded-[50px]"
      onClick={() => {
        onClick();
      }}
    >
      <Add className="w-[15px] h-[15px]" />
      <div className="font-bold text-[#000203] text-xs">Follow</div>
    </div>
  );
}

function UnFollowButton({ onClick }: { onClick: () => void }) {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] bg-transparent border border-neutral-700 hover:border-[#ff4d4f] hover:bg-[#ffe3e3] rounded-[50px]"
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <div
        className={`font-bold  ${isHovered ? 'text-[#ff4d4f]' : 'text-neutral-700'} text-xs`}
      >
        {isHovered ? 'Unfollow' : 'Following'}
      </div>
    </div>
  );
}

function SelectedType({
  selectedType,
  setSelectedType,
}: {
  selectedType: FollowTabType;
  setSelectedType: (selectedType: FollowTabType) => void;
}) {
  return (
    <div className="flex flex-row w-full justify-center items-center">
      <div
        className={`cursor-pointer flex flex-col w-[180px] h-[35px] justify-start items-center text-white text-base ${
          selectedType === FollowTab.FOLLOWERS
            ? 'border-b border-b-neutral-500 font-semibold'
            : 'border-none font-normal'
        }`}
        onClick={() => setSelectedType(FollowTab.FOLLOWERS)}
      >
        {FollowTab.FOLLOWERS}
      </div>
      <div
        className={`cursor-pointer flex flex-col w-[180px] h-[35px] justify-start items-center  text-white text-base ${
          selectedType === FollowTab.FOLLOWINGS
            ? 'border-b border-b-neutral-500 font-semibold'
            : 'border-none font-normal'
        }`}
        onClick={() => setSelectedType(FollowTab.FOLLOWINGS)}
      >
        {FollowTab.FOLLOWINGS}
      </div>
    </div>
  );
}
