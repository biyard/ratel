import { useCallback, useMemo } from 'react';
import { Col } from '@/components/ui/col';
import { useObserver } from '@/hooks/use-observer';
import useInfiniteMySpaces from './_hooks/use-my-spaces';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import Card from '@/components/card';
import { MySpace } from '@/features/spaces/types/space-common';
import { Row } from '@/components/ui/row';
import { useMySpacesI18n } from './i18n';
import { useUserInfo } from '@/hooks/use-user-info';

function SpaceCard({ space }: { space: MySpace }) {
  const user = useUserInfo();
  const navigate = useNavigate();
  const pk = user.data.pk;
  const isBlocked = pk === space.user_pk ? false : !!space.block_participate;

  const t = useMySpacesI18n();
  const status = useMemo(() => {
    if (isBlocked) {
      return 'blocked';
    }

    return space.invitation_status;
  }, [isBlocked, space.invitation_status]);

  const handleClick = () => {
    if (isBlocked) {
      return; // Don't navigate if participation is blocked
    }
    navigate(route.space(space.pk));
  };

  const getInvitationStatusStyle = (status: 'pending' | 'participating') => {
    if (status === 'pending') {
      return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
    }
    return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
  };

  return (
    <Card
      onClick={handleClick}
      className={`transition-colors ${
        isBlocked
          ? 'cursor-not-allowed opacity-60 bg-gray-50 dark:bg-gray-800'
          : 'cursor-pointer hover:bg-card-bg-hover'
      }`}
      data-testid="space-card"
    >
      <div className="flex flex-col gap-2">
        <div className="flex gap-3 items-center">
          <div className="flex flex-col">
            <h3 className="text-base font-semibold text-text-primary">
              {space.title}
            </h3>
            <Row>
              {space.author_profile_url && (
                <img
                  className="w-5 rounded-full"
                  src={space.author_profile_url}
                />
              )}
              <p className="text-sm text-text-secondary">
                {space.author_display_name}
              </p>
            </Row>
          </div>
        </div>

        <div className="flex gap-2 items-center text-sm text-text-secondary">
          <span
            className={`px-2 py-1 rounded font-medium ${getInvitationStatusStyle(space.invitation_status)}`}
          >
            {t.status[status]}
          </span>
        </div>
      </div>
    </Card>
  );
}

function EmptyState() {
  return (
    <div className="flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]">
      No spaces available
    </div>
  );
}

function FeedEndMessage({ msg }: { msg: string }) {
  return (
    <div className="flex flex-row justify-center items-center w-full text-base font-medium text-gray-500 h-fit px-[16px] py-[20px]">
      {msg}
    </div>
  );
}

export default function MySpacesPage() {
  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useInfiniteMySpaces();

  const handleIntersect = useCallback(() => {
    if (hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [fetchNextPage, hasNextPage, isFetchingNextPage]);

  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  const flattedSpaces = useMemo(() => {
    const spaces = data?.pages.flatMap((page) => page.items) ?? [];

    // Sort spaces: pending (non-blocked) -> participating -> blocked
    return spaces.sort((a, b) => {
      const getPriority = (space: MySpace) => {
        const isBlocked =
          space.invitation_status === 'pending' && space.block_participate;

        if (isBlocked) return 2; // Blocked spaces last
        if (space.invitation_status === 'pending') return 0; // Pending first
        return 1; // Participating second
      };

      return getPriority(a) - getPriority(b);
    });
  }, [data?.pages]);

  if (flattedSpaces.length === 0) {
    return <EmptyState />;
  }

  return (
    <div className="flex relative flex-1">
      <Col className="flex flex-1 max-mobile:px-2.5">
        <Col className="flex-1 gap-4">
          {flattedSpaces.map((space) => (
            <SpaceCard key={`space-${space.pk}`} space={space} />
          ))}

          <div ref={observerRef} />
          {!hasNextPage && (
            <FeedEndMessage msg="You have reached the end of your spaces." />
          )}
        </Col>
      </Col>
    </div>
  );
}
