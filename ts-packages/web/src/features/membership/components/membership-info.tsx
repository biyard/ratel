import { useQuery } from '@tanstack/react-query';
import { getUserMembership } from '@/lib/api/ratel/me.v3';
import { Sparkles } from 'lucide-react';

export default function MembershipInfo() {
  const { data: membership, isLoading } = useQuery({
    queryKey: ['user-membership'],
    queryFn: getUserMembership,
  });

  if (isLoading || !membership) {
    return null;
  }

  const tierColors = {
    Free: 'text-text-secondary',
    Pro: 'text-blue-500',
    Max: 'text-purple-500',
    Vip: 'text-amber-500',
  };

  const tierName = membership.tier.replace('MEMBERSHIP#', '');
  const tierColor =
    tierColors[tierName as keyof typeof tierColors] || 'text-text-secondary';

  return (
    <div className="flex flex-col gap-2 p-3 rounded-lg bg-background-secondary border border-card-border">
      <div className="flex items-center gap-2">
        <Sparkles className={`w-4 h-4 ${tierColor}`} />
        <span className={`font-semibold ${tierColor}`}>{tierName}</span>
      </div>
      <div className="text-sm text-text-secondary">
        <span className="font-medium">{membership.remaining_credits}</span> /{' '}
        {membership.total_credits} credits
      </div>
    </div>
  );
}
