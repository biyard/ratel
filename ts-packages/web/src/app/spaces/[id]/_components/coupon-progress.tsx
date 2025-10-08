import DisableBorderCard from '@/app/(social)/_components/disable-border-card';
import FirstActive from '@/assets/icons/progress/1_active.svg?react';
import FirstInActive from '@/assets/icons/progress/1_inactive.svg?react';

import SecondActive from '@/assets/icons/progress/2_active.svg?react';
import SecondInActive from '@/assets/icons/progress/2_inactive.svg?react';

import ThirdActive from '@/assets/icons/progress/3_active.svg?react';
import ThirdInActive from '@/assets/icons/progress/3_inactive.svg?react';

import FourthActive from '@/assets/icons/progress/4_active.svg?react';
import FourthInActive from '@/assets/icons/progress/4_inactive.svg?react';

import FifthActive from '@/assets/icons/progress/5_active.svg?react';
import FifthInActive from '@/assets/icons/progress/5_inactive.svg?react';

import SixthActive from '@/assets/icons/progress/6_active.svg?react';
import SixthInActive from '@/assets/icons/progress/6_inactive.svg?react';

import SeventhActive from '@/assets/icons/progress/7_active.svg?react';
import SeventhInActive from '@/assets/icons/progress/7_inactive.svg?react';

export default function SpaceCouponProgress({
  progress,
}: {
  progress: number;
}) {
  return (
    <DisableBorderCard>
      <div className="w-full flex flex-row gap-2.5">
        {progress > 0 ? <FirstActive /> : <FirstInActive />}
        {progress > 1 ? <SecondActive /> : <SecondInActive />}
        {progress > 2 ? <ThirdActive /> : <ThirdInActive />}
        {progress > 3 ? <FourthActive /> : <FourthInActive />}
        {progress > 4 ? <FifthActive /> : <FifthInActive />}
        {progress > 5 ? <SixthActive /> : <SixthInActive />}
        {progress > 6 ? <SeventhActive /> : <SeventhInActive />}
      </div>
    </DisableBorderCard>
  );
}
