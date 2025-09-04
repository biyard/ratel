'use client';

import { TotalUser } from '@/lib/api/models/user';
import React, { useState } from 'react';
// import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import { Clear } from '@/components/icons';
import SearchInput from '@/components/input/search-input';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import clsx from 'clsx';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { DiscussionInfo } from '../../types';
import { useTranslations } from 'next-intl';

export default function InviteMemberPopup({
  title,
  description,
  startTime,
  endTime,
  users,
  onadd,
}: {
  title: string;
  description: string;
  startTime: number;
  endTime: number;
  reminderEnabled: boolean;
  users: TotalUser[];
  onadd: (discussion: DiscussionInfo) => void;
}) {
  const t = useTranslations('DeliberationSpace');
  const { get } = useApiCall();

  const [selectedUsers, setSelectedUsers] = useState<TotalUser[]>(users);
  const [isError, setIsError] = useState<boolean[]>([]);
  const [searchValue, setSearchValue] = useState('');
  const [errorCount, setErrorCount] = useState(0);

  const setValue = async (value: string, isEnter: boolean) => {
    if (value.includes(',') || isEnter) {
      const identifiers = value
        .split(',')
        .map((v) => v.trim())
        .filter((v) => v !== '');

      for (const input of identifiers) {
        if (checkString(input)) continue;

        const isEmail = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(input);
        const isPhone = /^\+?[0-9]\d{7,14}$/.test(input);

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        let data: any = null;

        try {
          if (isEmail) {
            data = await get(ratelApi.users.getUserByEmail(input));
          } else if (isPhone) {
            data = await get(ratelApi.users.getUserByPhoneNumber(input));
          } else {
            data = await get(ratelApi.users.getUserByUsername(input));
          }

          if (data) {
            const exists = selectedUsers.some((u) => u.id === data.id);
            if (!exists) {
              setSelectedUsers((prev) => [...prev, data]);
            }
          } else {
            showErrorToast(t('invalid_user'));
          }
        } catch (err) {
          logger.error('failed to search user with error: ', err);
          showErrorToast(t('failed_search_user'));
        }
      }

      setSearchValue('');
    } else {
      setSearchValue(value);
    }
  };

  return (
    <div className="flex flex-col w-[900px] min-h-[300px] max-w-[900px] min-w-[400px] max-mobile:!w-full max-mobile:!max-w-full gap-5">
      <div className="flex flex-col w-full">
        <div className="font-bold text-[15px]/[28px] text-neutral-400">
          {t('invite_label')}
        </div>
        <div className="mt-[10px]">
          <SearchInput
            value={searchValue}
            placeholder={t('invite_hint')}
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
          onadd({
            started_at: Math.floor(startTime),
            ended_at: Math.floor(endTime),
            name: title,
            description,
            participants: selectedUsers,
          });
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
  const t = useTranslations('DeliberationSpace');
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
          if (!isError) {
            onclick();
          }
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
        className="w-6 h-6 cursor-pointer [&>path]:stroke-neutral-800"
        onClick={onremove}
      />
    </div>
  );
}
