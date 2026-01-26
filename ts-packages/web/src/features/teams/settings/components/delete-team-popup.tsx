'use client';

import clsx from 'clsx';
import { TeamSettingsI18n } from '../i18n';

interface DeleteTeamPopupProps {
  onConfirm: () => void;
  onCancel: () => void;
  i18n: TeamSettingsI18n;
}

export default function DeleteTeamPopup({
  onConfirm,
  onCancel,
  i18n,
}: DeleteTeamPopupProps) {
  return (
    <div className="flex flex-col w-[480px] max-w-full gap-6 p-6">
      <div className="flex flex-col gap-2">
        <div className="text-lg font-bold text-text-primary text-center">
          {i18n.delete_team_title}
        </div>
        <div className="text-sm text-text-secondary leading-6">
          {i18n.delete_team_description}
        </div>
      </div>

      <div className="flex items-center justify-end gap-3">
        <button
          type="button"
          onClick={onCancel}
          className={clsx(
            'h-10 px-4 rounded-lg border border-neutral-300 text-text-primary',
            'hover:bg-neutral-100 disabled:opacity-50 disabled:cursor-not-allowed',
          )}
          data-pw="delete-team-cancel-button"
        >
          {i18n.cancel}
        </button>
        <button
          type="button"
          onClick={onConfirm}
          className={clsx(
            'h-10 px-4 rounded-lg bg-red-600 text-white font-semibold',
            'hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed',
          )}
          data-pw="delete-team-confirm-button"
        >
          {i18n.confirm}
        </button>
      </div>
    </div>
  );
}
