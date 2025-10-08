import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import enNav from './en/Nav.json';
import enSignIn from './en/SignIn.json';
import enSignup from './en/Signup.json';
import enThreads from './en/Threads.json';
import enSpaceForms from './en/SpaceForms.json';
import enHome from './en/Home.json';
import enTeam from './en/Team.json';
import enSprintSpace from './en/SprintSpace.json';
import enNoticeSpace from './en/NoticeSpace.json';
import enDeliberationSpace from './en/DeliberationSpace.json';
import enPollSpace from './en/PollSpace.json';
import enSpace from './en/Space.json';
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
import koThreads from './ko/Threads.json';
import koSpaceForms from './ko/SpaceForms.json';
import koHome from './ko/Home.json';
import koTeam from './ko/Team.json';
import koSprintSpace from './ko/SprintSpace.json';
import koNoticeSpace from './ko/NoticeSpace.json';
import koDeliberationSpace from './ko/DeliberationSpace.json';
import koPollSpace from './ko/PollSpace.json';
import koSpace from './ko/Space.json';
import koSettings from './ko/Settings.json';
import koSubscribe from './ko/Subscribe.json';
import koConnect from './ko/Connect.json';
import koSpaceHeader from './ko/SpaceHeader.json';
import koSpaceUnsaveModal from './ko/SpaceUnsaveModal.json';
import koSpacePublishModal from './ko/SpacePublishModal.json';
import koSpaceMakePublicModal from './ko/SpaceMakePublicModal.json';
import koEditArtworkPost from './ko/EditArtworkPost.json';

export const LANGUAGES = ['en', 'ko'];

export const resources = {
  en: {
    Nav: enNav,
    SignIn: enSignIn,
    Signup: enSignup,
    Threads: enThreads,
    SpaceForms: enSpaceForms,
    Home: enHome,
    Team: enTeam,
    SprintSpace: enSprintSpace,
    NoticeSpace: enNoticeSpace,
    DeliberationSpace: enDeliberationSpace,
    PollSpace: enPollSpace,
    Space: enSpace,
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
    Threads: koThreads,
    SpaceForms: koSpaceForms,
    Home: koHome,
    Team: koTeam,
    SprintSpace: koSprintSpace,
    NoticeSpace: koNoticeSpace,
    DeliberationSpace: koDeliberationSpace,
    PollSpace: koPollSpace,
    Space: koSpace,
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
