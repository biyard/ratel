'use client';
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Image from 'next/image';
import { Check } from 'lucide-react';
import { useInView } from 'react-intersection-observer';
import News from '@/app/(social)/_components/News';
import Suggestions from '@/app/(social)/_components/suggestions';
import BlackBox from '@/app/(social)/_components/black-box';
import PromotionCard from '@/app/(social)/_components/promotion-card';
import { Col } from '@/components/ui/col';
import { useApiCall } from '@/lib/api/use-send';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { ratelApi, usePromotion } from '@/lib/api/ratel_api';
import { useFeedByID } from '@/app/(social)/_hooks/use-feed';
import {
  NotificationType,
  Notification,
  NotificationsFilter,
  getNotificationType,
  getNotificationContent,
} from './types';
import { useNotificationsInfinite } from '@/hooks/use-notifications';
import NotificationDropdown from './notification-dropdown';
import NotificationReadStatus from './notification-read-status';

const NotificationTab = {
  NOTIFICATIONS: 'Notifications',
  MESSAGES: 'Messages',
} as const;

type NotificationTabType =
  (typeof NotificationTab)[keyof typeof NotificationTab];

export default function NotificationPage() {
  const router = useRouter();
  const { data: promotion } = usePromotion();
  const { data: feed } = useFeedByID(promotion.feed_id);
  const [filterType, setFilterType] = useState<NotificationsFilter>('all');

  const handleTabChange = (newType: NotificationTabType) => {
    if (newType === NotificationTab.MESSAGES) {
      router.push('/messages', { scroll: false });
    } else {
      router.push('/notifications', { scroll: false });
    }
  };

  return (
    <div className="flex min-h-screen gap-5 justify-between max-w-desktop mx-auto text-white py-3 max-tablet:px-2.5 max-mobile:gap-0 max-mobile:py-2">
      <div className="flex-1 flex relative">
        <Col className="flex-1 flex max-mobile:px-[10px]">
          <div className="flex flex-col w-full gap-5 max-mobile:gap-3">
            <SelectedType handleTabChange={handleTabChange} />

            <NotificationsContent
              filterType={filterType}
              setFilterType={setFilterType}
            />
          </div>
        </Col>

        <aside className="w-70 pl-4 max-tablet:!hidden" aria-label="Sidebar">
          {/* <CreatePostButton /> */}

          <BlackBox>
            <PromotionCard promotion={promotion} feed={feed} />
          </BlackBox>

          <News />
          <div className="mt-[10px]">
            <Suggestions />
          </div>
        </aside>
      </div>
    </div>
  );
}

function SelectedType({
  handleTabChange,
}: {
  handleTabChange: (selectedType: NotificationTabType) => void;
}) {
  return (
    <div className="flex flex-row w-full justify-center items-center gap-20 max-mobile:gap-8">
      <div className="cursor-pointer flex flex-col w-[180px] max-mobile:w-auto max-mobile:flex-1 h-[35px] justify-start items-center text-white text-base max-mobile:text-sm font-semibold">
        <div className="relative pb-2">
          <span>{NotificationTab.NOTIFICATIONS}</span>
          <div className="absolute -bottom-1 left-1/2 transform -translate-x-1/2 w-8 h-0.5 bg-primary"></div>
        </div>
      </div>
      <div
        className="cursor-pointer flex flex-col w-[180px] max-mobile:w-auto max-mobile:flex-1 h-[35px] justify-start items-center text-white text-base max-mobile:text-sm font-normal"
        onClick={() => handleTabChange(NotificationTab.MESSAGES)}
      >
        <div className="relative pb-2">
          <span>{NotificationTab.MESSAGES}</span>
        </div>
      </div>
    </div>
  );
}

function NotificationsContent({
  filterType,
  setFilterType,
}: {
  filterType: NotificationsFilter;
  setFilterType: (type: NotificationsFilter) => void;
}) {
  const { ref, inView } = useInView({ threshold: 0.5 });
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
    isLoading,
    refetch,
  } = useNotificationsInfinite(filterType);
  const { post: apiPost } = useApiCall();
  const [isMarkingAllRead, setIsMarkingAllRead] = useState(false);

  const notifications = data?.pages.flatMap((page) => page.items) || [];

  // Infinite scroll effect
  useEffect(() => {
    if (inView && hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [inView, hasNextPage, isFetchingNextPage, fetchNextPage]);

  const handleMarkAllAsRead = async () => {
    if (isMarkingAllRead) return;

    setIsMarkingAllRead(true);
    try {
      await apiPost(ratelApi.notifications.markAllAsRead(), {});
      showSuccessToast('All notifications marked as read');
      refetch(); // Refresh the notifications list
    } catch (error) {
      console.error('Failed to mark all notifications as read:', error);
      showErrorToast(
        'Failed to mark all notifications as read. Please try again.',
      );
    } finally {
      setIsMarkingAllRead(false);
    }
  };

  return (
    <div className="flex flex-col w-full rounded-lg bg-[#191919] px-4 max-mobile:px-2 py-5 max-mobile:py-3 gap-2.5">
      <div className="flex max-mobile:flex-col items-center max-mobile:items-stretch gap-3 max-mobile:gap-2 mb-4 w-full">
        <FilterBar filterType={filterType} setFilterType={setFilterType} />
        <button
          onClick={handleMarkAllAsRead}
          disabled={isMarkingAllRead}
          className="flex items-center gap-2 px-3 max-mobile:px-2 py-2 max-mobile:py-1.5 bg-transparent border border-neutral-600 rounded-full text-white hover:bg-neutral-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap max-mobile:w-full max-mobile:justify-center"
          aria-label="Mark all notifications as read"
        >
          <Check className="w-4 h-4 max-mobile:w-3 max-mobile:h-3" />
          <span className="text-sm max-mobile:text-xs font-medium">
            {isMarkingAllRead ? 'Marking...' : 'Mark All Read'}
          </span>
        </button>
      </div>

      {isLoading ? (
        <div className="text-white text-center py-4 max-mobile:py-3">
          <div className="text-sm max-mobile:text-xs">
            Loading notifications...
          </div>
        </div>
      ) : notifications && notifications.length > 0 ? (
        <div className="flex flex-col">
          {notifications
            .map((notification: Notification) => {
              const content = getNotificationContent(notification);
              const notificationType = getNotificationType(notification);

              // Only show notifications that can be properly parsed
              if (!content) {
                return null;
              }

              return (
                <div
                  key={notification.id}
                  className={`flex flex-col w-full gap-[5px] max-mobile:gap-2 px-2.5 max-mobile:px-2 py-5 max-mobile:py-3 border-b border-b-neutral-800 ${
                    !notification.read ? 'bg-neutral-800/50' : ''
                  }`}
                >
                  <div className="flex flex-row max-mobile:flex-col w-full justify-between items-start max-mobile:gap-2">
                    <div className="flex flex-row w-fit gap-2 flex-1">
                      {content.imageUrl ? (
                        <Image
                          width={32}
                          height={32}
                          src={content.imageUrl}
                          alt=""
                          className="w-8 h-8 max-mobile:w-6 max-mobile:h-6 rounded-full object-cover"
                        />
                      ) : (
                        <div className="w-8 h-8 max-mobile:w-6 max-mobile:h-6 rounded-full bg-neutral-500 flex items-center justify-center">
                          <span className="text-xs max-mobile:text-[10px] text-white">
                            {notificationType === NotificationType.INVITE_TEAM
                              ? 'T'
                              : notificationType ===
                                  NotificationType.INVITE_DISCUSSION
                                ? 'D'
                                : notificationType ===
                                    NotificationType.BOOSTING_SPACE
                                  ? 'B'
                                  : notificationType ===
                                      NotificationType.CONNECT_NETWORK
                                    ? 'C'
                                    : 'N'}
                          </span>
                        </div>
                      )}

                      <div className="flex flex-col flex-1">
                        {content.title && content.title.trim() !== '' && (
                          <div className="font-semibold text-white text-sm/[20px] max-mobile:text-xs/[16px]">
                            {content.title}
                          </div>
                        )}
                        {content.description &&
                          content.description.trim() !== '' && (
                            <div className="font-medium text-neutral-300 text-[12px] max-mobile:text-[10px]">
                              {content.description}
                            </div>
                          )}
                      </div>
                    </div>

                    <div className="flex max-mobile:flex-row max-mobile:justify-between max-mobile:items-center max-mobile:w-full flex-col items-end gap-1 ml-2 max-mobile:ml-0">
                      <div className="flex items-center gap-2">
                        <NotificationReadStatus
                          notificationId={notification.id}
                          isRead={notification.read}
                          onStatusChange={() => refetch()}
                        />
                        <NotificationDropdown
                          notificationId={notification.id}
                          onDismiss={() => refetch()}
                        />
                      </div>
                      <div className="font-medium text-neutral-500 text-[10px] max-mobile:text-[8px]">
                        {new Date(
                          notification.created_at * 1000,
                        ).toLocaleString()}
                      </div>
                    </div>
                  </div>
                </div>
              );
            })
            .filter(Boolean)}{' '}
          {/* Filter out null values */}
          {/* Loading indicator for fetching more */}
          {isFetchingNextPage && (
            <div className="flex justify-center py-4">
              <div className="text-white text-sm">
                Loading more notifications...
              </div>
            </div>
          )}
          {/* Infinite scroll trigger */}
          {hasNextPage && !isLoading && !isFetchingNextPage && (
            <div ref={ref} className="h-10" />
          )}
          {/* End message */}
          {!hasNextPage && notifications.length > 0 && (
            <div className="flex justify-center py-4">
              <div className="text-neutral-500 text-sm">
                You've reached the end
              </div>
            </div>
          )}
        </div>
      ) : (
        <div className="flex flex-row w-full h-fit justify-center items-center px-[16px] max-mobile:px-[12px] py-[20px] max-mobile:py-[16px] border border-gray-500 rounded-[8px] font-medium text-base max-mobile:text-sm text-gray-500">
          No notifications yet
        </div>
      )}
    </div>
  );
}

function FilterBar({
  filterType,
  setFilterType,
}: {
  filterType: NotificationsFilter;
  setFilterType: (type: NotificationsFilter) => void;
}) {
  const filterOptions = [
    { value: 'all' as const, label: 'All' },
    { value: NotificationType.INVITE_TEAM, label: 'Team Invites' },
    { value: NotificationType.INVITE_DISCUSSION, label: 'Discussion Invites' },
    { value: NotificationType.BOOSTING_SPACE, label: 'Space Boosts' },
    { value: NotificationType.CONNECT_NETWORK, label: 'Network Connections' },
  ];

  return (
    <div className="flex w-full max-mobile:overflow-x-auto">
      <div className="flex w-full min-w-fit border border-neutral-600 rounded-full p-1 bg-transparent">
        {filterOptions.map((option) => (
          <div
            key={option.value}
            className={`cursor-pointer flex-1 min-w-fit px-3 max-mobile:px-2 py-1.5 max-mobile:py-1 rounded-full text-sm max-mobile:text-xs font-medium transition-all duration-200 whitespace-nowrap text-center ${
              filterType === option.value
                ? 'bg-[#FCB300] text-black border border-[#FCB300]'
                : 'text-white hover:bg-neutral-700'
            }`}
            onClick={() => setFilterType(option.value)}
          >
            {option.label}
          </div>
        ))}
      </div>
    </div>
  );
}
