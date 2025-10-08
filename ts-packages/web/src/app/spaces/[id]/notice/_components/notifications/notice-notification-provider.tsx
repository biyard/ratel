'use client';

import {
  createContext,
  useContext,
  useState,
  ReactNode,
  useCallback,
} from 'react';
import NoticeNotification, {
  NoticeNotificationData,
} from './notice-notification';
import { useTranslation } from 'react-i18next';

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

  const { t } = useTranslation('NoticeSpace');

  const showSuccessNotification = useCallback(
    (rewardAmount: number, penaltyCount?: number) => {
      const displayText =
        penaltyCount && penaltyCount > 0
          ? t('notif.success_body_with_penalty', { count: penaltyCount })
          : t('notif.success_body_no_penalty');

      setNotification({
        type: 'success',
        title: t('notif.success_title', {
          amount: rewardAmount.toLocaleString(),
        }),
        body: displayText,
        rewardAmount,
      });
    },
    [t],
  );

  const showFailedNotification = useCallback(() => {
    setNotification({
      type: 'failed',
      title: t('notif.failed_title'),
      body: t('notif.failed_body'),
    });
  }, [t]);

  const closeNotification = useCallback(() => {
    setNotification(null);
  }, []);

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
