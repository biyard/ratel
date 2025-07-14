import Game from './game';

export default function SprintLeaguePage() {
  return (
    <div className="w-full h-[calc(100vh-var(--header-height))] flex justify-center items-center">
      <Game />
    </div>
  );
}
