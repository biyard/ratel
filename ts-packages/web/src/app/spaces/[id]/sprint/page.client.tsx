'use client';

import dynamic from 'next/dynamic';
import { Background, Character } from './components/sprint-league';

const Base = dynamic(() => import('./components/sprint-league'), {
  ssr: false,
});
export default function SprintLeaguePage() {
  const user = [
    {
      name: '이준석',
      jsonPath: '/images/lee-jun.json',
    },
    {
      name: '김문수',
      jsonPath: '/images/kim-moon.json',
    },
    {
      name: '이재명',
      jsonPath: '/images/lee-jae.json',
    },
  ];
  return (
    //FIXME: Use height value, not fixed header height
    <div className="w-full h-[calc(100vh-120px)] flex justify-center items-center">
      <Base>
        <Background names={user.map((u) => u.name)} />
        {user.map((user, index) => (
          <Character key={index} jsonPath={user.jsonPath} rank={index + 1} />
        ))}
      </Base>
    </div>
  );
}
