'use client';

import React from 'react';
import { X } from 'lucide-react';

interface ConfirmModalProps {
  title: string;
  description: string;
  emphasisText?: string;
  confirmText: string;
  cancelText?: string;
  show: boolean;
  onClose: () => void;
  onConfirm: () => void;
}

export const SpaceConfirmModal = ({
  title,
  description,
  emphasisText,
  confirmText,
  cancelText = 'Cancel',
  show,
  onClose,
  onConfirm,
}: ConfirmModalProps) => {
  if (!show) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-[#1A1A1A] text-white rounded-xl p-6 w-[90%] max-w-md shadow-lg relative">
        <button
          className="absolute top-4 right-4 text-neutral-400 hover:text-white transition"
          onClick={onClose}
        >
          <X className="w-5 h-5" />
        </button>

        <h2 className="text-lg font-semibold text-center">{title}</h2>

        <p className="text-sm text-neutral-300 text-center mt-3">
          {description}{' '}
          {emphasisText && (
            <span className="font-semibold text-white">{emphasisText}</span>
          )}
        </p>

        <div className="flex justify-end items-center gap-4 mt-6">
          <button
            onClick={onClose}
            className="text-sm font-medium text-neutral-400 hover:text-white transition"
          >
            {cancelText}
          </button>
          <button
            onClick={onConfirm}
            className="bg-yellow-400 hover:bg-yellow-500 text-black font-semibold text-sm px-4 py-2 rounded-md transition"
          >
            {confirmText}
          </button>
        </div>
      </div>
    </div>
  );
};
