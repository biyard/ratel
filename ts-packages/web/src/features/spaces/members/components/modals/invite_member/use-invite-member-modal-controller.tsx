import { useNavigate } from 'react-router';
import { usePopup } from '@/lib/contexts/popup-service';
import { State } from '@/types/state';
import { useMemo, useState } from 'react';

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

    public members: InvitationMemberResponse[],
    public newMembers: State<InvitationMemberResponse[]>,
    public removedMembers: State<string[]>,
    public isError: State<boolean[]>,
    public searchValue: State<string>,
    public errorCount: State<number>,

    public upsertInvitation: ReturnType<typeof useUpsertInvitationMutation>,
    public t: TFunction<'SpaceMemberEditor', undefined>,
  ) {}

  handleSearchValue = async () => {
    const identifiers = this.searchValue
      .get()
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

        if (!data) {
          showErrorToast(this.t('invalid_user'));
          return;
        }
        const idx = this.removedMembers.get().indexOf(data.pk);
        if (idx !== -1) {
          this.removedMembers.set(
            this.removedMembers.get().filter((pk) => pk !== data.pk),
          );

          return;
        }

        const exists = this.members.some((u) => u.user_pk === data.pk);

        if (exists) {
          // FIXME: error toast
          showErrorToast(this.t('invalid_user'));
        }

        const user = {
          user_pk: data.pk,
          display_name: data.nickname,
          profile_url: data.profile_url,
          username: data.username,
          email: data.email,

          authorized: false,
        };

        this.newMembers.set([...this.newMembers.get(), user]);
      } catch (err) {
        logger.error('failed to search user with error: ', err);
        showErrorToast(this.t('search_user_failed'));
      }
    }

    this.searchValue.set('');
  };

  handleRemoveMember = async (index: number) => {
    const prevUsers = this.members[index];
    this.removedMembers.set([
      ...this.removedMembers.get(),
      prevUsers.user_pk as string,
    ]);
  };

  handleSend = async () => {
    if (this.searchValue.get() !== '') {
      await this.handleSearchValue();
    }

    const newUserPks: string[] = this.newMembers
      .get()
      .map((u) => u.user_pk)
      .filter((v): v is string => typeof v === 'string' && v.length > 0);

    const removedUserPks = this.removedMembers.get();

    try {
      await this.upsertInvitation.mutateAsync({
        spacePk: this.spacePk,
        new_user_pks: newUserPks,
        removed_user_pks: removedUserPks,
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
  const { t } = useTranslation('SpaceMemberEditor');

  const newMembers = useState([]);
  const removedMembers = useState([]);
  const isError = useState([]);
  const searchValue = useState('');
  const errorCount = useState(0);

  const members = useMemo(() => {
    const allMembers = [...member.members]
      .filter((member) => removedMembers[0].indexOf(member.user_pk) === -1)
      .concat(newMembers[0]);

    return allMembers;
  }, [member, newMembers, removedMembers]);

  const upsertInvitation = useUpsertInvitationMutation();

  return new InviteMemberModalController(
    navigate,
    popup,
    spacePk,

    members,
    new State(newMembers),
    new State(removedMembers),
    new State(isError),
    new State(searchValue),
    new State(errorCount),

    upsertInvitation,

    t,
  );
}
