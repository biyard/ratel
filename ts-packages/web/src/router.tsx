import { createBrowserRouter } from 'react-router';
import RootLayout from './app/layout';
import HomePage from './app/(social)/page';
import SocialLayout from './app/(social)/layout';
import { ErrorBoundary } from './components/error-boundary';
import MyPostsPage from './app/(social)/my-posts/page';
import MyDraftPage from './app/(social)/drafts/page';
import MyProfilePage from './app/(social)/settings/page';
import SettingLayout from './app/(social)/settings/layout';
import MyNetwork from './app/(social)/my-network/page';
import MyFollowerPage from './app/(social)/my-follower/page';
import { z } from 'zod';
import ThreadPage from './app/(social)/threads/[id]/thread-page';

// Space
import SpaceLayout from './app/spaces/layout';
import PollSpacePage from './app/spaces/poll/[id]/page';
import { TestReportPage } from './app/test-report/test-report-page';
import { StorybookPage } from './app/storybook/stroybook-page';
import DeliberationSpacePage from './app/spaces/deliberation/[id]/page';

export const routes = createBrowserRouter([
  {
    id: 'root-layout',
    Component: RootLayout,
    ErrorBoundary: ErrorBoundary,
    children: [
      // Social Layout
      {
        id: 'social-layout',
        Component: SocialLayout,
        children: [
          // Social home routes
          {
            id: 'home-page',
            index: true,
            Component: HomePage,
          },
          {
            id: 'my-posts-page',
            path: 'my-posts',
            Component: MyPostsPage,
          },
          {
            id: 'my-drafts-page',
            path: 'drafts',
            Component: MyDraftPage,
          },
          {
            id: 'settings-layout',
            Component: SettingLayout,
            path: 'settings',
            children: [
              {
                id: 'settings-page',
                index: true,
                Component: MyProfilePage,
              },
            ],
          },

          // Threads
          {
            id: 'thread-page',
            path: 'threads/:post_id',
            Component: ThreadPage,
          },

          // My network
          {
            id: 'my-network-page',
            path: 'my-network',
            Component: MyNetwork,
          },

          {
            id: 'my-followers-page',
            path: 'my-follower',
            loader: ({ request }) => {
              const url = new URL(request.url);
              const type = z
                .enum(['followers', 'followings'])
                .default('followers')
                .parse(url.searchParams.get('type') ?? undefined);
              return { type };
            },
            Component: MyFollowerPage,
          },
        ],
      }, // End of Social Layout

      // Space Layout
      {
        id: 'space-layout',
        path: 'spaces/:spacePk',
        Component: SpaceLayout,
        children: [
          {
            id: 'poll-space',
            path: 'poll',
            Component: PollSpacePage,
          },
          {
            id: 'deliberation-space',
            path: 'deliberation',
            Component: DeliberationSpacePage,
          },
        ],
      }, // End of Space Layout

      // Test Report Page
      {
        id: 'test-report-page',
        path: 'test-report',
        Component: TestReportPage,
      }, // End of TestReportPage

      {
        id: 'storybook-page',
        path: 'storybook',
        Component: StorybookPage,
      }, // End of StorybookPage
    ],
  },
]);
