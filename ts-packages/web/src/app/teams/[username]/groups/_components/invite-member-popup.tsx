'use client';

import SelectBox from '@/components/selectbox/selectbox';
import { Group, TotalUser } from '@/lib/api/models/user';
import React, { useState } from 'react';
// import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import { Clear } from '@/components/icons';
import SearchInput from '@/components/input/search-input';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { checkEmailRequest } from '@/lib/api/models/group';
import clsx from 'clsx';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';

export default function InviteMemberPopup({
  team_id,
  groups,
  onclick,
}: {
  team_id: number;
  groups: Group[];
  onclick: (group_id: number, users: number[]) => void;
}) {
  const { post, get } = useApiCall();
  const [groupIndex, setGroupIndex] = useState(0);
  const [selectedGroup, setSelectedGroup] = useState(groups[0]);

  const [selectedUsers, setSelectedUsers] = useState<TotalUser[]>([]);
  const [isError, setIsError] = useState<boolean[]>([]);
  const [searchValue, setSearchValue] = useState('');
  const [errorCount, setErrorCount] = useState(0);

  const setValue = async (value: string, isEnter: boolean) => {
    if (value.includes(',') || isEnter) {
      const emails = value
        .split(',')
        .map((email) => email.trim())
        .filter((email) => email !== '');

      for (const email of emails) {
        const isValidEmail = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
        if (checkString(email) || !isValidEmail) {
          continue;
        }

        const data = await get(ratelApi.users.getUserByEmail(email));
        const result = await post(
          ratelApi.groups.check_email(team_id, selectedGroup.id),
          checkEmailRequest(email),
        );

        if (data) {
          const exists = selectedUsers.some((u) => u.id === data.id);
          if (!exists) {
            const value: boolean = !result ? true : false;
            setSelectedUsers((prev) => [...prev, data]);
            setIsError((prevErrors) => [...prevErrors, value]);

            if (value) {
              setErrorCount((prev) => Math.max(prev + 1, 0));
            }
          }
        }
      }

      setSearchValue('');
    } else {
      setSearchValue(value);
    }
  };

  return (
    <div className="flex flex-col w-[900px] min-h-[400px] max-w-[900px] min-w-[400px] max-mobile:!w-full max-mobile:!max-w-full gap-5">
      <div className="flex flex-col w-full gap-[10px]">
        <div className="font-bold text-[15px]/[28px] text-neutral-400">
          Select the group
        </div>
        <SelectBox
          groups={groups}
          groupIndex={groupIndex}
          setGroupIndex={setGroupIndex}
          selectedGroup={selectedGroup}
          setSelectedGroup={setSelectedGroup}
        />
      </div>

      <div className="flex flex-col w-full">
        <div className="font-bold text-[15px]/[28px] text-neutral-400">
          Input User Email
        </div>
        <div className="mt-[10px]">
          <SearchInput
            value={searchValue}
            placeholder={
              'Input the value (ex: example@example.com, example2@example.com, example3@example.com)'
            }
            setValue={async (value) => {
              setValue(value, false);
            }}
            onenter={async () => {
              setValue(searchValue, true);
            }}
          />
        </div>
      </div>

      <div className="flex flex-col w-full gap-[10px]">
        <div className="flex flex-wrap gap-1">
          {selectedUsers.map((user, index) => {
            return (
              <SelectedUserInfo
                key={user.id}
                username={user.nickname}
                isError={isError[index]}
                onremove={() => {
                  setSelectedUsers((prevUsers) => {
                    const newUsers = [...prevUsers];
                    newUsers.splice(index, 1);
                    return newUsers;
                  });

                  setIsError((prevErrors) => {
                    const newErrors = [...prevErrors];
                    const v = newErrors.splice(index, 1)[0];
                    logger.debug('value: ', v);

                    const newErrorCount = newErrors.filter(
                      (e) => e === true,
                    ).length;
                    setErrorCount(newErrorCount);

                    return newErrors;
                  });
                }}
              />
            );
          })}
        </div>
      </div>

      <InviteMemberButton
        isError={errorCount != 0}
        onclick={() => {
          const selectedUserIds = selectedUsers.map((user) => user.id);
          onclick(selectedGroup.id, selectedUserIds);
        }}
      />
    </div>
  );
}

function InviteMemberButton({
  isError,
  onclick,
}: {
  isError: boolean;
  onclick: () => void;
}) {
  const containerClass = clsx(
    'flex flex-row w-full justify-center items-center my-[15px] py-[15px] rounded-lg font-bold text-[#000203] text-base',
    isError ? 'cursor-not-allowed bg-neutral-500' : 'cursor-pointer bg-primary',
  );
  return (
    <div className="flex flex-col w-full">
      <div
        className={containerClass}
        onClick={() => {
          if (!isError) {
            onclick();
          }
        }}
      >
        Send
      </div>

      {isError && (
        <div className="font-semibold text-base text-red-400">
          The user does not exist or already exists in the group. Please check
          the email again.
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
  onremove: () => void;
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
        className="w-6 h-6 cursor-pointer"
        onClick={onremove}
      />
    </div>
  );
}
