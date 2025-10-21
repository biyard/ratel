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

export class SpacePollViewerController {
  constructor(
    public space: Space,
    public poll: Poll,
    public t: TFunction<'SpaceSurvey', undefined>,
    public answers: State<Record<number, SurveyAnswer>>,
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
}

export function useSpacePollViewerController(spacePk, pollPk) {
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { t } = useTranslation('SpaceSurvey');
  // FIXME: This should be my current answers
  const answers = useState<Record<number, SurveyAnswer>>({});

  return new SpacePollViewerController(space, poll, t, new State(answers));
}
