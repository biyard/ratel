import { MembershipResponse } from '../dto/membership-response';
import { useMembershipsI18n } from '../i18n';

interface MembershipTableProps {
  memberships: MembershipResponse[];
  onEdit: (membership: MembershipResponse) => void;
  onDelete: (membership: MembershipResponse) => void;
}

export function MembershipTable({
  memberships,
  onEdit,
  onDelete,
}: MembershipTableProps) {
  const i18n = useMembershipsI18n();

  if (memberships.length === 0) {
    return (
      <div className="py-8 text-center text-gray-500 dark:text-gray-400">
        {i18n.noMemberships}
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <thead>
          <tr className="bg-gray-100 border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
            <th className="p-3 font-semibold text-left">{i18n.tier}</th>
            <th className="p-3 font-semibold text-left">{i18n.price}</th>
            <th className="p-3 font-semibold text-left">{i18n.credits}</th>
            <th className="p-3 font-semibold text-left">{i18n.duration}</th>
            <th className="p-3 font-semibold text-left">{i18n.displayOrder}</th>
            <th className="p-3 font-semibold text-left">{i18n.status}</th>
            <th className="p-3 font-semibold text-left">{i18n.actions}</th>
          </tr>
        </thead>
        <tbody>
          {memberships.map((membership) => (
            <tr
              key={membership.id}
              className="border-b border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800/50"
            >
              <td className="p-3 font-medium">{membership.tier}</td>
              <td className="p-3">${membership.price_dollars}</td>
              <td className="p-3">{membership.credits}</td>
              <td className="p-3">
                {membership.duration_days} {i18n.days}
              </td>
              <td className="p-3">{membership.display_order}</td>
              <td className="p-3">
                <span
                  className={`px-2 py-1 rounded text-sm ${
                    membership.is_active
                      ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
                      : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300'
                  }`}
                >
                  {membership.is_active ? i18n.active : i18n.inactive}
                </span>
              </td>
              <td className="p-3">
                <div className="flex gap-2">
                  <button
                    onClick={() => onEdit(membership)}
                    className="py-1 px-3 text-sm text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
                  >
                    {i18n.edit}
                  </button>
                  <button
                    onClick={() => onDelete(membership)}
                    className="py-1 px-3 text-sm text-white bg-red-500 rounded transition-colors hover:bg-red-600"
                  >
                    {i18n.delete}
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
