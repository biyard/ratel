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
        <div className="w-12 h-12 rounded-full border-b-2 animate-spin border-primary"></div>
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
    <div className="flex flex-col gap-6 p-6 mx-auto w-full max-w-4xl">
      <h1 className="text-2xl font-bold text-text-primary">{t.title}</h1>

      {/* Current Membership Card */}
      <div className="p-6 rounded-lg border bg-card-bg border-card-border">
        <h2 className="mb-4 text-xl font-semibold text-text-primary">
          {t.current_plan}
        </h2>

        <div className="flex flex-col gap-4">
          <div className="flex gap-3 items-center">
            <Sparkles className={`w-6 h-6 ${tierColor}`} />
            <div>
              <div className="text-sm text-text-secondary">{t.tier}</div>
              <div className={`text-lg font-bold ${tierColor}`}>{tierName}</div>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <div className="text-sm text-text-secondary">
                {t.total_credits}
              </div>
              <div className="text-lg font-semibold text-text-primary">
                {membership.total_credits}
              </div>
            </div>

            <div>
              <div className="text-sm text-text-secondary">
                {t.remaining_credits}
              </div>
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
            <div className="p-3 rounded border bg-background-secondary border-amber-500/30">
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
      <div className="p-6 rounded-lg border bg-card-bg border-card-border">
        <h2 className="mb-4 text-xl font-semibold text-text-primary">
          {t.purchase_history}
        </h2>

        {isLoadingHistory ? (
          <div className="flex justify-center py-8">
            <div className="w-8 h-8 rounded-full border-b-2 animate-spin border-primary"></div>
          </div>
        ) : purchaseHistory && purchaseHistory.items.length > 0 ? (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-card-border">
                  <th className="py-3 px-2 text-sm font-semibold text-left text-text-secondary">
                    {t.transaction_type}
                  </th>
                  <th className="py-3 px-2 text-sm font-semibold text-left text-text-secondary">
                    {t.amount}
                  </th>
                  <th className="py-3 px-2 text-sm font-semibold text-left text-text-secondary">
                    {t.payment_id}
                  </th>
                  <th className="py-3 px-2 text-sm font-semibold text-left text-text-secondary">
                    {t.date}
                  </th>
                </tr>
              </thead>
              <tbody>
                {purchaseHistory.items.map((item, index) => (
                  <tr
                    key={index}
                    className="border-b last:border-0 border-card-border"
                  >
                    <td className="py-3 px-2 text-sm text-text-primary">
                      {item.tx_type}
                    </td>
                    <td className="py-3 px-2 text-sm text-text-primary">
                      ${item.amount}
                    </td>
                    <td className="py-3 px-2 font-mono text-xs text-text-secondary">
                      {item.payment_id}
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
          <div className="py-8 text-center text-text-secondary">
            {t.no_purchases}
          </div>
        )}
      </div>
    </div>
  );
}
