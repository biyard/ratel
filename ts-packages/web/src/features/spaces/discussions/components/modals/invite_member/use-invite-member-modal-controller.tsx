import { useNavigate } from 'react-router';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import useDiscussionMemberSpace from '../../../hooks/use-discussion-member-space';
import { State } from '@/types/state';
import { SpaceDiscussionMemberResponse } from '../../../types/space-discussion-member-response';
import { useState } from 'react';
import { User } from '@/lib/api/ratel/auth.v3';
import { checkString } from '@/lib/string-filter-utils';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { useCreateDiscussionMutation } from '../../../hooks/use-create-discussion-mutation';

export class InviteMemberModalController {
  constructor(
    public navigate: ReturnType<typeof useNavigate>,
    public popup: ReturnType<typeof usePopup>,
    public t: TFunction<'SpaceDiscussionEditor', undefined>,

    public spacePk: string,
    public discussionPk: string | null | undefined,
    public startedAt: number,
    public endedAt: number,
    public name: string,
    public description: string,

    public selectedUsers: State<SpaceDiscussionMemberResponse[]>,
    public isError: State<boolean[]>,
    public searchValue: State<string>,
    public errorCount: State<number>,
    public get,

    public createDiscussion: ReturnType<typeof useCreateDiscussionMutation>,
  ) {}

  handleSearchValue = async (
    nextSelected: SpaceDiscussionMemberResponse[],
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
          let data: User | null = null;
          if (isEmail) {
            data = await this.get(ratelApi.users.getUserByEmail(input));
          } else if (isPhone) {
            data = await this.get(ratelApi.users.getUserByPhoneNumber(input));
          } else {
            data = await this.get(ratelApi.users.getUserByUsername(input));
          }
          if (data) {
            const exists = nextSelected.some((u) => u.user_pk === data.pk);
            if (!exists) {
              const user = {
                user_pk: data.pk,
                author_display_name:
                  data.display_name != undefined && data.display_name != ''
                    ? data.display_name
                    : data.email,
                author_profile_url: data.profile_url,
                author_username:
                  data.display_name != undefined && data.display_name != ''
                    ? data.display_name
                    : data.email,
              };
              ns = [...ns, user];
            }
          } else {
            showErrorToast(this.t('invalid_user'));
          }
        } catch (err) {
          logger.error('failed to search user with error: ', err);
          showErrorToast(this.t('failed_search_user'));
        }
      }
      this.selectedUsers.set(nextSelected);
      this.searchValue.set('');
      return nextSelected;
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

  handleSend = async (selected: SpaceDiscussionMemberResponse[]) => {
    const userPks: string[] = selected
      .map((u) => u.user_pk)
      .filter((v): v is string => typeof v === 'string' && v.length > 0);

    try {
      await this.createDiscussion.mutateAsync({
        spacePk: this.spacePk,
        started_at: this.startedAt,
        ended_at: this.endedAt,
        name: this.name,
        description: this.description,
        user_ids: userPks,
      });

      showSuccessToast('Success to create discussion');
    } catch {
      showErrorToast('Failed to create discussion');
    } finally {
      this.popup.close();
    }
  };

  handleClose = () => {
    this.popup.close();
  };
}

export function useInviteMemberModalController(
  spacePk: string,
  discussionPk: string | null | undefined,
  name: string,
  description: string,
  startedAt: number,
  endedAt: number,
) {
  const { data: member } = useDiscussionMemberSpace(
    spacePk,
    discussionPk ?? spacePk,
  );
  const { get } = useApiCall();

  const navigate = useNavigate();
  const popup = usePopup();
  const { t } = useTranslation('SpaceDiscussionEditor');

  const members = useState(member.members || []);
  const isError = useState([]);
  const searchValue = useState('');
  const errorCount = useState(0);

  const createDiscussion = useCreateDiscussionMutation();

  return new InviteMemberModalController(
    navigate,
    popup,
    t,
    spacePk,
    discussionPk,

    startedAt,
    endedAt,
    name,
    description,

    new State(members),
    new State(isError),
    new State(searchValue),
    new State(errorCount),
    get,

    createDiscussion,
  );
}
