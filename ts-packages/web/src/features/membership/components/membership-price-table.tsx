import { MembershipResponse } from '../dto/membership-response';
import { PurchaseMembershipRequest } from '../dto/purchase-membership-request';
import { useMembershipsI18n } from '../i18n';
import { MembershipTier } from '../types/membership-tier';
import MembershipPriceCard from './membership-price-card';

interface MembershipPriceTableProps {
  memberships: MembershipResponse[];
  handlePurchaseMembership: (request: PurchaseMembershipRequest) => void;
}

const features: Record<MembershipTier, string[]> = {
  Free: [
    'Business Solutions',
    'Regular News Update',
    'Live Chat and Support',
    'Social Media Marketing',
  ],
  Pro: [
    'Business Solutions',
    'Regular News Update',
    'Live Chat and Support',
    'Social Media Marketing',
  ],
  Max: [
    'Business Solutions',
    'Regular News Update',
    'Live Chat and Support',
    'Social Media Marketing',
  ],
  Vip: [
    'Business Solutions',
    'Regular News Update',
    'Live Chat and Support',
    'Social Media Marketing',
  ],
};

function normalizeTier(tier: string | undefined | null): MembershipTier | null {
  if (!tier) return null;
  const t = tier.trim().toLowerCase();
  if (t === 'free' || t == 'FREE') return MembershipTier.Free;
  if (t === 'pro' || t == 'PRO') return MembershipTier.Pro;
  if (t === 'max' || t == 'MAX') return MembershipTier.Max;
  if (t === 'vip' || t === 'VIP') return MembershipTier.Vip;
  return null;
}

export function MembershipPriceTable({
  memberships,
  handlePurchaseMembership,
}: MembershipPriceTableProps) {
  const i18n = useMembershipsI18n();

  if (memberships.length === 0) {
    return (
      <div className="py-8 text-center text-gray-500 dark:text-gray-400">
        {i18n.noMemberships}
      </div>
    );
  }

  return (
    <div className="w-full h-fit bg-card-bg border-card-border px-4 py-12 mt-12.5">
      <div className="mx-auto max-w-6xl">
        <div className="text-center">
          <div className="mx-auto inline-flex items-center rounded-full bg-follow-button-bg px-3 py-1 text-xs text-follow-button-text">
            {i18n.subscribeDesc1}
          </div>
          <h2 className="mt-4 text-2xl font-semibold text-text-primary">
            {i18n.subscribeDesc2}
          </h2>
          <p className="mt-2 text-sm text-desc-text">{i18n.subscribeDesc3}</p>
        </div>
        <div
          className="
    mt-8 grid gap-3 justify-center
    grid-cols-[repeat(auto-fit,minmax(260px,max-content))]
  "
        >
          {memberships.map((p) => {
            const tier = normalizeTier(p.tier?.toString());
            const f = tier ? features[tier] : [];

            return (
              <div
                key={p.tier?.toString() ?? p.price_dollars}
                className="w-[260px]"
              >
                <MembershipPriceCard
                  title={p.tier?.toString() ?? 'Unknown'}
                  price={p.price_dollars}
                  features={f}
                  buttonEnabled
                  selected={false}
                  onClick={() =>
                    handlePurchaseMembership({
                      membership_id: p.id,
                      payment_method: null,
                      transaction_id: null,
                    })
                  }
                  onUnsubscribe={() => {}}
                />
              </div>
            );
          })}
        </div>

        <div className="mt-8 flex items-center justify-center gap-3 text-xs text-desc-text">
          <span>{i18n.subscribeInfo}</span>
        </div>
      </div>
    </div>
  );
}
