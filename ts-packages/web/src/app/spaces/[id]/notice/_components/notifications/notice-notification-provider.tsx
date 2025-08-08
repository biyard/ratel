'use client';

import React, { createContext, useContext, useState, ReactNode } from 'react';
import NoticeNotification, {
  NoticeNotificationData,
} from './notice-notification';

interface NoticeNotificationContextType {
  showSuccessNotification: (
    rewardAmount: number,
    penaltyCount?: number,
  ) => void;
  showFailedNotification: () => void;
}

const NoticeNotificationContext = createContext<
  NoticeNotificationContextType | undefined
>(undefined);

interface NoticeNotificationProviderProps {
  children: ReactNode;
}

export function NoticeNotificationProvider({
  children,
}: NoticeNotificationProviderProps) {
  const [notification, setNotification] =
    useState<NoticeNotificationData | null>(null);

  const showSuccessNotification = (
    rewardAmount: number,
    penaltyCount?: number,
  ) => {
    const displayText =
      penaltyCount && penaltyCount > 0
        ? `Coin Earned! (${penaltyCount}x penalty applied)`
        : 'Coin Earned! View it in your profile.';

    setNotification({
      type: 'success',
      title: `+ ${rewardAmount.toLocaleString()} P`,
      body: displayText,
      rewardAmount,
    });
  };

  const showFailedNotification = () => {
    setNotification({
      type: 'failed',
      title: 'X 0.5 Penalty',
      body: 'Each wrong answer cuts your reward in half!',
    });
  };

  const closeNotification = () => {
    setNotification(null);
  };

  return (
    <NoticeNotificationContext.Provider
      value={{
        showSuccessNotification,
        showFailedNotification,
      }}
    >
      {children}
      {notification && (
        <NoticeNotification
          notification={notification}
          onClose={closeNotification}
        />
      )}
    </NoticeNotificationContext.Provider>
  );
}

export function useNoticeNotification() {
  const context = useContext(NoticeNotificationContext);
  if (context === undefined) {
    throw new Error(
      'useNoticeNotification must be used within a NoticeNotificationProvider',
    );
  }
  return context;
}
