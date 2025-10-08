import CharacterWithCircle from '@/assets/icons/character.svg?react';

interface WelcomeHeaderProps {
  title: string;
  description: string;
}

export const WelcomeHeader = ({ title, description }: WelcomeHeaderProps) => {
  return (
    <div className="w-full flex flex-col gap-6 items-center justify-center">
      <p className="text-modal-label-text font-bold text-2xl">{title}</p>
      <CharacterWithCircle width={100} />
      <p className="text-desc-text text-center text-base font-medium">
        {description}
      </p>
    </div>
  );
};
