'use client';
import React from 'react';
import MyInfo from './_components/tab/my-info';
import MySettings from './_components/tab/my-settings';
import { useTranslations } from 'next-intl';

export default function MyProfilePage() {
  const t = useTranslations('Settings');
  const tabs = [t('my_info'), t('settings')] as const;
  const [activeIndex, setActiveIndex] = React.useState(0);

  return (
    <div className="w-full flex flex-col gap-6">
      <div className="w-full max-w-[800px] mx-auto px-4">
        <div
          role="tablist"
          aria-label="Profile tabs"
          className="flex text-sm font-bold text-foreground-muted"
        >
          {tabs.map((label, idx) => (
            <button
              key={label}
              role="tab"
              id={`tab-${idx}`}
              aria-controls={`panel-${idx}`}
              aria-selected={activeIndex === idx}
              onClick={() => setActiveIndex(idx)}
              className={
                'group flex-1 flex flex-col items-center justify-center py-3 transition-colors text-tab-label' +
                (activeIndex === idx
                  ? 'text-tab-label/80'
                  : 'hover:text-tab-label/80')
              }
              type="button"
            >
              <span>{label}</span>
              <div
                className="
                  mt-2 h-0.5 w-[29px] rounded bg-primary
                  opacity-0 transition-opacity duration-200
                  group-aria-selected:opacity-100
                "
              />
            </button>
          ))}
        </div>
      </div>

      <div className="w-full px-4 md:px-0">
        <section
          id="panel-0"
          role="tabpanel"
          aria-labelledby="tab-0"
          hidden={activeIndex !== 0}
          className="w-full max-w-[800px] mx-auto"
        >
          <MyInfo />
        </section>

        {/* <section
          id="panel-1"
          role="tabpanel"
          aria-labelledby="tab-1"
          hidden={activeIndex !== 1}
          className="w-full max-w-[800px] mx-auto text-foreground-muted"
        >
          {t('my_inventory_coming_soon')}
        </section> */}

        <section
          id="panel-2"
          role="tabpanel"
          aria-labelledby="tab-2"
          hidden={activeIndex !== 1}
          className="w-full max-w-[800px] mx-auto"
        >
          <MySettings />
        </section>
      </div>
    </div>
  );
}
