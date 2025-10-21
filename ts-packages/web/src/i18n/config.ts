import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import enNav from './en/Nav.json';
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

import koNav from './ko/Nav.json';
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
import { i18nSpaceTypeSelectModal } from '@/features/spaces/modals/space-type-selector-modal';
import { i18nSpaceHome } from '@/app/spaces/[id]/space-home-i18n';
import { i18nSpacePollEditor } from '@/features/spaces/polls/pages/creator/space-poll-editor-i18n';
import { i18nPollSpace } from '@/app/spaces/[id]/poll/space-poll-i18n';
import { i18nDeliberationPage } from '@/app/spaces/[id]/deliberations/deliberation-page-i18n';
import i18nSpaceSprintLeague from '@/app/spaces/[id]/sprint-league/i18n';
import { i18nTimeRangeSetting } from '@/features/spaces/polls/components/time-range-setting';
export const LANGUAGES = ['en', 'ko'];

export const resources = {
  en: {
    Nav: enNav,
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
    Nav: koNav,
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
  SpaceTypeSelectModal: i18nSpaceTypeSelectModal,
  Space: i18nSpaceHome,
  SpacePollEditor: i18nSpacePollEditor,
  SpaceSprintLeague: i18nSpaceSprintLeague,
  TimeRangeSetting: i18nTimeRangeSetting,
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

i18next.init({
  lng: 'en',
  debug: true,
  resources,
  fallbackLng: {
    ko: ['ko'],
    default: ['en'],
  },
});
