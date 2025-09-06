'use client';

import React, { useEffect, useState } from 'react';
import Rewards from '@/assets/icons/rewards.svg';
import HexDown from '@/assets/icons/hex-down.svg';

export interface NoticeNotificationData {
  type: 'success' | 'failed';
  title: string;
  body: string;
  rewardAmount?: number;
}

interface NoticeNotificationProps {
  notification: NoticeNotificationData;
  onClose: () => void;
}

export default function NoticeNotification({
  notification,
  onClose,
}: NoticeNotificationProps) {
  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    // Fade out after 2 seconds
    const timer = setTimeout(() => {
      setIsVisible(false);
      // Remove notification after fade animation
      setTimeout(() => {
        onClose();
      }, 300);
    }, 2000);

    return () => clearTimeout(timer);
  }, [onClose]);

  const bgColor = notification.type === 'success' ? '#22C55E' : '#EF4444';

  return (
    <div
      className={`fixed top-32 right-[270px] z-50 min-w-[280px] max-w-[250px] rounded-lg shadow-lg transition-all duration-300 ${
        isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-4'
      }`}
      style={{ backgroundColor: bgColor }}
    >
      <div className="p-4">
        {/* Title */}
        <div className="flex items-center gap-2 mb-2">
          {notification.type === 'success' ? (
            <Rewards
              width={20}
              height={20}
              className="[&>path]:stroke-white [&>circle]:stroke-white"
            />
          ) : (
            <HexDown width={20} height={20} className="[&>path]:stroke-white" />
          )}
          <span className="text-white font-semibold text-[14px]">
            {notification.type === 'success'
              ? `+ ${notification.rewardAmount?.toLocaleString()} P`
              : 'X 0.5 Penalty'}
          </span>
        </div>

        {/* Body */}
        <div className="text-white text-[13px] leading-relaxed">
          {notification.body}
        </div>
      </div>
    </div>
  );
}
