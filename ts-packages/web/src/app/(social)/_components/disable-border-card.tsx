export interface DisableBorderCardProps {
  children: React.ReactNode;
}

/**
 * @deprecated use Card from '@/components/card' instead
 * <Card variant="secondary" />
 */

export default function DisableBorderCard({
  children,
}: DisableBorderCardProps) {
  return (
    <div
      className={`flex flex-col w-full justify-start items-start bg-card-bg border border-card-border rounded-[10px] px-4 py-5`}
    >
      {children}
    </div>
  );
}
