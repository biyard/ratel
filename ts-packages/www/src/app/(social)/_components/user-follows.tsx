import { route } from '@/route';
import { RelationType } from '@/types/relation-type';
import { NavLink } from 'react-router';

export interface UserFollowsProps {
  followers_count: number;
  followings_count: number;
}

export default function UserFollows({
  followers_count,
  followings_count,
}: UserFollowsProps) {
  return (
    <div className="flex flex-row w-full justify-around items-center gap-[20px] max-tablet:gap-[10px]">
      <NavLink
        className="flex flex-col w-fit justify-start items-center gap-[2px] text-text-secondary hover:text-text-secondary/80"
        to={route.myFollower(RelationType.FOLLOWER)}
      >
        <div className="font-bold text-sm">
          {followers_count.toLocaleString()}
        </div>
        <div className="font-medium text-xs">Followers</div>
      </NavLink>

      <NavLink
        className="flex flex-col w-fit justify-start items-center gap-[2px] text-text-secondary hover:text-text-secondary/80"
        to={route.myFollower(RelationType.FOLLOWING)}
      >
        <div className="font-bold text-sm">
          {followings_count.toLocaleString()}
        </div>
        <div className="font-medium text-xs">Followings</div>
      </NavLink>
    </div>
  );
}
