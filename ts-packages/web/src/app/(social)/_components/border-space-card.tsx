
export interface BorderSpaceCardProps {
  children: React.ReactNode;
}

export default function BorderSpaceCard({ children }: BorderSpaceCardProps) {
  return (
    <div
      className={`flex flex-col w-full justify-start items-start bg-card-bg-secondary border border-card-border rounded-[10px] px-4 py-5`}
    >
      {children}
    </div>
  );
}
