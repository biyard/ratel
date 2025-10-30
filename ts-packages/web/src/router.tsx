import { createBrowserRouter } from 'react-router';
import RootLayout from './app/layout';
import HomePage from './app/(social)/home-page';
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
// Team components
import TeamLayout from './app/teams/[username]/layout';
import TeamHome from './app/teams/[username]/home/page';
import TeamGroups from './app/teams/[username]/groups/page';
import TeamMembers from './app/teams/[username]/members/page';
import TeamSettings from './app/teams/[username]/settings/page';
import TeamDrafts from './app/teams/[username]/drafts/page';

// Space
import SpacePollPage from './app/spaces/[id]/poll/space-poll-page';
import { TestReportPage } from './app/test-report/test-report-page';
import { StorybookPage } from './app/storybook/stroybook-page';
import ThreadNotFound from './app/(social)/threads/[id]/thread-not-found';
import SpaceByIdLayout from './app/spaces/[id]/space-by-id-layout';
import { SpaceHomePage } from './app/spaces/[id]/space-home-page';
import { SpaceSettingsPage } from './app/spaces/[id]/settings/space-settings-page';
import SpaceSprintLeaguePage from './app/spaces/[id]/sprint-league/page';
import SpaceDiscussionPage from './app/spaces/[id]/discussions/space-discussion-page';
import SpaceFilePage from './app/spaces/[id]/file/space-file-page';
import SpaceRecommendationPage from './app/spaces/[id]/recommendation/space-recommendation-page';
import DiscussionPage from './app/spaces/[id]/discussions/[discussion-id]/discussion-page';

// Admin
import AdminPage from './app/admin/page';
import { MembershipsPage } from './app/admin/memberships/memberships-page';
import TelegramPage from './app/telegram/page';
import SpaceAnalyzePage from './app/spaces/[id]/analyze/space-analyze-page';
import SpacePanelPage from './app/spaces/[id]/panels/space-panel-page';

import CreatePostPage from './features/posts/components/create-post-page';
import CreateArtworkPage from './features/posts/components/create-artwork-page';
import SpaceArtNftPreviewPage from './app/spaces/[id]/art-nfts/space-art-nft-page';
import SpaceArtNftArtTwinPage from './app/spaces/[id]/art-nfts/space-art-nft-twin-page';

export const routes = createBrowserRouter([
  {
    id: 'root-layout',
    Component: RootLayout,
    ErrorBoundary: ErrorBoundary,
    children: [
      {
        id: 'create-post-page',
        path: 'posts/new',
        Component: CreatePostPage,
      },
      {
        id: 'create-artwork-page',
        path: 'artworks/new',
        Component: CreateArtworkPage,
      },

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
            id: 'my-drafts',
            path: 'drafts',
            children: [
              {
                id: 'my-drafts-list',
                path: '',
                Component: MyDraftPage,
              },
            ],
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
            ErrorBoundary: ThreadNotFound,
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

      // Team routes
      {
        id: 'teams-layout',
        path: 'teams/:username',
        Component: TeamLayout,
        children: [
          {
            id: 'team-home',
            path: 'home',
            Component: TeamHome,
          },
          {
            id: 'team-groups',
            path: 'groups',
            Component: TeamGroups,
          },
          {
            id: 'team-members',
            path: 'members',
            Component: TeamMembers,
          },
          {
            id: 'team-settings',
            path: 'settings',
            Component: TeamSettings,
          },
          {
            id: 'team-drafts',
            path: 'drafts',
            Component: TeamDrafts,
          },
        ],
      },

      // Space Layout
      {
        id: 'space-layout',
        path: 'spaces/:spacePk',
        Component: SpaceByIdLayout,
        children: [
          // Space Common
          {
            id: 'space-home-page',
            path: '',
            Component: SpaceHomePage,
          },
          {
            id: 'space-settings-page',
            path: 'settings',
            Component: SpaceSettingsPage,
          },

          {
            id: 'space-sprint-league-feature',
            path: 'sprint-leagues',
            Component: SpaceSprintLeaguePage,
          },
          // Space Poll Feature
          {
            id: 'space-poll-feature',
            path: 'polls',
            children: [
              {
                id: 'poll-by-id',
                path: ':pollPk',
                Component: SpacePollPage,
              },
            ],
          }, // End of Poll Feature
          // Space Analyze Feature
          {
            id: 'space-poll-analyze-feature',
            path: 'polls',
            children: [
              {
                id: 'poll-analyze-by-id',
                path: ':pollPk/analyzes',
                Component: SpaceAnalyzePage,
              },
            ],
          }, // End of Analyze Feature
          // Space Discussion Feature
          {
            id: 'space-discussion',
            path: 'discussions',
            children: [
              {
                id: 'space-discussion-index',
                index: true,
                Component: SpaceDiscussionPage,
              },
              {
                id: 'discussion-by-id',
                path: ':discussionPk',
                Component: DiscussionPage,
              },
            ],
          }, // End of Discussion Feature
          // Space File Feature
          {
            id: 'space-file-feature',
            path: 'files',
            Component: SpaceFilePage,
          }, // End of File Feature
          // Space Panel Feature
          {
            id: 'space-panel-feature',
            path: 'panels',
            Component: SpacePanelPage,
          }, // End of Panel Feature
          // Space Recommendation Feature
          {
            id: 'space-recommendation-feature',
            path: 'recommendations',
            Component: SpaceRecommendationPage,
          }, // End of Recommendation Feature
          {
            id: 'space-nft-page',
            path: 'nfts',
            children: [
              {
                id: 'space-nft-preview-feature',
                path: '',
                Component: SpaceArtNftPreviewPage,
              },
              {
                id: 'space-nft-art-twin-feature',
                path: 'art-twin',
                Component: SpaceArtNftArtTwinPage,
              },
            ],
          },
        ],
      }, // End of Space Layout
      {
        id: 'telegram-page',
        path: 'telegram',
        Component: TelegramPage,
      },
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

      // Admin Routes
      {
        id: 'admin-page',
        path: 'admin',
        Component: AdminPage,
      },
      {
        id: 'admin-memberships-page',
        path: 'admin/memberships',
        Component: MembershipsPage,
      }, // End of Admin Routes
    ],
  },
]);
