import { useMembershipPageController } from './use-membership-page-controller';
import { Sparkles } from 'lucide-react';

export default function MembershipPage() {
  const {
    t,
    membership,
    purchaseHistory,
    isLoadingMembership,
    isLoadingHistory,
    formatDate,
    formatTier,
  } = useMembershipPageController();

  if (isLoadingMembership) {
    return (
      <div className="flex justify-center items-center min-h-screen">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (!membership) {
    return null;
  }

  const tierColors = {
    Free: 'text-text-secondary',
    Pro: 'text-blue-500',
    Max: 'text-purple-500',
    Vip: 'text-amber-500',
  };

  const tierName = formatTier(membership.tier);
  const tierColor =
    tierColors[tierName as keyof typeof tierColors] || 'text-text-secondary';

  return (
    <div className="flex flex-col gap-6 p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold text-text-primary">{t.title}</h1>

      {/* Current Membership Card */}
      <div className="p-6 rounded-lg bg-card-bg border border-card-border">
        <h2 className="text-xl font-semibold text-text-primary mb-4">
          {t.current_plan}
        </h2>

        <div className="flex flex-col gap-4">
          <div className="flex items-center gap-3">
            <Sparkles className={`w-6 h-6 ${tierColor}`} />
            <div>
              <div className="text-sm text-text-secondary">{t.tier}</div>
              <div className={`text-lg font-bold ${tierColor}`}>
                {tierName}
              </div>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <div className="text-sm text-text-secondary">{t.total_credits}</div>
              <div className="text-lg font-semibold text-text-primary">
                {membership.total_credits}
              </div>
            </div>

            <div>
              <div className="text-sm text-text-secondary">{t.remaining_credits}</div>
              <div className="text-lg font-semibold text-text-primary">
                {membership.remaining_credits}
              </div>
            </div>
          </div>

          <div>
            <div className="text-sm text-text-secondary">{t.expiration}</div>
            <div className="text-lg font-semibold text-text-primary">
              {formatDate(membership.expired_at)}
            </div>
          </div>

          {membership.next_membership && (
            <div className="p-3 rounded bg-background-secondary border border-amber-500/30">
              <div className="text-sm font-semibold text-amber-500">
                {t.scheduled_downgrade}
              </div>
              <div className="text-sm text-text-secondary">
                {t.next_membership}: {formatTier(membership.next_membership)}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Purchase History */}
      <div className="p-6 rounded-lg bg-card-bg border border-card-border">
        <h2 className="text-xl font-semibold text-text-primary mb-4">
          {t.purchase_history}
        </h2>

        {isLoadingHistory ? (
          <div className="flex justify-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
          </div>
        ) : purchaseHistory && purchaseHistory.items.length > 0 ? (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-card-border">
                  <th className="text-left py-3 px-2 text-sm font-semibold text-text-secondary">
                    {t.transaction_type}
                  </th>
                  <th className="text-left py-3 px-2 text-sm font-semibold text-text-secondary">
                    {t.amount}
                  </th>
                  <th className="text-left py-3 px-2 text-sm font-semibold text-text-secondary">
                    {t.payment_id}
                  </th>
                  <th className="text-left py-3 px-2 text-sm font-semibold text-text-secondary">
                    {t.date}
                  </th>
                </tr>
              </thead>
              <tbody>
                {purchaseHistory.items.map((item, index) => (
                  <tr
                    key={index}
                    className="border-b border-card-border last:border-0"
                  >
                    <td className="py-3 px-2 text-sm text-text-primary">
                      {item.tx_type}
                    </td>
                    <td className="py-3 px-2 text-sm text-text-primary">
                      ${item.amount}
                    </td>
                    <td className="py-3 px-2 text-sm text-text-secondary font-mono text-xs">
                      {item.payment_id.slice(0, 16)}...
                    </td>
                    <td className="py-3 px-2 text-sm text-text-secondary">
                      {formatDate(item.created_at)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className="text-center py-8 text-text-secondary">
            {t.no_purchases}
          </div>
        )}
      </div>
    </div>
  );
}
