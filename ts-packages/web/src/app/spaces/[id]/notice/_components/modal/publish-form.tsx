'use client';

import React, { useState } from 'react';
import { PublishingScope } from '@/lib/api/models/notice';
import Lock from '@/assets/icons/lock.svg';
import Internet from '@/assets/icons/internet.svg';
import Clear from '@/assets/icons/clear.svg';

export interface PublishFormProps {
  onPublish: (scope: PublishingScope) => void;
  onClose: () => void;
  currentScope?: PublishingScope;
}

export default function PublishForm({
  onPublish,
  onClose,
  currentScope,
}: PublishFormProps) {
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
    <div className="w-[450px] h-[425px] flex flex-col justify-between">
      <div>
        {/* Header with title and close icon on the same line */}
        <div className="flex justify-between items-center mb-6">
          <div className="font-bold text-white text-[24px]">
            Publish this Space
          </div>
          <button
            onClick={onClose}
            className="text-neutral-500 hover:text-gray-300 transition-colors"
          >
            <Clear className="w-8 h-8" />
          </button>
        </div>

        {/* Publishing Options */}
        <div className="flex flex-col gap-4">
          {/* Private Option */}
          <div
            className={`p-4 rounded-lg border-2 transition-colors ${
              wasPublishedAsPublic
                ? 'border-neutral-600 bg-neutral-900 cursor-not-allowed opacity-50'
                : selectedScope === PublishingScope.Private
                  ? 'border-primary bg-neutral-800 cursor-pointer'
                  : 'border-neutral-700 hover:border-neutral-600 cursor-pointer'
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
                    wasPublishedAsPublic ? 'text-neutral-500' : 'text-white'
                  }`}
                >
                  Private Publish
                </div>
                <div
                  className={`text-sm mt-1 ${
                    wasPublishedAsPublic
                      ? 'text-neutral-600'
                      : 'text-neutral-400'
                  }`}
                >
                  {wasPublishedAsPublic
                    ? 'Cannot change back to private once published publicly.'
                    : 'Only your team members will be able to access this space.'}
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
                ? 'border-primary bg-neutral-800'
                : 'border-neutral-700 hover:border-neutral-600'
            }`}
            onClick={() => setSelectedScope(PublishingScope.Public)}
          >
            <div className="flex items-center gap-3">
              <Internet width={24} height={24} className="text-neutral-400" />
              <div className="flex-1">
                <div className="font-bold text-white text-lg">
                  Public Publish
                </div>
                <div className="text-neutral-400 text-sm mt-1">
                  Anyone can access and participate in this space.
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
          Publish
        </button>
      </div>
    </div>
  );
}
