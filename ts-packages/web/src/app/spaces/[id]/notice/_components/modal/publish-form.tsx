'use client';

import { useState } from 'react';
import { PublishingScope } from '@/lib/api/models/notice';
import Lock from '@/assets/icons/lock.svg?react';
import Internet from '@/assets/icons/internet.svg?react';
import { useTranslation } from 'react-i18next';

export interface PublishFormProps {
  onPublish: (scope: PublishingScope) => void;
  currentScope?: PublishingScope;
}

export default function PublishForm({
  onPublish,
  currentScope,
}: PublishFormProps) {
  const { t } = useTranslation('NoticeSpace');
  const [selectedScope, setSelectedScope] = useState<PublishingScope>(
    currentScope === PublishingScope.Public
      ? PublishingScope.Public
      : currentScope || PublishingScope.Private,
  );

  // Check if this space was previously published as public
  const wasPublishedAsPublic = currentScope === PublishingScope.Public;

  const handleSubmit = () => {
    onPublish(selectedScope);
  };

  return (
    <div className="w-[450px] h-[425px] max-tablet:w-full flex flex-col justify-between">
      <div>
        {/* Header */}
        <div className="mb-6">
          <div className="font-bold text-text-primary text-[24px]">
            {t('publish_space')}
          </div>
        </div>

        {/* Publishing Options */}
        <div className="flex flex-col gap-4">
          {/* Private Option */}
          <div
            className={`p-4 rounded-lg border-2 transition-colors ${
              wasPublishedAsPublic
                ? 'border-neutral-600 bg-neutral-900 cursor-not-allowed opacity-50'
                : selectedScope === PublishingScope.Private
                  ? 'border-primary bg-neutral-800 light:bg-primary/10 cursor-pointer'
                  : 'border-neutral-700 light:border-neutral-300 light:hover:border-neutral-300 hover:border-neutral-600 cursor-pointer'
            }`}
            onClick={() =>
              !wasPublishedAsPublic && setSelectedScope(PublishingScope.Private)
            }
          >
            <div className="flex items-center gap-3">
              <Lock width={24} height={24} className="text-neutral-400" />
              <div className="flex-1">
                <div
                  className={`font-bold text-lg ${
                    wasPublishedAsPublic
                      ? 'text-neutral-500'
                      : 'text-text-primary'
                  }`}
                >
                  {t('private_publish')}
                </div>
                <div
                  className={`text-sm mt-1 ${
                    wasPublishedAsPublic
                      ? 'text-neutral-600'
                      : 'text-neutral-400 light:text-[#525252]'
                  }`}
                >
                  {wasPublishedAsPublic
                    ? t('private_publish_desc_1')
                    : t('private_publish_desc_2')}
                </div>
              </div>
              <div
                className={`w-5 h-5 rounded-full border-2 ${
                  selectedScope === PublishingScope.Private &&
                  !wasPublishedAsPublic
                    ? 'border-primary bg-primary'
                    : wasPublishedAsPublic
                      ? 'border-neutral-600'
                      : 'border-neutral-500'
                }`}
              >
                {selectedScope === PublishingScope.Private &&
                  !wasPublishedAsPublic && (
                    <div className="w-full h-full rounded-full bg-primary flex items-center justify-center">
                      <div className="w-2 h-2 rounded-full bg-black"></div>
                    </div>
                  )}
              </div>
            </div>
          </div>

          {/* Public Option */}
          <div
            className={`p-4 rounded-lg border-2 cursor-pointer transition-colors ${
              selectedScope === PublishingScope.Public
                ? 'border-primary bg-neutral-800 light:bg-primary/10'
                : 'border-neutral-700 hover:border-neutral-600 light:border-neutral-300 light:hover:border-neutral-300'
            }`}
            onClick={() => setSelectedScope(PublishingScope.Public)}
          >
            <div className="flex items-center gap-3">
              <Internet width={24} height={24} className="text-neutral-400" />
              <div className="flex-1">
                <div className="font-bold text-text-primary text-lg">
                  {t('public_publish')}
                </div>
                <div className="text-neutral-400 light:text-[#525252] text-sm mt-1">
                  {t('public_publish_desc')}
                </div>
              </div>
              <div
                className={`w-5 h-5 rounded-full border-2 ${
                  selectedScope === PublishingScope.Public
                    ? 'border-primary bg-primary'
                    : 'border-neutral-500'
                }`}
              >
                {selectedScope === PublishingScope.Public && (
                  <div className="w-full h-full rounded-full bg-primary flex items-center justify-center">
                    <div className="w-2 h-2 rounded-full bg-black"></div>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Publish Button */}
      <div className="flex justify-end">
        <button
          onClick={handleSubmit}
          className="w-full py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px] hover:bg-primary/90 transition-colors"
        >
          {t('publish')}
        </button>
      </div>
    </div>
  );
}
