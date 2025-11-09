import { Clear } from '@/components/icons';
import SearchInput from '@/components/input/search-input';
import clsx from 'clsx';
import { useInviteMemberModalController } from './use-invite-member-modal-controller';
import { TFunction } from 'i18next';

export type InviteMemberPopupProps = {
  spacePk: string;
};

export default function InviteMemberPopup({ spacePk }: InviteMemberPopupProps) {
  const ctrl = useInviteMemberModalController(spacePk);

  const errorList: boolean[] = ctrl?.isError?.get?.() ?? [];

  return (
    <div className="flex flex-col min-h-[300px] w-[900px] max-w-[900px] max-tablet:!w-full max-tablet:!max-w-full gap-5">
      <div className="flex flex-col w-full">
        <div className="font-bold text-[15px]/[28px] text-modal-label-text">
          {ctrl.t('email_label')}
        </div>
        <div className="flex flex-col w-full max-mobile:max-h-[350px] max-mobile:overflow-y-auto">
          <div className="mt-[10px]">
            <SearchInput
              value={ctrl.searchValue.get()}
              placeholder={ctrl.t('email_hint')}
              setValue={(value) => ctrl.searchValue.set(value)}
              onenter={ctrl.handleSearchValue}
            />
          </div>

          <div className="flex flex-col w-full gap-[10px] mt-[10px]">
            <div className="flex flex-wrap gap-1">
              {ctrl.members.map((user, index) => {
                const isErr = Boolean(errorList[index]);
                return (
                  <SelectedUserInfo
                    key={user.user_pk}
                    username={user.username}
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
        onclick={ctrl.handleSend}
      />
    </div>
  );
}

function InviteMemberButton({
  t,
  isError,
  onclick,
}: {
  t: TFunction<'SpaceMemberEditor', undefined>;
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
        <div className="text-base font-semibold text-red-400">
          {t('invalid_user_error')}
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
