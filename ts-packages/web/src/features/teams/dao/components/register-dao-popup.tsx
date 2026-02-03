'use client';

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { XMarkIcon, CheckIcon } from '@heroicons/react/24/outline';
import { EligibleAdmin } from '../hooks/use-dao-data';

interface RegisterDaoPopupProps {
  eligibleAdmins: EligibleAdmin[];
  onRegister: (selectedAddresses: string[]) => Promise<void>;
  onCancel: () => void;
  isRegistering: boolean;
}

export function RegisterDaoPopup({
  eligibleAdmins,
  onRegister,
  onCancel,
  isRegistering,
}: RegisterDaoPopupProps) {
  const { t } = useTranslation('TeamDao');
  const [selectedAddresses, setSelectedAddresses] = useState<Set<string>>(
    new Set(),
  );

  const minRequired = 3;
  const canConfirm = selectedAddresses.size >= minRequired && !isRegistering;

  const handleToggleSelect = (address: string) => {
    const newSelected = new Set(selectedAddresses);
    if (newSelected.has(address)) {
      newSelected.delete(address);
    } else {
      newSelected.add(address);
    }
    setSelectedAddresses(newSelected);
  };

  const handleConfirm = async () => {
    if (canConfirm) {
      await onRegister(Array.from(selectedAddresses));
    }
  };

  const truncateAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60">
      <div className="bg-background text-neutral-900 dark:text-neutral-100 rounded-lg shadow-xl w-full max-w-[600px] max-h-[80vh] flex flex-col border border-neutral-200/70 dark:border-neutral-700/60">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-neutral-200/70 dark:border-neutral-700/60">
          <div>
            <h2 className="text-2xl font-bold text-text-primary">
              {t('select_admins')}
            </h2>
            <p className="text-sm text-text-secondary mt-1">
              {t('select_admins_description')}
            </p>
          </div>
          <button
            onClick={onCancel}
            disabled={isRegistering}
            className="p-2 hover:bg-neutral-100 dark:hover:bg-neutral-800 rounded-full transition-colors disabled:opacity-50"
          >
            <XMarkIcon className="w-6 h-6 text-neutral-500" />
          </button>
        </div>

        {/* Admin List */}
        <div className="flex-1 overflow-y-auto p-6">
          <div className="space-y-3">
            {eligibleAdmins.map((admin) => {
              const isSelected = selectedAddresses.has(admin.evm_address);

              return (
                <div
                  key={admin.user_id}
                  onClick={() => handleToggleSelect(admin.evm_address)}
                  className={`
                    flex items-center gap-4 p-4 rounded-lg border-2 cursor-pointer transition-all
                    ${
                      isSelected
                        ? 'border-primary bg-primary/10'
                        : 'border-neutral-200/70 dark:border-neutral-700/60 hover:border-neutral-400/50 dark:hover:border-neutral-500/50'
                    }
                    ${isRegistering ? 'opacity-50 cursor-not-allowed' : ''}
                  `}
                >
                  {/* Checkbox */}
                  <div
                    className={`
                    w-5 h-5 rounded border-2 flex items-center justify-center shrink-0
                    ${
                      isSelected
                        ? 'bg-primary border-primary'
                        : 'border-neutral-300/70 dark:border-neutral-600/70'
                    }
                  `}
                  >
                    {isSelected && <CheckIcon className="w-4 h-4 text-white" />}
                  </div>

                  {/* Avatar */}
                  <img
                    src={admin.profile_url || '/default-avatar.png'}
                    alt={admin.display_name}
                    className="w-12 h-12 rounded-full object-cover"
                  />

                  {/* Admin Info */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <p className="font-semibold text-text-primary dark:text-neutral-100 truncate">
                        {admin.display_name}
                      </p>
                      {admin.is_owner && (
                        <span className="px-2 py-0.5 bg-yellow-100/80 dark:bg-yellow-900/60 text-yellow-800 dark:text-yellow-200 text-xs font-medium rounded">
                          Owner
                        </span>
                      )}
                    </div>
                    <p className="text-sm text-neutral-500 dark:text-neutral-400 truncate">
                      @{admin.username}
                    </p>
                    <p className="text-xs font-mono text-neutral-400 dark:text-neutral-500 mt-1">
                      {truncateAddress(admin.evm_address)}
                    </p>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Footer */}
        <div className="p-6 border-t border-neutral-200/70 dark:border-neutral-700/60">
          <div className="flex items-center justify-between mb-4">
            <p className="text-sm text-neutral-500 dark:text-neutral-400">
              {t('selected_count', { count: selectedAddresses.size })}
            </p>
            {selectedAddresses.size < minRequired && (
              <p className="text-sm text-red-500">{t('min_admins_required')}</p>
            )}
          </div>

          <div className="flex gap-3">
            <button
              onClick={onCancel}
              disabled={isRegistering}
              className="flex-1 px-6 py-3 bg-neutral-300 text-neutral-900 dark:bg-neutral-800 dark:text-neutral-100 rounded-md font-medium hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {t('cancel')}
            </button>
            <button
              onClick={handleConfirm}
              disabled={!canConfirm}
              className="flex-1 px-6 py-3 bg-primary hover:bg-primary/80 text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isRegistering ? t('registering_dao') : t('confirm')}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
