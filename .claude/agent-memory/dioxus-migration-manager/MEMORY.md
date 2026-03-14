# Migration Manager Memory

## App Shell Migration Status (analyzed 2026-02-22)
- Shell routing: 6 top-level routes migrated (/, /auth, /posts, /:username, /teams/:teamname, /spaces)
- Header/AppMenu: Migrated with Home, MyNetwork, Notification, Membership, SignIn, Language, Theme
- Missing from shell: Footer, MobileSideMenu (toggle exists but no menu), ErrorBoundary
- ProfileDropdown: Migrated with team list, create team, logout (app/shell/src/components/profile_dropdown/)
- Missing routes: /membership, /terms, /privacy, /refund, /telegram, /admin/*, /posts/new, /artworks/new
- Missing pages: notifications (in React under social layout), rewards, messages, explore, my-network, my-follower, threads
- Social layout sidebar (UserSidemenu) exists in React but NOT in Dioxus shell -- it's delegated to user shell

## Dioxus App Module Structure
- app/shell -> orchestrator, routes to auth, posts, users, teams, spaces via ChildRouter
- app/common -> shared components (Badge, Button, Layover, Popup, ThemeSwitcher), utils, config
- app/auth -> login, signup, forgot-password, OAuth
- app/posts -> feed list, feed card, Team/TeamGroup/TeamOwner models, TeamGroupPermission types
- app/socials/users/shell -> user profile pages (posts, rewards, settings, membership, drafts, credentials, spaces)
- app/socials/teams/shell -> team pages (home, draft, group, member, dao, reward, setting)
- app/spaces/shell -> space pages with sub-modules

## Team Module Migration Status (analyzed 2026-02-22)
- See `team-migration-analysis.md` for full details
- Shell: TeamLayout + TeamSidemenu migrated (layout + nav). Missing: TeamProfile, TeamSelector, permission-based visibility
- Home page: Functional via ratel_post::components::TeamPosts (basic, no infinite scroll)
- Draft/Group/Member/DAO/Reward/Setting: ALL pure placeholders ("Coming soon...")
- DynamoDB models in app/posts: Team, TeamGroup, TeamOwner, TeamGroupPermission/Permissions
- No controllers/hooks in any team page module (all mod.rs are empty)
- main-api has 20+ team endpoints; Dioxus has only list_team_posts server fn

## React Providers Stack
QueryClient -> AuthProvider -> ThemeProvider -> PopupProvider -> TeamProvider

## TeamContext Architecture (migrated 2026-02-22)
- TeamItem + TeamContext + use_team_context moved to common/src/contexts/ (shared by all modules)
- Server fns (get_user_teams_handler, create_team_handler) in BOTH app/shell/src/contexts/team_context.rs AND app/socials/users/shell/src/controllers/team.rs
- TeamContext::init() called in AppLayout (shell), so all child routes inherit the context
- TeamSelector component in ratel-user-shell, reads from common::contexts::use_team_context()
- UserSidemenu shows TeamSelector dropdown above profile (matching React ProfileSection)
- ratel-user-shell cannot depend on app-shell (circular); has its own controllers + TeamCreationPopup
- TeamCreationPopup in user-shell opens via PopupService (use_popup) from TeamSelector dropdown
- Server fn endpoints: /api/user-shell/teams/create and /api/user-shell/teams/list (separate from shell's /api/teams/*)

## Team Creation Flow (migrated 2026-02-22)
- React: TeamSelector dropdown -> popup.open(<TeamCreationPopup/>) -> POST /v3/teams
- Dioxus: TeamSelector dropdown -> popup.open(rsx!{TeamCreationPopup{}}) -> create_team_handler server fn
- Form fields: nickname (display name), username (@team-id with validation), description
- Validation: username min 3 chars, lowercase + digits + underscores only
- On success: reload teams into TeamContext, navigate to /teams/{username}
- Default profile: https://metadata.ratel.foundation/ratel/default-profile.png

## Key Patterns
- React `route.tsx` has ~50 route helpers, Dioxus uses typed Route enum
- React Header uses Radix DropdownMenu for Profile; Dioxus ProfileDropdown uses custom dropdown
- React has Footer component with business info & i18n; Dioxus has none
- React has MobileSideMenu with team switching; Dioxus has hamburger icon only (no menu)
- React uses react-toastify for notifications; Dioxus has no toast system yet
- React TeamSidemenu shows/hides links based on TeamGroupPermissions bitmask; Dioxus shows all links

## Dependency & Build Patterns
- common cannot depend on ratel-auth or ratel-post (circular); modules needing Team/UserTeam must add them directly
- tower-sessions must be added as direct dep to any module using #[post(..., session: Extension<tower_sessions::Session>)]
- Config::dynamodb() convenience method must be added to each module's Config struct (#[cfg(feature = "server")])
- aws_sdk_dynamodb is re-exported from common::server_lib but needs explicit import in config.rs
- #[post] macro from by_macros handles server/client code generation; controller modules should NOT be gated with #[cfg(feature = "server")]
- Session .get::<String>() needs .map_err() instead of ? to avoid type inference issues in some contexts
- Dioxus hooks (use_popup, use_context, etc.) must be called at component top level, not inside event handlers
