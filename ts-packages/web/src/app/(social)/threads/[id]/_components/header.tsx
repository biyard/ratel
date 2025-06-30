'use client';

import { useFeedByID } from '@/app/(social)/_hooks/feed';
import { ArrowLeft } from '@/components/icons';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { UserType } from '@/lib/api/models/user';
import { getTimeAgo } from '@/lib/time-utils';
import { Plus } from 'lucide-react';
import Image from 'next/image';
import { BadgeIcon } from '@/components/icons';
import Link from 'next/link';
import { route } from '@/route';
import { usePopup } from '@/lib/contexts/popup-service';
import SpaceCreateModal from './space-create-modal';
import { SpaceType } from '@/lib/api/models/spaces';
import { useRouter } from 'next/navigation';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';

export default function Header({ post_id }: { post_id: number }) {
  const { data: post } = useFeedByID(post_id);
  const popup = usePopup();
  const router = useRouter();
  const user = useSuspenseUserInfo();
  const space_id = post?.spaces[0]?.id;

  console.log('post author: ', post?.author);

  const author_id = post?.author[0].id;
  const user_id = user.data.id;

  let target;
  if (space_id) {
    if (post.spaces[0].space_type === SpaceType.Deliberation) {
      target = route.deliberationSpaceById(space_id);
    } else {
      target = route.commiteeSpaceById(space_id);
    }
  }
  const handleCreateSpace = () => {
    popup
      .open(<SpaceCreateModal feed_id={post_id} />)
      .withoutBackdropClose()
      .withTitle('Select a Space Type');
  };
  return (
    <div className="flex flex-col w-full gap-2.5">
      <button onClick={router.back}>
        <ArrowLeft />
      </button>
      <div className="flex flex-row justify-between">
        <div>
          {post?.industry?.map((industry) => (
            <Badge
              key={industry.id}
              variant="outline"
              className="border-c-wg-70 mr-2"
              size="lg"
            >
              {industry.name}
            </Badge>
          ))}
        </div>
        {space_id ? (
          <Link href={target ?? ''}>
            <Button variant="rounded_secondary">Join Space</Button>
          </Link>
        ) : author_id == user_id ? (
          <Button variant="rounded_secondary" onClick={handleCreateSpace}>
            <Plus className="size-5" />
            Create Space
          </Button>
        ) : (
          <></>
        )}
      </div>

      <div>
        <h2 className="text-2xl font-bold">{post?.title}</h2>
      </div>
      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={post?.author[0]?.profile_url ?? ''}
          proposerName={post?.author[0]?.nickname ?? ''}
          userType={post?.author[0]?.user_type || UserType.Individual}
        />
        <div className="font-light text-white text-sm/[14px]">
          {post?.created_at !== undefined ? getTimeAgo(post.created_at) : ''}
        </div>
      </div>
    </div>
  );
}

export function ProposerProfile({
  profileUrl = '',
  proposerName = '',
  userType = UserType.Individual,
}: {
  profileUrl: string;
  proposerName: string;
  userType: UserType;
}) {
  return (
    <div className="flex flex-row w-fit gap-2 justify-between items-center">
      <Image
        src={profileUrl || '/default-profile.png'}
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
      <BadgeIcon />
    </div>
  );
}
