'use client';

import dynamic from 'next/dynamic';
import { Background, Character } from './components/sprint-league';

const Base = dynamic(() => import('./components/sprint-league'), {
  ssr: false,
});
export default function SprintLeaguePage() {
  /*
    TODO: Fetch users from API or accept as props
    const { data: users } = useSprintLeagueUsers(); // hypothetical hook
    if (!users?.length) {
      return <div>Loading users...</div>;
    }  
  */
  const user = [
    {
      name: '이준석',
      alias: 'lee-jun',
    },
    {
      name: '김문수',
      alias: 'kim-moon',
    },
    {
      name: '이재명',
      alias: 'lee-jae',
    },
  ];
  return (
    <div className="w-full h-[calc(100vh-var(--header-height))] flex justify-center items-center">
      <Base>
        <Background names={user.map((u) => u.name)} />
        {user.map((user, index) => (
          <Character key={user.alias} alias={user.alias} rank={index + 1} />
        ))}
      </Base>
    </div>
  );
}
