import BlackBox from '@/app/(social)/_components/black-box';
import FirstActive from '@/assets/icons/progress/1_active.svg';
import FirstInActive from '@/assets/icons/progress/1_inactive.svg';

import SecondActive from '@/assets/icons/progress/2_active.svg';
import SecondInActive from '@/assets/icons/progress/2_inactive.svg';

import ThirdActive from '@/assets/icons/progress/3_active.svg';
import ThirdInActive from '@/assets/icons/progress/3_inactive.svg';

import FourthActive from '@/assets/icons/progress/4_active.svg';
import FourthInActive from '@/assets/icons/progress/4_inactive.svg';

import FifthActive from '@/assets/icons/progress/5_active.svg';
import FifthInActive from '@/assets/icons/progress/5_inactive.svg';

import SixthActive from '@/assets/icons/progress/6_active.svg';
import SixthInActive from '@/assets/icons/progress/6_inactive.svg';

import SeventhActive from '@/assets/icons/progress/7_active.svg';
import SeventhInActive from '@/assets/icons/progress/7_inactive.svg';

export default function SpaceCouponProgress({
  progress,
}: {
  progress: number;
}) {
  return (
    <BlackBox>
      <div className="w-full flex flex-row gap-2.5">
        {progress > 0 ? <FirstActive /> : <FirstInActive />}
        {progress > 1 ? <SecondActive /> : <SecondInActive />}
        {progress > 2 ? <ThirdActive /> : <ThirdInActive />}
        {progress > 3 ? <FourthActive /> : <FourthInActive />}
        {progress > 4 ? <FifthActive /> : <FifthInActive />}
        {progress > 5 ? <SixthActive /> : <SixthInActive />}
        {progress > 6 ? <SeventhActive /> : <SeventhInActive />}
      </div>
    </BlackBox>
  );
}
