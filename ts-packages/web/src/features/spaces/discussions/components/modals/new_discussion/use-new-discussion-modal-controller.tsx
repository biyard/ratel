import { useState } from 'react';
import { State } from '@/types/state';
import { useNavigate } from 'react-router';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { showErrorToast } from '@/lib/toast';
import InviteMemberPopup from '../invite_member';

export class NewDiscussionModalController {
  constructor(
    public navigate: ReturnType<typeof useNavigate>,
    public popup: ReturnType<typeof usePopup>,
    public t: TFunction<'SpaceDiscussionEditor', undefined>,

    public spacePk: string,
    public discussionPk: string | null | undefined,
    public startedAt: State<number>,
    public endedAt: State<number>,
    public name: State<string>,
    public description: State<string>,
  ) {}

  handleChangeStartedAt = (startedAt: number) => {
    this.startedAt.set(startedAt);
  };

  handleChangeEndedAt = (endedAt: number) => {
    this.endedAt.set(endedAt);
  };

  handleChangeName = (name: string) => {
    this.name.set(name);
  };

  handleChangeDescription = (description: string) => {
    this.description.set(description);
  };

  handleNext = async () => {
    if (this.name.get() === '') {
      showErrorToast('Please enter a title.');
      return;
    }

    this.popup
      .open(
        <InviteMemberPopup
          spacePk={this.spacePk}
          discussionPk={this.discussionPk}
          name={this.name.get()}
          description={this.description.get()}
          startTime={this.startedAt.get()}
          endTime={this.endedAt.get()}
        />,
      )
      .withTitle(this.t('new_discussion'))
      .withoutBackdropClose();
  };

  handleClose = () => {
    this.popup.close();
  };
}

export function useNewDiscussionModalController(
  spacePk: string,
  discussionPk: string | null | undefined,

  startedAt: number,
  endedAt: number,
  name: string,
  description: string,
) {
  const navigate = useNavigate();
  const popup = usePopup();
  const { t } = useTranslation('SpaceDiscussionEditor');

  const st = useState(startedAt || 0);
  const et = useState(endedAt || 0);
  const n = useState(name || '');
  const d = useState(description || '');

  return new NewDiscussionModalController(
    navigate,
    popup,
    t,
    spacePk,
    discussionPk,

    new State(st),
    new State(et),
    new State(n),
    new State(d),
  );
}
