'use client';

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { CheckIcon, ClipboardIcon } from '@heroicons/react/24/outline';

interface DaoInfoCardProps {
  daoAddress: string;
  explorerUrl: string | null;
}

export function DaoInfoCard({ daoAddress, explorerUrl }: DaoInfoCardProps) {
  const { t } = useTranslation('TeamDao');
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(daoAddress);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (error) {
      console.error('Failed to copy:', error);
    }
  };

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700">
      <div className="flex items-start justify-between mb-4">
        <div>
          <h3 className="text-xl font-semibold text-text-primary mb-1">
            {t('dao_address')}
          </h3>
          <p className="text-sm text-text-secondary">{t('dao_description')}</p>
        </div>
        <div className="px-3 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full text-sm font-medium">
          {t('active')}
        </div>
      </div>

      <div className="bg-gray-50 dark:bg-gray-900 rounded-md p-4 mb-4">
        <div className="flex items-center justify-between gap-3">
          <code className="text-sm font-mono text-text-primary break-all">
            {daoAddress}
          </code>
          <button
            onClick={handleCopy}
            className="shrink-0 p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors"
            title={t('copy_address')}
          >
            {copied ? (
              <CheckIcon className="w-5 h-5 text-green-600" />
            ) : (
              <ClipboardIcon className="w-5 h-5 text-text-secondary" />
            )}
          </button>
        </div>
      </div>

      {explorerUrl && (
        <a
          href={explorerUrl}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-md hover:bg-primary-dark transition-colors"
        >
          {t('view_on_explorer')}
          <svg
            className="w-4 h-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
            />
          </svg>
        </a>
      )}
    </div>
  );
}
