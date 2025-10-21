import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';
import usePollSpace from '../../hooks/use-poll-space';
import { useTranslation } from 'react-i18next';
import { Poll } from '../../types/poll';
import { SurveyAnswer } from '../../types/poll-question';
import { logger } from '@/lib/logger';
import { State } from '@/types/state';
import { useState } from 'react';
import { useUserInfo } from '@/hooks/use-user-info';
import { LoginModal } from '@/components/popup/login-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { UserResponse } from '@/lib/api/ratel/me.v3';

export class SpacePollViewerController {
  constructor(
    public space: Space,
    public poll: Poll,
    public t: TFunction<'SpaceSurvey', undefined>,
    public answers: State<Record<number, SurveyAnswer>>,
    public user: UserResponse | null,
    public popup: ReturnType<typeof usePopup>,
  ) {}

  handleUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    logger.debug(
      `handleUpdateAnswer called for questionIdx ${questionIdx}`,
      answer,
    );
    const currentAnswers = this.answers.get();
    currentAnswers[questionIdx] = answer;
    this.answers.set({ ...currentAnswers });
  };

  handleSubmit = () => {};

  handleLogin = () => {
    this.popup
      .open(<LoginModal />)
      .withTitle(this.t('Nav:join_the_movement'))
      .withoutBackdropClose();
  };
}

export function useSpacePollViewerController(spacePk, pollPk) {
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { data: user } = useUserInfo();

  const { t } = useTranslation('SpaceSurvey');
  const popup = usePopup();
  const answers = useState<Record<number, SurveyAnswer>>({});

  return new SpacePollViewerController(
    space,
    poll,
    t,
    new State(answers),
    user,
    popup,
  );
}
