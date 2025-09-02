'use client';
import React from 'react';
import MyInfo from './_components/tab/my-info';
import MySettings from './_components/tab/my-settings';

export default function MyProfilePage() {
  const tabs = ['My Info', 'My Inventory', 'Settings'] as const;
  const [activeIndex, setActiveIndex] = React.useState<number>(0);

  return (
    <div className="w-full flex flex-col gap-6">
      {/* Top tabs section with active indicator */}
      <div className="w-full max-w-[800px] mx-auto px-4">
        <div className="relative">
          <div className="flex text-sm font-bold text-neutral-400 relative">
            {tabs.map((label, idx) => (
              <button
                key={label}
                type="button"
                className={`
                  flex-1 py-3 text-center transition-colors relative
                  ${activeIndex === idx ? 'text-neutral-400' : 'hover:text-neutral-600'}
                `}
                onClick={() => setActiveIndex(idx)}
              >
                {label}
                {activeIndex === idx && (
                  <div className="absolute bottom-0 left-1/2 transform -translate-x-1/2 h-[2px] bg-primary rounded-lg w-10" />
                )}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* Tab content */}
      <div className="w-full px-4 md:px-0">
        {activeIndex === 0 && <MyInfo />}
        {activeIndex === 1 && (
          <div className="w-full max-w-[800px] mx-auto text-neutral-300">
            My Inventory content coming soon.
          </div>
        )}
        {activeIndex === 2 && (
          <div className="w-full max-w-[800px] mx-auto">
            <MySettings />
          </div>
        )}
      </div>
    </div>
  );
}