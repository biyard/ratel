import { TFunction } from 'i18next';
import { InvitationMemberResponse } from '../../types/invitation-member-response';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { CheckCircle2, Extra } from '@/components/icons';
import { Button } from '@/components/ui/button';

export interface InviteMemberTableProps {
  isDraft: boolean;
  inviteMembers: InvitationMemberResponse[];
  t: TFunction<'SpaceMemberEditor', undefined>;
  handleDeleteMember: (index: number) => void;
  handleSendCode: (email: string) => void;
}

export default function InviteMemberTable({
  isDraft,
  inviteMembers,
  t,
  handleDeleteMember,
  handleSendCode,
}: InviteMemberTableProps) {
  return inviteMembers.length === 0 ? (
    <div className="text-sm text-neutral-500">{t('no_invitations')}</div>
  ) : (
    <ul className="rounded-xl border border-neutral-800 light:border-card-border divide-y divide-neutral-800 light:divide-card-border overflow-hidden">
      {inviteMembers.map((m, index) => (
        <li
          key={m.user_pk}
          className="flex items-center w-full justify-between p-3"
        >
          <div className="flex items-center gap-3">
            {m.profile_url && m.profile_url !== '' ? (
              <img
                src={m.profile_url}
                alt="User Profile"
                className="w-8 h-8 rounded-full object-cover"
              />
            ) : (
              <div className="w-8 h-8 bg-neutral-500 rounded-full" />
            )}
            <div className="flex-1 min-w-0">
              <div className="text-sm font-semibold truncate">{m.username}</div>
              <div className="text-xs text-neutral-400 truncate">
                {m.email || ''}
              </div>
            </div>
          </div>

          <div className="flex flex-row w-fit gap-3">
            {!isDraft && (
              <div>
                {m.authorized ? (
                  <CheckCircle2 className="[&>path]:stroke-green-500 [&>circle]:stroke-green-500" />
                ) : (
                  <Button
                    variant="primary"
                    onClick={() => {
                      handleSendCode(m.email);
                    }}
                  >
                    {t('resend')}
                  </Button>
                )}
              </div>
            )}

            {!m.authorized && (
              <ContextMenu
                t={t}
                handleDeleteMember={() => {
                  handleDeleteMember(index);
                }}
              />
            )}
          </div>
        </li>
      ))}
    </ul>
  );
}

export function ContextMenu({
  t,
  handleDeleteMember,
}: {
  t: TFunction<'SpaceMemberEditor', undefined>;
  handleDeleteMember: () => void;
}) {
  return (
    <div>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <button
            className="p-1 hover:bg-hover rounded-full focus:outline-none transition-colors"
            aria-haspopup="true"
            aria-label="Post options for desktop"
          >
            <Extra id="menu-option" className="size-6 text-gray-400" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          align="end"
          className="w-40 border-gray-700 transition ease-out duration-100"
        >
          <DropdownMenuItem>
            <button
              aria-label="Delete Panel"
              onClick={handleDeleteMember}
              className="flex items-center w-full px-4 max-tablet:justify-start max-tablet:gap-1 max-tablet:hover:bg-transparent max-tablet:px-0 py-2 text-sm text-neutral-700 hover:bg-gray-700 hover:text-white cursor-pointer"
            >
              {t('delete')}
            </button>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
