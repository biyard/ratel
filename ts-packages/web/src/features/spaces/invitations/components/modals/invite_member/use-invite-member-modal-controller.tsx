import { useNavigate } from 'react-router';
import { usePopup } from '@/lib/contexts/popup-service';
import { State } from '@/types/state';
import { useEffect, useState } from 'react';

import useInvitationMember from '../../../hooks/use-invitation';
import { InvitationMemberResponse } from '../../../types/invitation-member-response';
import { checkString } from '@/lib/string-filter-utils';
import {
  findUserByEmail,
  findUserByPhoneNumber,
  findUserByUsername,
  UserDetailResponse,
} from '@/lib/api/ratel/users.v3';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { useUpsertInvitationMutation } from '../../../hooks/use-upsert-invitation-mutation';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';

export class InviteMemberModalController {
  constructor(
    public navigate: ReturnType<typeof useNavigate>,
    public popup: ReturnType<typeof usePopup>,
    public spacePk: string,

    public selectedUsers: State<InvitationMemberResponse[]>,
    public isError: State<boolean[]>,
    public searchValue: State<string>,
    public errorCount: State<number>,

    public upsertInvitation: ReturnType<typeof useUpsertInvitationMutation>,
    public t: TFunction<'SpaceInvitationEditor', undefined>,
  ) {}

  handleSearchValue = async (
    nextSelected: InvitationMemberResponse[],
    value: string,
    isEnter: boolean,
  ) => {
    let ns = nextSelected;
    if (value.includes(',') || isEnter) {
      const identifiers = value
        .split(',')
        .map((v) => v.trim())
        .filter((v) => v !== '');
      for (const input of identifiers) {
        if (checkString(input)) continue;
        const isEmail = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(input);
        const isPhone = /^\+?[0-9]\d{7,14}$/.test(input);
        try {
          let data: UserDetailResponse | null = null;
          if (isEmail) {
            data = await findUserByEmail(input);
          } else if (isPhone) {
            data = await findUserByUsername(input);
          } else {
            data = await findUserByPhoneNumber(input);
          }
          if (data) {
            const exists = nextSelected.some((u) => u.user_pk === data.pk);
            if (!exists) {
              const user = {
                user_pk: data.pk,
                display_name: data.nickname,
                profile_url: data.profile_url,
                username: data.username,
                email: data.email,

                authorized: false,
              };
              ns = [...ns, user];
            }
          } else {
            showErrorToast(this.t('invalid_user'));
          }
        } catch (err) {
          logger.error('failed to search user with error: ', err);
          showErrorToast(this.t('search_user_failed'));
        }
      }
      this.selectedUsers.set(ns);
      this.searchValue.set('');
      return ns;
    } else {
      this.searchValue.set(value);
    }
  };

  handleRemoveMember = async (index: number) => {
    {
      const prevUsers = this.selectedUsers.get();
      const newUsers = prevUsers.slice();
      newUsers.splice(index, 1);
      this.selectedUsers.set(newUsers);
    }

    {
      const prevErrors = this.isError.get();
      const newErrors = prevErrors.slice();
      const removed = newErrors.splice(index, 1)[0];
      logger.debug('value: ', removed);

      const newErrorCount = newErrors.filter(Boolean).length;
      this.errorCount.set(newErrorCount);

      this.isError.set(newErrors);
    }
  };

  handleSend = async (selected: InvitationMemberResponse[]) => {
    const userPks: string[] = selected
      .map((u) => u.user_pk)
      .filter((v): v is string => typeof v === 'string' && v.length > 0);

    try {
      await this.upsertInvitation.mutateAsync({
        spacePk: this.spacePk,
        user_pks: userPks,
      });

      showSuccessToast(this.t('success_invitation_users'));
    } catch {
      showErrorToast(this.t('failed_invitation_users'));
    } finally {
      this.popup.close();
    }
  };

  handleClose = () => {
    this.popup.close();
  };
}

export function useInviteMemberModalController(spacePk: string) {
  const { data: member } = useInvitationMember(spacePk);

  const navigate = useNavigate();
  const popup = usePopup();
  const { t } = useTranslation('SpaceInvitationEditor');

  const members = useState(member.members || []);
  const isError = useState([]);
  const searchValue = useState('');
  const errorCount = useState(0);

  useEffect(() => {
    members[1](member.members ?? []);
  }, [member.members]);

  const upsertInvitation = useUpsertInvitationMutation();

  return new InviteMemberModalController(
    navigate,
    popup,
    spacePk,

    new State(members),
    new State(isError),
    new State(searchValue),
    new State(errorCount),

    upsertInvitation,

    t,
  );
}
