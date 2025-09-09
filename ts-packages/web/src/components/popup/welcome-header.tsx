import React from 'react';
import CharacterWithCircle from '@/assets/icons/character.svg';

interface WelcomeHeaderProps {
  title: string;
  description: string;
}

export const WelcomeHeader = ({ title, description }: WelcomeHeaderProps) => {
  return (
    <div className="w-full flex flex-col gap-6 items-center justify-center">
      <p className="text-create-space-label font-bold text-2xl">{title}</p>
      <CharacterWithCircle width={100} />
      <p className="text-create-space-desc text-center text-base font-medium">
        {description}
      </p>
    </div>
  );
};
