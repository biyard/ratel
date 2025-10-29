import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import enSignIn from './en/SignIn.json';
import enSignup from './en/Signup.json';
import enSpaceForms from './en/SpaceForms.json';
import enHome from './en/Home.json';
import enTeam from './en/Team.json';
import enSprintSpace from './en/SprintSpace.json';
import enNoticeSpace from './en/NoticeSpace.json';
import enPollSpace from './en/PollSpace.json';
import enSettings from './en/Settings.json';
import enSubscribe from './en/Subscribe.json';
import enConnect from './en/Connect.json';
import enSpaceHeader from './en/SpaceHeader.json';
import enSpaceUnsaveModal from './en/SpaceUnsaveModal.json';
import enSpacePublishModal from './en/SpacePublishModal.json';
import enSpaceMakePublicModal from './en/SpaceMakePublicModal.json';
import enEditArtworkPost from './en/EditArtworkPost.json';

import koSignIn from './ko/SignIn.json';
import koSignup from './ko/Signup.json';
import koSpaceForms from './ko/SpaceForms.json';
import koHome from './ko/Home.json';
import koTeam from './ko/Team.json';
import koSprintSpace from './ko/SprintSpace.json';
import koNoticeSpace from './ko/NoticeSpace.json';
import koPollSpace from './ko/PollSpace.json';
import koSettings from './ko/Settings.json';
import koSubscribe from './ko/Subscribe.json';
import koConnect from './ko/Connect.json';
import koSpaceHeader from './ko/SpaceHeader.json';
import koSpaceUnsaveModal from './ko/SpaceUnsaveModal.json';
import koSpacePublishModal from './ko/SpacePublishModal.json';
import koSpaceMakePublicModal from './ko/SpaceMakePublicModal.json';
import koEditArtworkPost from './ko/EditArtworkPost.json';
import { i18nThreadPage } from '@/app/(social)/threads/[id]/thread-page-i18n';
import i18nSpaceSurveyComponent from '@/features/spaces/components/survey/i18n';
import i18nSpaceSurveyReportComponent from '@/features/spaces/components/report/i18n';
import i18nSpaceFileComponent from '@/features/spaces/files/components/space-file-editor/i18n';
import i18nSpaceDiscussionEditorPage from '@/features/spaces/discussions/pages/creator/i18n';
import i18nSpacePanelEditorPage from '@/features/spaces/panels/pages/creator/i18n';
import { i18nSpaceTypeSelectModal } from '@/features/spaces/modals/space-type-selector-modal';
import { i18nSpaceHome } from '@/app/spaces/[id]/space-home-i18n';
import { i18nSpacePollEditor } from '@/features/spaces/polls/pages/creator/space-poll-editor-i18n';
import { i18nPollSpace } from '@/app/spaces/[id]/poll/space-poll-i18n';
import { i18nDeliberationPage } from '@/app/spaces/[id]/deliberations/deliberation-page-i18n';
import i18nSpaceSprintLeague from '@/app/spaces/[id]/sprint-league/i18n';
import { i18nTimeRangeSetting } from '@/features/spaces/polls/components/time-range-setting';
import { i18nSpacePollViewerPage } from '@/features/spaces/polls/pages/viewer/space-poll-viewer-i18n';
import { i18nSpaceFileEditor } from '@/features/spaces/files/pages/creator/space-file-editor-i18n';
import { i18nSpaceRecommendationEditor } from '@/features/spaces/recommendations/pages/creator/space-recommendation-editor-i18n';
import { i18nAdmin } from '@/app/admin/admin-page-i18n';
import { i18nMemberships } from '@/features/membership/i18n';
import { i18nHeader } from '@/components/header/i18n';
import { CreatePostPage } from '@/features/posts/components/create-post-page/i18n';
import i18nListDrafts from '@/features/drafts/components/list-drafts/i18n';
import i18nEditDraftPage from '@/app/(social)/drafts/[post-id]/edit/i18n';
export const LANGUAGES = ['en', 'ko'];

// NOTE: it should be migrated to namespace based code splitting later
export const resources = {
  en: {
    SignIn: enSignIn,
    Signup: enSignup,
    SpaceForms: enSpaceForms,
    Home: enHome,
    Team: enTeam,
    SprintSpace: enSprintSpace,
    NoticeSpace: enNoticeSpace,
    PollSpace: enPollSpace,
    Settings: enSettings,
    Subscribe: enSubscribe,
    Connect: enConnect,
    SpaceHeader: enSpaceHeader,
    SpaceUnsaveModal: enSpaceUnsaveModal,
    SpacePublishModal: enSpacePublishModal,
    SpaceMakePublicModal: enSpaceMakePublicModal,
    EditArtworkPost: enEditArtworkPost,
  },
  ko: {
    SignIn: koSignIn,
    Signup: koSignup,
    SpaceForms: koSpaceForms,
    Home: koHome,
    Team: koTeam,
    SprintSpace: koSprintSpace,
    NoticeSpace: koNoticeSpace,
    PollSpace: koPollSpace,
    Settings: koSettings,
    Subscribe: koSubscribe,
    Connect: koConnect,
    SpaceHeader: koSpaceHeader,
    SpaceUnsaveModal: koSpaceUnsaveModal,
    SpacePublishModal: koSpacePublishModal,
    SpaceMakePublicModal: koSpaceMakePublicModal,
    EditArtworkPost: koEditArtworkPost,
  },
};

Object.entries({
  Threads: i18nThreadPage,
  DeliberationSpace: i18nDeliberationPage,
  PollSpace: i18nPollSpace,
  SpaceSurvey: i18nSpaceSurveyComponent,
  SpaceSurveyReport: i18nSpaceSurveyReportComponent,
  SpaceFile: i18nSpaceFileComponent,
  SpaceTypeSelectModal: i18nSpaceTypeSelectModal,
  Space: i18nSpaceHome,
  SpacePollEditor: i18nSpacePollEditor,
  SpaceSprintLeague: i18nSpaceSprintLeague,
  TimeRangeSetting: i18nTimeRangeSetting,
  SpacePollViewer: i18nSpacePollViewerPage,
  SpaceFileEditor: i18nSpaceFileEditor,
  SpaceDiscussionEditor: i18nSpaceDiscussionEditorPage,
  SpacePanelEditor: i18nSpacePanelEditorPage,
  SpaceRecommendationEditor: i18nSpaceRecommendationEditor,
  Admin: i18nAdmin,
  Memberships: i18nMemberships,
  Nav: i18nHeader,
  CreatePostPage,
  ListDrafts: i18nListDrafts,
  EditDraftPage: i18nEditDraftPage,
}).forEach(([key, value]) => {
  resources.en[key] = value.en;
  resources.ko[key] = value.ko;
});

i18next.use(LanguageDetector).use(initReactI18next);

i18next.on('failedLoading', (lng, ns, err) => {
  console.error('[i18next] failedLoading:', { lng, ns, err });
});

i18next.on('missingKey', (lngs, ns, key) => {
  console.warn('[i18next] missingKey:', { lngs, ns, key });
});

// Detect browser language and normalize it
const getBrowserLanguage = (): string => {
  if (typeof window === 'undefined') return 'en';

  const browserLang = navigator.language || (navigator as any).userLanguage;
  // Extract the language code (e.g., 'ko' from 'ko-KR', 'en' from 'en-US')
  const langCode = browserLang.split('-')[0].toLowerCase();

  // Check if the detected language is supported, otherwise default to 'en'
  return LANGUAGES.includes(langCode) ? langCode : 'en';
};

// Read saved language from localStorage, fallback to browser language
const savedLanguage =
  typeof window !== 'undefined'
    ? localStorage.getItem('user-language') || getBrowserLanguage()
    : 'en';

i18next.init({
  lng: savedLanguage,
  debug: true,
  resources,
  fallbackLng: {
    ko: ['ko'],
    default: ['en'],
  },
});
