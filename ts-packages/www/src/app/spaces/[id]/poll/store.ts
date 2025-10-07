import { create } from 'zustand';
import { Poll, SurveyAnswer } from '../type';
import { useEditCoordinatorStore } from '../space-store';
import { Answer } from '@/lib/api/models/response';

// import { useEditCoordinatorStore } from '../space-store';

export const Tab = {
  Poll: 'Poll',
  Analyze: 'Analyze',
} as const;

export type Tab = typeof Tab[keyof typeof Tab];

type State = {
  activeTab: Tab;
  survey: Poll;
  answer: SurveyAnswer;
};

type Actions = {
  initialize: (survey: Poll, answer: SurveyAnswer, initialTab: Tab) => void;
  changeTab: (tab: Tab) => void;
  updateAnswer: (answer: Answer[]) => void;
  updateSurvey: (survey: Poll) => void;
  reset: () => void;
};

const initialState: State = {
  activeTab: Tab.Poll,
  survey: { surveys: [] },
  answer: { answers: [], is_completed: false },
};

export const usePollStore = create<State & Actions>((set) => ({
  ...initialState,
  initialize: (survey, answer, initialTab) => {
    set({ survey, answer, activeTab: initialTab });
  },

  changeTab: (activeTab) => {
    set({ activeTab });
  },

  reset: () => set(initialState),

  updateSurvey: (survey) => {
    useEditCoordinatorStore.getState().setModified();
    set({ survey });
  },

  updateAnswer: (answer) => {
    set(({ answer: prevAnswer }) => ({
      answer: {
        ...prevAnswer,
        answers: answer,
      },
    }));
  },
}));
