import { createBrowserRouter } from 'react-router';
import React from 'react';
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
import TeamDao from './app/teams/[username]/dao/page';
import TeamRewards from './app/teams/[username]/rewards/page';

// Space
import SpacePollPage from './app/spaces/[id]/poll/space-poll-page';
import { TestReportPage } from './app/test-report/test-report-page';
import { StorybookPage } from './app/storybook/stroybook-page';
import ThreadNotFound from './app/(social)/threads/[id]/thread-not-found';
import SpaceByIdLayout from './features/spaces/layout/space-by-id-layout';
import { SpaceHomePage } from './app/spaces/[id]/space-home-page';
import { SpaceSettingsPage } from './app/spaces/[id]/settings/space-settings-page';
import { SpaceRewardsPage } from './app/spaces/[id]/rewards/space-rewards-page';
import SpaceSprintLeaguePage from './app/spaces/[id]/sprint-league/page';
import SpaceDiscussionPage from './app/spaces/[id]/discussions/space-discussion-page';
import SpaceFilePage from './app/spaces/[id]/file/space-file-page';
import SpaceRecommendationPage from './app/spaces/[id]/recommendation/space-recommendation-page';
import DiscussionPage from './app/spaces/[id]/discussions/[discussion-id]/discussion-page';
import SpaceRequirementPage from './app/spaces/[id]/requirements/space-requirment-page';

// Admin
import AdminPage from './app/admin/page';
import { MembershipsPage } from './app/admin/memberships/memberships-page';
import AttributeCodesPage from './app/admin/attribute-codes/page';
import AdminUsersPage from './app/admin/users/page';
import { RewardsPage as AdminRewardsPage } from './app/admin/rewards/rewards-page';
import MigrationsPage from './app/admin/migrations/page';
import TelegramPage from './app/telegram/page';
import SpaceAnalyzePage from './app/spaces/[id]/analyze/space-analyze-page';
import SpacePanelPage from './app/spaces/[id]/panels/space-panel-page';

// Auth
import ForgotPasswordPage from './app/(auth)/forgot-password/page';
import ResetPasswordPage from './app/(auth)/reset-password/page';

import CreatePostPage from './features/posts/components/create-post-page';
import SpacePollsPage from './app/spaces/[id]/polls/space-polls-page';
import SpaceAnalyzesPage from './app/spaces/[id]/analyzes/space-analyzes-page';
import CreateArtworkPage from './features/posts/components/create-artwork-page';
import SpaceArtNftPreviewPage from './app/spaces/[id]/art-nfts/space-art-nft-page';
import SpaceArtNftArtTwinPage from './app/spaces/[id]/art-nfts/space-art-nft-twin-page';
import MembershipPage from './app/membership/membership-page';
import SpaceBoardPage from './app/spaces/[id]/boards/[post-id]/space-board-page';
import SpaceBoardsPage from './app/spaces/[id]/boards/space-boards-page';
import SpaceBoardCreatePage from './app/spaces/[id]/boards/create/space-board-create-page';
import { Credentials } from './features/did/components/credentials';
import { Terms } from './app/terms';
import { Privacy } from './app/privacy';
import { Refund } from './app/refund';
import SpaceMemberPage from './app/spaces/[id]/members/space-member-page';
import MySpacesPage from './app/(social)/my-spaces/page';
import NotificationsPage from './app/notifications/page';
import RewardsPage from './app/rewards/page';
import { MembershipPlan } from './features/membership/components/membership-plan';
import Providers from './providers/providers';
import { PdfViewerLoader } from './features/spaces/files/components/pdf-viewer-loader';
import SpaceDaoPage from './app/spaces/[id]/dao/space-dao-page';
import SpaceIncentivePage from './app/spaces/[id]/incentive/space-incentive-page';

export const routes = createBrowserRouter([
  // PDF Viewer - Completely standalone without any layout
  {
    id: 'space-pdf-viewer-standalone',
    path: '/spaces/:spacePk/files/:fileId',
    Component: () => (
      <Providers>
        <PdfViewerLoader />
      </Providers>
    ),
  },
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

      {
        id: 'membership-page',
        path: 'membership',
        Component: MembershipPlan,
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
            id: 'my-membership-page',
            path: 'my-membership',
            Component: MembershipPage,
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
            id: 'my-spaces-page',
            path: 'my-spaces',
            Component: MySpacesPage,
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

          {
            id: 'my-credential-page',
            path: 'credentials',
            Component: Credentials,
          },
          {
            id: 'notifications-page',
            path: 'notifications',
            Component: NotificationsPage,
          },
          {
            id: 'rewards-page',
            path: 'rewards',
            Component: RewardsPage,
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
          {
            id: 'team-rewards',
            path: 'rewards',
            Component: TeamRewards,
          },
          {
            id: 'team-dao',
            path: 'dao',
            Component: TeamDao,
          },
        ],
      },

      // Space Layout
      {
        id: 'space-layout',
        path: 'spaces/:spacePk',
        Component: SpaceByIdLayout,
        handle: { hideHeader: true },
        children: [
          // Space Common

          {
            id: 'space-home-page',
            path: '',
            Component: SpaceHomePage,
          },
          {
            id: 'space-members',
            path: 'members',
            Component: SpaceMemberPage,
          },
          // Space Boards Feature
          {
            id: 'space-boards',
            path: 'boards',
            children: [
              {
                id: 'space-board-index',
                index: true,
                Component: SpaceBoardsPage,
              },
              {
                id: 'create-space-post',
                path: 'create',
                Component: SpaceBoardCreatePage,
              },
              {
                id: 'space-post-detail',
                path: 'posts/:postPk',
                Component: SpaceBoardPage,
                handle: { hideSpaceHeader: true },
              },
            ],
          }, // End of Boards Feature
          {
            id: 'space-settings-page',
            path: 'settings',
            Component: SpaceSettingsPage,
          },
          {
            id: 'space-rewards-page',
            path: 'rewards',
            Component: SpaceRewardsPage,
          },
          // Space Dao Feature
          {
            id: 'space-dao-page',
            path: 'dao',
            Component: SpaceDaoPage,
          }, // End of Dao Feature
          {
            id: 'space-incentive-page',
            path: 'incentive',
            Component: SpaceIncentivePage,
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
                id: 'poll',
                path: '',
                Component: SpacePollsPage,
              },
              {
                id: 'analyzes',
                path: 'analyzes',
                Component: SpaceAnalyzesPage,
              },
              {
                id: 'poll-by-id',
                path: ':pollPk',
                Component: SpacePollPage,
              },
              {
                id: 'poll-analyze-by-id',
                path: ':pollPk/analyzes',
                Component: SpaceAnalyzePage,
              },
            ],
          }, // End of Poll Feature
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
          {
            id: 'space-requirement-page',
            path: 'requirements',
            Component: SpaceRequirementPage,
            handle: { hideSpaceHeader: true },
          },
        ],
      }, // End of Space Layout
      {
        id: 'telegram-page',
        path: 'telegram',
        Component: TelegramPage,
      },
      // Auth Routes
      {
        id: 'forgot-password-page',
        path: 'forgot-password',
        Component: ForgotPasswordPage,
      },
      {
        id: 'reset-password-page',
        path: 'reset-password',
        Component: ResetPasswordPage,
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
        id: 'admin-users-page',
        path: 'admin/users',
        Component: AdminUsersPage,
      },
      {
        id: 'admin-memberships-page',
        path: 'admin/memberships',
        Component: MembershipsPage,
      },
      {
        id: 'admin-attribute-codes-page',
        path: 'admin/attribute-codes',
        Component: AttributeCodesPage,
      },
      {
        id: 'admin-rewards-page',
        path: 'admin/rewards',
        Component: AdminRewardsPage,
      },
      {
        id: 'admin-migrations-page',
        path: 'admin/migrations',
        Component: MigrationsPage,
      }, // End of Admin Routes

      {
        id: 'terms-page',
        path: 'terms',
        Component: Terms,
      },
      {
        id: 'privacy-page',
        path: 'privacy',
        Component: Privacy,
      },
      {
        id: 'refund-page',
        path: 'refund',
        Component: Refund,
      },
    ],
  },
]);
