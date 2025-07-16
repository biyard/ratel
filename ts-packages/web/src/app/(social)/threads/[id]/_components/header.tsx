'use client';

import { useFeedByID } from '@/app/(social)/_hooks/feed';
import { ArrowLeft, Palace } from '@/components/icons';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { UserType } from '@/lib/api/models/user';
import { getTimeAgo } from '@/lib/time-utils';
import { Trash2, Edit } from 'lucide-react';
import Image from 'next/image';
import { BadgeIcon, Extra, UnlockPublic } from '@/components/icons';
import Link from 'next/link';
import { route } from '@/route';
import { usePopup } from '@/lib/contexts/popup-service';
import SpaceCreateModal from './space-create-modal';
import { SpaceType } from '@/lib/api/models/spaces';
import { useRouter } from 'next/navigation';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { useContext, useState, useEffect } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { showSuccessToast, showErrorToast } from '@/lib/toast';
import { usePostDraft } from '@/app/(social)/_components/create-post';

export default function Header({ post_id }: { post_id: number }) {
  const { data: post } = useFeedByID(post_id);
  const popup = usePopup();
  const router = useRouter();
  const { teams } = useContext(TeamContext);
  const user = useSuspenseUserInfo();

  const author_id = post?.author[0].id;
  const [selectedTeam, setSelectedTeam] = useState<boolean>(false);
  const { post: apiPost } = useApiCall();
  const { loadDraft } = usePostDraft();

  const space_id = post?.spaces[0]?.id;

  const user_id = user.data ? user.data.id : 0;

  useEffect(() => {
    const index = teams.findIndex((t) => t.id === author_id);
    setSelectedTeam(index !== -1);
  }, [teams, author_id]);

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

  const handleDeletePost = async () => {
    try {
      await apiPost(ratelApi.feeds.removeDraft(post_id), { delete: {} });
      showSuccessToast('Post deleted successfully');
      router.push('/'); // Navigate to homepage after successful deletion
    } catch (error) {
      console.error('Failed to delete post:', error);
      showErrorToast('Failed to delete post. Please try again.');
      // Remain on the feed page on failure
    }
  };

  const handleEditPost = async () => {
    try {
      await loadDraft(post_id);
    } catch (error) {
      console.error('Failed to load draft for editing:', error);
      showErrorToast('Failed to load post for editing. Please try again.');
    }
  };

  const isPostOwner = author_id === user_id || selectedTeam;

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row justify-between items-center">
        <button onClick={router.back}>
          <ArrowLeft />
        </button>
        <div className="flex items-center space-x-2.5">
          {space_id ? (
            <Link href={target ?? ''}>
              <Button variant="rounded_secondary" className="max-tablet:hidden">
                Join Space
              </Button>
            </Link>
          ) : isPostOwner ? (
            <>
              <Button
                variant="default"
                className="rounded-md max-tablet:hidden text-lg px-3 py-1.5"
                onClick={handleEditPost}
              >
                <Edit className="size-6" />
                Edit
              </Button>
              <Button
                variant="default"
                className="rounded-md max-tablet:hidden text-lg px-3 py-1.5"
              >
                <UnlockPublic className="size-6 [&>path]:stroke-black" />
                Make Public
              </Button>
              <Button
                variant="rounded_secondary"
                onClick={handleCreateSpace}
                className="max-tablet:hidden bg-[#FCB300] hover:bg-[#FCB300]/80 text-lg px-3 py-1.5"
              >
                <Palace className="size-6" />
                Create a Space
              </Button>
            </>
          ) : (
            <></>
          )}

          {/* 3-dot overflow menu - only shown for post owners or when there's a space to join */}
          {(isPostOwner || space_id) && (
            <DropdownMenu modal={false}>
              <DropdownMenuTrigger asChild>
                <button
                  className="p-1 hover:bg-gray-700 rounded-full focus:outline-none transition-colors"
                  aria-haspopup="true"
                  aria-label="Post options"
                >
                  <Extra className="size-6 text-gray-400" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                align="end"
                className="w-40 bg-[#404040] border-gray-700 transition ease-out duration-100"
              >
                {/* Mobile-only menu items */}
                <div className="hidden max-tablet:block">
                  {space_id ? (
                    <DropdownMenuItem asChild>
                      <Link href={target ?? ''}>
                        <button className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer">
                          Join Space
                        </button>
                      </Link>
                    </DropdownMenuItem>
                  ) : isPostOwner ? (
                    <>
                      <DropdownMenuItem asChild>
                        <button
                          onClick={handleCreateSpace}
                          className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer"
                        >
                          <Palace className="w-4 h-4 [&>path]:stroke-white" />
                          Create a Space
                        </button>
                      </DropdownMenuItem>
                      <DropdownMenuItem asChild>
                        <button
                          onClick={handleEditPost}
                          className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer"
                        >
                          <Edit className="w-4 h-4" />
                          Edit
                        </button>
                      </DropdownMenuItem>
                      <DropdownMenuItem asChild>
                        <button className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer">
                          <UnlockPublic className="w-4 h-4 [&>path]:stroke-white" />
                          Make Public
                        </button>
                      </DropdownMenuItem>
                    </>
                  ) : null}
                </div>

                {/* Always visible delete option for post owners */}
                {isPostOwner && (
                  <DropdownMenuItem asChild>
                    <button
                      onClick={handleDeletePost}
                      className="flex items-center w-full px-4 py-2 text-sm text-red-400 hover:bg-gray-700 cursor-pointer"
                    >
                      <Trash2 className="w-4 h-4" />
                      Delete
                    </button>
                  </DropdownMenuItem>
                )}
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </div>
      </div>
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
