'use client';

import React from 'react';
import MyInfo from './_components/tab/my-info';
import MySettings from './_components/tab/my-settings';

export default function MyProfilePage() {
  const tabs = ['My Info', 'My Inventory', 'Settings'] as const;
  const [activeIndex, setActiveIndex] = React.useState<number>(0);

  // Refs to measure tab button positions
  const tabRefs = React.useRef<HTMLButtonElement[]>([]);
  const indicatorRef = React.useRef<HTMLDivElement>(null);
  const [indicatorLeft, setIndicatorLeft] = React.useState<number>(0);

  const updateIndicator = React.useCallback(() => {
    const btn = tabRefs.current[activeIndex];
    const indicator = indicatorRef.current;
    if (!btn || !indicator) return;
    const btnRect = btn.getBoundingClientRect();
    const parentRect = btn.parentElement?.getBoundingClientRect();
    if (!parentRect) return;
    const indicatorWidth = indicator.offsetWidth; // uses w-10
    const centerX = btnRect.left - parentRect.left + btnRect.width / 2;
    const left = centerX - indicatorWidth / 2;
    setIndicatorLeft(left);
  }, [activeIndex]);

  React.useLayoutEffect(() => {
    updateIndicator();
  }, [updateIndicator]);

  React.useEffect(() => {
    const onResize = () => updateIndicator();
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, [updateIndicator]);

  return (
    <div className="w-full flex flex-col gap-6">

      {/* Top tabs section with active indicator */}
      <div className="w-full max-w-[800px] mx-auto px-4">
        <div className="relative">
          <div className="flex text-sm font-bold text-neutral-400">
            {tabs.map((label, idx) => (
              <button
                key={label}
                type="button"
                ref={(el) => {
                  if (el) tabRefs.current[idx] = el;
                }}
                className={
                  'flex-1 py-3 text-center transition-colors ' +
                  (activeIndex === idx ? 'text-neutral-100' : 'hover:text-neutral-200')
                }
                onClick={() => setActiveIndex(idx)}
              >
                {label}
              </button>
            ))}
          </div>
          {/* Active underline indicator centered under tab label */}
          <div
            ref={indicatorRef}
            className="absolute bottom-0 h-[2px] bg-primary rounded-lg w-10 transition-all duration-300 ease-out"
            style={{ left: indicatorLeft }}
          />
        </div>
      </div>

      {/* Tab content */}
      <div className="w-full px-4 md:px-0">
        {activeIndex === 0 && (
          <MyInfo />
        )}
        {activeIndex === 1 && (
          <div className="w-full max-w-[800px] mx-auto text-neutral-300">
            {/* TODO: Replace with MyInventory component when available */}
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
