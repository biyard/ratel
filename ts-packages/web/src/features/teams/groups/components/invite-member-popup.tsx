import SelectBox from '@/components/selectbox/selectbox';
import { Clear } from '@/components/icons';
import SearchInput from '@/components/input/search-input';
import clsx from 'clsx';
import { useTranslation } from 'react-i18next';
import { TeamGroup } from '@/features/teams/types/team_group';
import { useInviteMemberPopupController } from './use-invite-member-popup-controller';

export default function InviteMemberPopup({
  teamPk,
  username,
  groups,
  onClose,
}: {
  teamPk: string;
  username: string;
  groups: TeamGroup[];
  onClose: () => void;
}) {
  const ctrl = useInviteMemberPopupController({
    teamPk,
    username,
    groups,
    onClose,
  });
  const { t } = useTranslation('Team');

  return (
    <div className="flex flex-col w-tablet min-h-[400px] max-w-tablet min-w-[400px] max-mobile:w-full! max-mobile:max-w-full! gap-5">
      <div className="flex flex-col w-full gap-[10px]">
        <div className="font-bold text-[15px]/[28px] text-modal-label-text">
          {t('select_group')}
        </div>
        <SelectBox
          groups={ctrl.convertedGroups}
          groupIndex={ctrl.groupIndex}
          setGroupIndex={(index) =>
            ctrl.handleGroupChange(index, ctrl.convertedGroups[index])
          }
          selectedGroup={ctrl.selectedGroup}
          setSelectedGroup={(group) => {
            const index = ctrl.convertedGroups.findIndex(
              (g) => g.id === group.id,
            );
            if (index !== -1) {
              ctrl.handleGroupChange(index, group);
            }
          }}
        />
      </div>

      <div className="flex flex-col w-full">
        <div className="font-bold text-[15px]/[28px] text-modal-label-text">
          {t('email_label')}
        </div>
        <div className="mt-2.5">
          <SearchInput
            data-testid="invite-member-search-input"
            value={ctrl.searchValue}
            placeholder={t('email_hint')}
            setValue={ctrl.handleSearchValueChange}
            onenter={async () => {
              await ctrl.handleSearchUser(ctrl.searchValue);
            }}
          />
        </div>
        {ctrl.isSearching && (
          <div className="text-sm text-gray-400 mt-2">Searching...</div>
        )}
      </div>

      <div className="flex flex-col w-full gap-[10px]">
        <div className="flex flex-wrap gap-1">
          {ctrl.selectedUsers.map((user) => {
            return (
              <SelectedUserInfo
                key={user.pk}
                username={user.nickname}
                onremove={() => {
                  ctrl.handleRemoveUser(user.pk);
                }}
              />
            );
          })}
        </div>
      </div>

      <InviteMemberButton
        disabled={ctrl.selectedUsers.length === 0 || ctrl.isSubmitting}
        onclick={ctrl.handleInvite}
        i18n={t}
      />
    </div>
  );
}

function InviteMemberButton({
  disabled,
  onclick,
  i18n,
}: {
  disabled: boolean;
  onclick: () => void;
  i18n: (key: string) => string;
}) {
  const containerClass = clsx(
    'flex flex-row w-full justify-center items-center my-[15px] py-[15px] rounded-lg font-bold text-[#000203] text-base',
    disabled
      ? 'cursor-not-allowed bg-neutral-500'
      : 'cursor-pointer bg-primary',
  );
  return (
    <div className="flex flex-col w-full">
      <div
        data-testid="send-invite-button"
        className={containerClass}
        onClick={() => {
          if (!disabled) {
            onclick();
          }
        }}
      >
        {i18n('send')}
      </div>
    </div>
  );
}

function SelectedUserInfo({
  username,
  onremove,
}: {
  username: string;
  onremove: () => void;
}) {
  return (
    <div className="flex flex-row w-fit gap-1 justify-start items-center bg-primary rounded-[100px] px-[12px] py-[2px]">
      <div className="font-medium text-neutral-900 text-[15px]/[24px]">
        {username}
      </div>
      <Clear
        width={24}
        height={24}
        className="w-6 h-6 cursor-pointer [&>path]:stroke-neutral-800"
        onClick={onremove}
      />
    </div>
  );
}
