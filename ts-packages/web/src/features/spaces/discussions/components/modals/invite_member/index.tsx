import { Clear } from '@/components/icons';
import SearchInput from '@/components/input/search-input';
import clsx from 'clsx';
import { useUserInfo } from '@/hooks/use-user-info';
import { useInviteMemberModalController } from './use-invite-member-modal-controller';
import { SpaceDiscussionMemberResponse } from '../../../types/space-discussion-member-response';
import { TFunction } from 'i18next';

export type InviteMemberPopupProps = {
  spacePk: string;
  discussionPk: string;
  name: string;
  description: string;
  startTime: number;
  endTime: number;
};

export default function InviteMemberPopup({
  spacePk,
  discussionPk,
  name,
  description,
  startTime,
  endTime,
}: InviteMemberPopupProps) {
  const ctrl = useInviteMemberModalController(
    spacePk,
    discussionPk,
    name,
    description,
    startTime,
    endTime,
  );
  const { data: me } = useUserInfo();

  const ensureMe = (list: SpaceDiscussionMemberResponse[]) => {
    if (!me) return list;
    return list.some((u) => u.user_pk === me.pk)
      ? list
      : [
          ...list,
          {
            user_pk: me.pk,
            author_display_name:
              me.nickname != undefined && me.nickname !== ''
                ? me.nickname
                : me.email,
            author_profile_url: me.profile_url,
            author_username:
              me.nickname != undefined && me.nickname !== ''
                ? me.nickname
                : me.email,
          } as SpaceDiscussionMemberResponse,
        ];
  };

  const setValue = async (
    value: string,
    isEnter: boolean,
  ): Promise<SpaceDiscussionMemberResponse[] | void> => {
    const currentSelected = ctrl?.selectedUsers?.get?.() ?? [];
    const nextSelected = ensureMe(currentSelected);
    return await ctrl.handleSearchValue(nextSelected, value, isEnter);
  };

  const selectedUsers = ctrl?.selectedUsers?.get?.() ?? [];
  const errorList: boolean[] = ctrl?.isError?.get?.() ?? [];
  const searchValue = ctrl?.searchValue?.get?.() ?? '';

  return (
    <div className="flex flex-col min-h-[300px] w-[900px] max-w-[900px] max-tablet:!w-full max-tablet:!max-w-full gap-5">
      <div className="flex flex-col w-full">
        <div className="font-bold text-[15px]/[28px] text-modal-label-text">
          Email, Username, or Phone Number
        </div>
        <div className="flex flex-col w-full max-mobile:max-h-[350px] max-mobile:overflow-y-auto">
          <div className="mt-[10px]">
            <SearchInput
              value={searchValue}
              placeholder={
                'Enter email, username, or phone number (ex: john@example.com or john or 01012345678)'
              }
              setValue={async (value) => {
                await setValue(value, false);
              }}
              onenter={async () => {
                await setValue(ctrl?.searchValue?.get?.() ?? '', true);
              }}
            />
          </div>

          <div className="flex flex-col w-full gap-[10px] mt-[10px]">
            <div className="flex flex-wrap gap-1">
              {selectedUsers.map((user, index) => {
                const isErr = Boolean(errorList[index]);
                return (
                  <SelectedUserInfo
                    key={user.user_pk}
                    username={user.author_username}
                    isError={isErr}
                    onremove={() => ctrl.handleRemoveMember(index)}
                  />
                );
              })}
            </div>
          </div>
        </div>
      </div>

      <InviteMemberButton
        t={ctrl.t}
        isError={(ctrl?.errorCount?.get?.() ?? 0) !== 0}
        onclick={async () => {
          const selected = ensureMe(selectedUsers);
          await ctrl.handleSend(selected);
        }}
      />
    </div>
  );
}

function InviteMemberButton({
  t,
  isError,
  onclick,
}: {
  t: TFunction<'SpaceDiscussionEditor', undefined>;
  isError: boolean;
  onclick: () => void | Promise<void>;
}) {
  const containerClass = clsx(
    'flex flex-row w-full justify-center items-center my-[15px] py-[15px] rounded-lg font-bold text-[#000203] text-base',
    isError
      ? 'cursor-not-allowed bg-neutral-500 hover:opacity-60'
      : 'cursor-pointer bg-primary hover:opacity-60',
  );
  return (
    <div className="flex flex-col w-full">
      <div
        className={containerClass}
        onClick={() => {
          if (!isError) void onclick();
        }}
      >
        {t('send')}
      </div>

      {isError && (
        <div className="font-semibold text-base text-red-400">
          {t('invite_warning')}
        </div>
      )}
    </div>
  );
}

function SelectedUserInfo({
  username,
  isError,
  onremove,
}: {
  username: string;
  isError: boolean;
  onremove: () => void | Promise<void>;
}) {
  const containerClass = clsx(
    'flex flex-row w-fit gap-1 justify-start items-center bg-primary rounded-[100px] px-[12px] py-[2px]',
    isError ? 'border-[3px] border-[#ff0000]' : '',
  );

  return (
    <div className={containerClass}>
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
