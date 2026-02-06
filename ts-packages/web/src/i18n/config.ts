import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import enSignIn from './en/SignIn.json';
import enSignup from './en/Signup.json';
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
import i18nSpaceMemberEditorPage from '@/features/spaces/members/pages/creator/i18n';
import i18nSpaceMemberViewerPage from '@/features/spaces/members/pages/viewer/i18n';
import { i18nSpaceTypeSelectModal } from '@/features/spaces/modals/space-type-selector-modal';
import { i18nSpaceHome } from '@/app/spaces/[id]/space-home-i18n';
import { i18nSpacePollEditor } from '@/features/spaces/polls/pages/creator/space-poll-editor-i18n';
import { i18nSpacePollAnalyze } from '@/features/spaces/polls/pages/analyze/space-poll-analyze-i18n';
import { i18nSpacePollsEditor } from '@/features/spaces/polls/pages/creator/list/space-polls-editor-i18n';
import { i18nPollSpace } from '@/app/spaces/[id]/poll/space-poll-i18n';
import { i18nDeliberationPage } from '@/app/spaces/[id]/deliberations/deliberation-page-i18n';
import i18nSpaceSprintLeague from '@/app/spaces/[id]/sprint-league/i18n';
import { i18nTimeRangeSetting } from '@/features/spaces/polls/components/time-range-setting';
import { i18nSpacePollViewerPage } from '@/features/spaces/polls/pages/viewer/space-poll-viewer-i18n';
import { i18nSpaceFileEditor } from '@/features/spaces/files/pages/creator/space-file-editor-i18n';
import { i18nSpaceRecommendationEditor } from '@/features/spaces/recommendations/pages/creator/space-recommendation-editor-i18n';
import { i18nAdmin } from '@/app/admin/admin-page-i18n';
import { i18nAdminMigrations } from '@/app/admin/migrations/migrations-page-i18n';
import { i18nAdmins } from '@/app/admin/users/admins-page-i18n';
import { i18nMemberships } from '@/features/membership/i18n';
import { i18nNotifications } from '@/features/notification/i18n';
import { i18nHeader } from '@/components/header/i18n';
import i18nListDrafts from '@/features/drafts/components/list-drafts/i18n';
import { CreatePostPage } from '@/features/posts/components/create-post-page/i18n';
import { CreateArtworkPage } from '@/features/posts/components/create-artwork-page/i18n';
import { i18nSpaceForm } from '@/features/spaces/components/space-form-i18n';
import { i18nSubmitSurveyModal } from '@/features/spaces/polls/components/modal/submit_survey/submit-survey-modal-i18n';
import { MembershipPlan } from '@/features/membership/components/membership-plan/i18n';
import { MembershipPage } from '@/app/membership/membership-page-i18n';
import { UserSidemenu } from '@/features/users/components/user-sidemenu/i18n';
import { Credentials } from '@/features/did/components/credentials/i18n';
import { i18nFooter } from '@/components/footer/i18n';
import { Terms } from '@/app/terms/i18n';
import { Privacy } from '@/app/privacy/i18n';
import { Refund } from '@/app/refund/i18n';
import { SpaceSettings } from '@/app/spaces/[id]/settings/i18n';
import { i18nSpaceBoardsCreate } from '@/features/spaces/boards/pages/creator/create/space-boards-create-i18n';
import { i18nSpaceBoardsEditor } from '@/features/spaces/boards/pages/creator/space-boards-editor-i18n';
import { i18nSpaceBoardsEditorDetail } from '@/features/spaces/boards/pages/creator/detail/space-boards-editor-detail-i18n';
import { i18nAttributeCodes } from '@/app/admin/attribute-codes/attribute-codes-page-i18n';
import { i18nAdminRewards } from '@/app/admin/rewards/rewards-page-i18n';
import { Errors } from '@/features/errors/i18n';
import { MySpaces } from '@/app/(social)/my-spaces/i18n';
import { i18nCompleteSurveyModal } from '@/features/spaces/polls/components/modal/complete_survey/complete-survey-modal-i18n';
import { i18nSpaceRewards } from '@/features/spaces/rewards/i18n';
import { i18nRewardsPage } from '@/app/rewards/rewards-page-i18n';
import { i18nTeamGroups } from '@/features/teams/groups/i18n';
import { i18nTeamDao } from '@/features/teams/dao/i18n';
import { i18nTeamRewardsPage } from '@/features/teams/rewards/i18n';
import { i18nTeamSettingsPage } from '@/features/teams/settings/i18n';
import { i18nSpaceIncentiveEditor } from '@/features/spaces/incentive/pages/creator/space-incentive-editor-i18n';
import { i18nSpaceLayout } from '@/features/spaces/layout/space-layout-i18n';
export const LANGUAGES = ['en', 'ko'];

// NOTE: it should be migrated to namespace based code splitting later
export const resources = {
  en: {
    SignIn: enSignIn,
    Signup: enSignup,
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
  SpaceHome: i18nSpaceHome,
  SpaceLayout: i18nSpaceLayout,
  SpacePollSubmitSurvey: i18nSubmitSurveyModal,
  SpaceCompleteSurvey: i18nCompleteSurveyModal,
  SpacePollAnalyze: i18nSpacePollAnalyze,
  SpacePollEditor: i18nSpacePollEditor,
  SpacePollsEditor: i18nSpacePollsEditor,
  SpaceSprintLeague: i18nSpaceSprintLeague,
  TimeRangeSetting: i18nTimeRangeSetting,
  SpacePollViewer: i18nSpacePollViewerPage,
  SpaceFileEditor: i18nSpaceFileEditor,
  SpaceBoardsCreate: i18nSpaceBoardsCreate,
  SpaceBoardsEditor: i18nSpaceBoardsEditor,
  SpaceBoardsEditorDetail: i18nSpaceBoardsEditorDetail,
  SpaceDiscussionEditor: i18nSpaceDiscussionEditorPage,
  SpacePanelEditor: i18nSpacePanelEditorPage,
  SpaceRecommendationEditor: i18nSpaceRecommendationEditor,
  SpaceMemberEditor: i18nSpaceMemberEditorPage,
  SpaceMemberViewer: i18nSpaceMemberViewerPage,
  Admin: i18nAdmin,
  AdminMigrations: i18nAdminMigrations,
  Admins: i18nAdmins,
  Memberships: i18nMemberships,
  Notifications: i18nNotifications,
  AttributeCodes: i18nAttributeCodes,
  AdminRewards: i18nAdminRewards,
  Nav: i18nHeader,
  ListDrafts: i18nListDrafts,
  CreatePostPage,
  CreateArtworkPage,
  SpaceForm: i18nSpaceForm,
  MembershipPlan,
  MembershipPage,
  UserSidemenu,
  Credentials,
  Footer: i18nFooter,
  Terms,
  Privacy,
  Refund,
  SpaceSettings,
  Errors,
  MySpaces,
  SpaceRewards: i18nSpaceRewards,
  MyRewards: i18nRewardsPage,
  TeamGroups: i18nTeamGroups,
  TeamDao: i18nTeamDao,
  SpaceIncentiveEditor: i18nSpaceIncentiveEditor,
  TeamRewards: i18nTeamRewardsPage,
  TeamSettings: i18nTeamSettingsPage,
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

  const browserLang = navigator.language || navigator.languages[0];
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

export type lang = 'en' | 'ko';
