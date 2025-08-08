'use client';

import SwitchButton from '@/components/switch-button';
import BlackBox from '../black-box';
import ContentBox from '../content-box';
import { useTheme } from 'next-themes';

export default function Setting() {
  const { theme, setTheme } = useTheme();

  return (
    <div className="flex flex-col w-full justify-start gap-6">
      <BlackBox title="Appearance">
        <ContentBox canClicked={false} onClick={() => {}}>
          <div className="flex flex-row w-full justify-between items-center py-4">
            <div
              className={`font-bold text-base ${theme === 'dark' ? 'text-white' : 'text-neutral-600'}`}
            >
              Dark Theme
            </div>

            <div className="flex flex-row w-fit justify-start items-center gap-2.5">
              <div className="font-semibold text-sm text-primary">
                {theme === 'dark' ? 'On' : 'Off'}
              </div>
              <SwitchButton
                value={theme === 'dark'}
                onChange={() => {
                  setTheme(theme === 'dark' ? 'light' : 'dark');
                }}
                color="bg-[#1890ff]"
              />
            </div>
          </div>
        </ContentBox>
      </BlackBox>
    </div>
  );
}
