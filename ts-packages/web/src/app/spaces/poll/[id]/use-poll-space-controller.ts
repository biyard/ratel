import useFeedById from '@/hooks/feeds/use-feed-by-id';
import usePollSpace from '@/features/spaces/polls/hooks/use-poll-space';
import {
  SpaceHeaderController,
  useSpaceHeader,
} from '@/features/spaces/components/header/use-space-header';
import { useState } from 'react';
import { Post } from '@/lib/api/ratel/posts.v3';
import { SurveyProps } from '@/features/spaces/components/survey';
import {
  createEmptyAnswer,
  SurveyAnswer,
  SurveyAnswerType,
  SurveyQuestion,
} from '@/types/survey-type';
import { ReportProps } from '@/features/spaces/components/report';
import { PollSpaceResponse } from '@/lib/api/ratel/poll.spaces.v3';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import { useUpdatePollSpaceMutation } from '@/features/spaces/polls/hooks/use-update-poll-mutation';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { usePollResponseMutation } from '@/features/spaces/polls/hooks/use-poll-response-mutation';
import validateSurveyAnswer from '@/features/spaces/utils/validate-survey-answer';

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
    public questions: SurveyQuestion[],
    public isAnswerModified: boolean,
    public answers: Record<number, SurveyAnswer>,
    public onAddQuestion: () => void,
    public onUpdateQuestion: (index: number, question: SurveyQuestion) => void,
    public onDeleteQuestion: (index: number) => void,
    public onUpdateAnswer: (questionIdx: number, answer: SurveyAnswer) => void,
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
}

export enum Tab {
  Poll = 'poll',
  Analyze = 'analyze',
}

export function usePollSpaceController(spacePk: string): PollSpaceController {
  const { data: space } = usePollSpace(spacePk);
  const { data: feed } = useFeedById(space.post_pk);
  const hasEditPermission = true; // TODO: replace with actual permission check
  const { t } = useTranslation('PollSpace');

  const [activeTab, setActiveTab] = useState<Tab>(
    space.status === SpaceStatus.Finished && hasEditPermission
      ? Tab.Analyze
      : Tab.Poll,
  );
  const updatePollSpace = useUpdatePollSpaceMutation().mutateAsync;
  const submitSurveyResponse = usePollResponseMutation().mutateAsync;
  //Edit mode state
  const [editableQuestions, setQuestions] = useState<SurveyQuestion[]>([]);

  const [isAnswerModified, setIsAnswerModified] = useState(false);
  // From Array, Convert into Record

  const [answers, setAnswers] = useState<Record<number, SurveyAnswer>>(
    Object.fromEntries(
      (space.my_response || []).map((answer, index) => [index, answer]),
    ),
  );

  const onSave = async (title: string, htmlContent: string) => {
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

  const onStartEdit = () => {
    setQuestions(space.questions || []);
  };

  const headerCtrl = useSpaceHeader(
    feed.post,
    space,
    hasEditPermission,
    onSave,
    onStartEdit,
  );

  const questions = headerCtrl.isEditingMode
    ? editableQuestions
    : space.questions;

  const handleSelectTab = (tab: Tab) => {
    setActiveTab(tab);
  };

  const onUpdateQuestion = (index: number, question: SurveyQuestion) => {
    const updatedQuestions = [...editableQuestions];
    updatedQuestions[index] = question;
    setQuestions(updatedQuestions);
    headerCtrl.onModifyContent();
  };
  const onDeleteQuestion = (index: number) => {
    const updatedQuestions = editableQuestions.filter((_, i) => i !== index);
    setQuestions(updatedQuestions);
    headerCtrl.onModifyContent();
  };
  const onAddQuestion = () => {
    const newQuestion: SurveyQuestion = {
      answer_type: SurveyAnswerType.ShortAnswer,
      title: '',
      description: '',
      is_required: false,
    };
    setQuestions((q) => [...q, newQuestion]);
    headerCtrl.onModifyContent();
  };
  const onUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    console.log(answer);
    setAnswers((prev) => {
      return { ...prev, [questionIdx]: answer };
    });
    setIsAnswerModified(true);
  };

  const onSubmitSurvey = async () => {
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
  return new PollSpaceController(
    t,
    spacePk,
    feed.post,
    space,
    headerCtrl,
    activeTab,
    handleSelectTab,
    questions,
    isAnswerModified,
    answers,
    onAddQuestion,
    onUpdateQuestion,
    onDeleteQuestion,

    onUpdateAnswer,
    onSubmitSurvey,
  );
}
