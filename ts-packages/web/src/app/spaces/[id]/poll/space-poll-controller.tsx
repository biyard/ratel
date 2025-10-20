import { SpaceHeaderController } from '@/features/spaces/components/header/use-space-header';
import { useState } from 'react';
import { Post } from '@/lib/api/ratel/posts.v3';
import { SurveyProps } from '@/features/spaces/components/survey';
import {
  createEmptyAnswer,
  PollAnswer,
  SurveyAnswerType,
  PollQuestion,
} from '@/features/spaces/polls/types/poll-question';
import { ReportProps } from '@/features/spaces/components/report';
import { PollSpaceResponse } from '@/lib/api/ratel/poll.spaces.v3';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import { useUpdatePollSpaceMutation } from '@/features/spaces/polls/hooks/use-update-poll-mutation';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { usePollResponseMutation } from '@/features/spaces/polls/hooks/use-poll-response-mutation';
import validateSurveyAnswer from '@/features/spaces/utils/validate-survey-answer';
import { useSpaceHomeData } from '../use-space-home-data';

export interface ISpaceController
  extends SurveyProps,
    Omit<ReportProps, 'summaries'> {
  post: Post;
  space: PollSpaceResponse;
  headerCtrl: SpaceHeaderController;
  isEditMode: boolean;
  activeTab: Tab;
  onSelectTab: (tab: Tab) => void;
  onSubmitSurvey: () => Promise<void>;
}

export class PollSpaceController implements ISpaceController {
  public post: Post;
  public space: PollSpaceResponse;
  public headerCtrl: SpaceHeaderController;
  public startedAt: number;
  public endedAt: number;
  public totalResponses: number;

  isEditMode: boolean;
  isSurveyProgress: boolean = false;
  constructor(
    public t: TFunction<'PollSpace', undefined>,
    public spacePk: string,
    post: Post,
    space: PollSpaceResponse,
    headerCtrl: SpaceHeaderController,
    public activeTab: Tab,
    public onSelectTab: (tab: Tab) => void,
    public questions: PollQuestion[],
    public isAnswerModified: boolean,
    public answers: Record<number, PollAnswer>,
    public onAddQuestion: () => void,
    public onUpdateQuestion: (index: number, question: PollQuestion) => void,
    public onDeleteQuestion: (index: number) => void,
    public onUpdateAnswer: (questionIdx: number, answer: PollAnswer) => void,
    public onSubmitSurvey: () => Promise<void>,
  ) {
    this.post = post;
    this.space = space;
    this.headerCtrl = headerCtrl;
    this.isEditMode = headerCtrl.isEditingMode;
    this.startedAt = space.started_at;
    this.endedAt = space.ended_at;
    this.totalResponses = space.user_response_count;
    this.isSurveyProgress = space.status === SpaceStatus.InProgress;
  }

  onUpdateQuestion = (index: number, question: PollQuestion) => {
    const updatedQuestions = [...editableQuestions];
    updatedQuestions[index] = question;
    setQuestions(updatedQuestions);
    headerCtrl.onModifyContent();
  };
  onDeleteQuestion = (index: number) => {
    const updatedQuestions = editableQuestions.filter((_, i) => i !== index);
    setQuestions(updatedQuestions);
    headerCtrl.onModifyContent();
  };
  onAddQuestion = () => {
    const newQuestion: PollQuestion = {
      answer_type: SurveyAnswerType.ShortAnswer,
      title: '',
      description: '',
      is_required: false,
    };
    setQuestions((q) => [...q, newQuestion]);
    headerCtrl.onModifyContent();
  };
  onUpdateAnswer = (questionIdx: number, answer: PollAnswer) => {
    setAnswers((prev) => {
      return { ...prev, [questionIdx]: answer };
    });
    setIsAnswerModified(true);
  };

  onSubmitSurvey = async () => {
    // Create answers array
    const answersArray = questions.map((q, idx) => {
      return answers[idx] || createEmptyAnswer(q.answer_type);
    });
    //Validate Answers
    if (!validateSurveyAnswer(questions, answersArray)) return;
    await submitSurveyResponse({
      spacePk,
      answers: answersArray,
    });
    setIsAnswerModified(false);
  };

  onSave = async (title: string, htmlContent: string) => {
    // Update Poll Space
    await updatePollSpace({
      postPk: feed.post.pk,
      spacePk,
      title,
      htmlContent,
      timeRange: [space.started_at ?? 0, space.ended_at ?? 0],
      questions: editableQuestions,
    });
  };

  onStartEdit = () => {
    setQuestions(space.questions || []);
  };
}

export enum Tab {
  Poll = 'poll',
  Analyze = 'analyze',
}

export function useSpacePollController(spacePk: string): PollSpaceController {
  const { space, user } = useSpaceHomeData(spacePk);

  const hasEditPermission = true; // TODO: replace with actual permission check
  const { t } = useTranslation('PollSpace');

  const updatePollSpace = useUpdatePollSpaceMutation().mutateAsync;
  const submitSurveyResponse = usePollResponseMutation().mutateAsync;
  //Edit mode state
  const [editableQuestions, setQuestions] = useState<PollQuestion[]>([]);

  const [isAnswerModified, setIsAnswerModified] = useState(false);
  // From Array, Convert into Record

  const [answers, setAnswers] = useState<Record<number, PollAnswer>>(
    Object.fromEntries(
      (space.my_response || []).map((answer, index) => [index, answer]),
    ),
  );

  return new PollSpaceController(
    t,
    spacePk,
    space,
    headerCtrl,
    questions,
    isAnswerModified,
    answers,
  );
}
