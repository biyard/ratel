'use client';
import { usePopup } from '@/lib/contexts/popup-service';
import React, { useEffect } from 'react';
import RemoveIcon from '@/assets/icons/remove.svg';

export const PopupZone = () => {
  const popup = usePopup();
  const popupData = popup.popup;

  useEffect(() => {
    if (popupData) {
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.body.style.overflow = 'auto';
    };
  }, [popupData]);

  if (!popupData) return null;

  const {
    id = 'popup-zone',
    title,
    description,
    content,
    closable = true,
    overflow = false,
    backdropClosable = true,
  } = popupData;

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-labelledby={title ? 'popup-title' : undefined}
      aria-describedby={description ? 'popup-description' : undefined}
      className="fixed top-0 left-0 w-screen h-screen bg-popup-background backdrop-blur-[10px] flex justify-center items-center z-[101] bg-no-s"
      onClick={() => {
        if (backdropClosable) {
          popup.close();
        }
      }}
    >
      <div
        className={`relative rounded-[20px] p-[25px] min-w-[300px] max-mobile:!w-full max-mobile:!mx-[20px] bg-popover text-foreground ${overflow ? 'overflow-visible' : 'overflow-hidden'}`}
        style={{
          boxShadow: '0px 0px 100px rgba(255, 206, 71, 0.25)',
        }}
        onClick={(e) => e.stopPropagation()}
      >
        {closable && (
          <button
            className="group absolute top-[25px] right-[25px] rounded-[4px] cursor-pointer bg-transparent hover:bg-secondary"
            onClick={() => popup.close()}
          >
            <RemoveIcon className="[&>path]:stroke-neutral-80 group-hover:[&>path]:stroke-text-primary" />
          </button>
        )}

        <div
          id={id}
          className="flex flex-col items-center justify-center gap-[25px]"
        >
          {title && (
            <div
              id="popup-title"
              className="text-[20px] font-bold text-foreground"
            >
              {title}
            </div>
          )}
          {description && (
            <div id="popup-description" className="text-foreground-muted">
              {description}
            </div>
          )}
          {content}
        </div>
      </div>
    </div>
  );
};
