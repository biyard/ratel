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
import { UserResponse } from '@/lib/api/ratel/users.v3';
import { usePollResponseMutation } from '../../hooks/use-poll-response-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export class SpacePollViewerController {
  constructor(
    public space: Space,
    public poll: Poll,
    public t: TFunction<'SpaceSurvey', undefined>,
    public answers: State<Record<number, SurveyAnswer>>,
    public user: UserResponse | null,
    public popup: ReturnType<typeof usePopup>,
    public submitPollResponse: ReturnType<typeof usePollResponseMutation>,
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

  handleSubmit = () => {
    try {
      this.submitPollResponse.mutate({
        spacePk: this.space.pk,
        pollSk: this.poll.sk,
        answers: Object.values(this.answers.get()),
      });

      showSuccessToast(this.t('success_submit_answer'));
    } catch (err) {
      logger.error('submit answer failed: ', err);
      showErrorToast(this.t('failed_submit_answer'));
    }
  };

  handleLogin = () => {
    this.popup
      .open(<LoginModal />)
      .withTitle(this.t('Nav:join_the_movement'))
      .withoutBackdropClose();
  };
}

export function useSpacePollViewerController(spacePk, pollPk) {
  // Fetching data from remote
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { data: user } = useUserInfo();

  // mutations
  const usePollResponse = usePollResponseMutation();

  const { t } = useTranslation('SpaceSurvey');
  const popup = usePopup();
  const answers = useState<Record<number, SurveyAnswer>>(
    poll?.myResponse.reduce(
      (acc, answer, idx) => {
        acc[idx] = answer;
        return acc;
      },
      {} as Record<number, SurveyAnswer>,
    ) || {},
  );

  return new SpacePollViewerController(
    space,
    poll,
    t,
    new State(answers),
    user,
    popup,
    usePollResponse,
  );
}
