import useFeedById from '@/hooks/feeds/use-feed-by-id';
import usePollSpace from '@/features/poll-space/hooks/use-poll-space';
import {
  SpaceHeaderController,
  useSpaceHeader,
} from '@/features/spaces/components/header/use-space-header';
import { useState } from 'react';
import { SpaceCommon } from '@/types/space-common';
import { Post } from '@/lib/api/ratel/posts.v3';
import { SpaceSurveyProps } from '@/features/spaces/components/survey';
import {
  SurveyAnswer,
  SurveyAnswerType,
  SurveyQuestion,
} from '@/types/survey-type';

export interface ISpaceController extends SpaceSurveyProps {
  post: Post;
  space: SpaceCommon;
  headerCtrl: SpaceHeaderController;
  isEditMode: boolean;
  activeTab: Tab;
  onSelectTab: (tab: Tab) => void;
}

export class PollSpaceController implements ISpaceController {
  public post: Post;
  public space: SpaceCommon;
  public headerCtrl: SpaceHeaderController;

  isEditMode: boolean;
  constructor(
    post: Post,
    space: SpaceCommon,
    headerCtrl: SpaceHeaderController,
    public activeTab: Tab,
    public onSelectTab: (tab: Tab) => void,
    public questions: SurveyQuestion[],
    public answers: SurveyAnswer[],
    public handleAddQuestion: () => void,
    public handleUpdateQuestion: (
      index: number,
      question: SurveyQuestion,
    ) => void,
    public handleDeleteQuestion: (index: number) => void,
    public handleUpdateAnswer: (
      questionIdx: number,
      answer: SurveyAnswer,
    ) => void,
  ) {
    this.post = post;
    this.space = space;
    this.headerCtrl = headerCtrl;
    this.isEditMode = headerCtrl.isEditingMode;
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

  const onSave = async (title: string, html_content: string) => {
    // Implement save logic here
    console.log('Save changes', { title, html_content });
  };

  const [activeTab, setActiveTab] = useState<Tab>(Tab.Poll);
  const [questions, setQuestions] = useState<SurveyQuestion[]>([]);
  const [answers, setAnswers] = useState<SurveyAnswer[]>([]);

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

  const handleSelectTab = (tab: Tab) => {
    setActiveTab(tab);
  };

  const handleUpdateQuestion = (index: number, question: SurveyQuestion) => {
    const updatedQuestions = [...questions];
    updatedQuestions[index] = question;
    setQuestions(updatedQuestions);
  };
  const handleDeleteQuestion = (index: number) => {
    const updatedQuestions = questions.filter((_, i) => i !== index);
    setQuestions(updatedQuestions);
  };
  const handleAddQuestion = () => {
    const newQuestion: SurveyQuestion = {
      answer_type: SurveyAnswerType.ShortAnswer,
      content: {
        title: '',
        description: '',
        is_required: false,
      },
    };
    setQuestions((q) => [...q, newQuestion]);
  };
  const handleUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    const updatedAnswers = [...answers];
    updatedAnswers[questionIdx] = answer;
    setAnswers(updatedAnswers);
  };

  return new PollSpaceController(
    feed.post,
    space as SpaceCommon,
    headerCtrl,
    activeTab,
    handleSelectTab,

    questions,
    answers,

    handleAddQuestion,
    handleUpdateQuestion,
    handleDeleteQuestion,

    handleUpdateAnswer,
  );
}
