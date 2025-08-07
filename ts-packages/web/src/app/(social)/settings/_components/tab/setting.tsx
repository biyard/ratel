'use client';

import SwitchButton from '@/components/switch-button';
import BlackBox from '../black-box';
import ContentBox from '../content-box';
import { useTheme } from '@/app/_providers/ThemeProvider';

export default function Setting() {
  const { isToggled, toggleTheme } = useTheme();
  return (
    <div className="flex flex-col w-full justify-start gap-6.25">
      <BlackBox title={'Appearance'}>
        {/* <ContentBox canClicked={false} onClick={() => {}}>
          <div className="flex flex-row w-full justify-start items-center py-[16.5px]">
            <div className="font-bold text-white text-base/[20px]">
              Language
            </div>
          </div>
        </ContentBox> */}
        <ContentBox canClicked={false} onClick={() => {}}>
          <div className="flex flex-row w-full justify-between items-center py-[16.5px]">
            <div className="font-bold text-white text-base/[20px]">
              Dark Theme
            </div>

            <div className="flex flex-row w-fit justify-start items-center gap-2.5">
              <div className="font-semibold text-sm/[20px] text-primary">
                {isToggled ? 'On' : 'Off'}
              </div>
              <SwitchButton
                value={isToggled}
                onChange={() => {
                  toggleTheme();
                }}
                color={'bg-[#1890ff]'}
              />
            </div>
          </div>
        </ContentBox>
      </BlackBox>
    </div>
  );
}
