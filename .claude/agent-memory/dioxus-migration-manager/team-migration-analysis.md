# Team Module Migration Analysis (2026-02-22)

## Summary
- 7 team sub-pages exist in Dioxus, but 6 of 7 are pure placeholders
- Only Team Home has minimal functionality (delegates to TeamPosts component)
- 20+ backend endpoints in main-api have NO corresponding Dioxus server functions
- DynamoDB models (Team, TeamGroup, TeamOwner) exist in app/posts/src/models/ but are not used by team pages

## Dioxus Pages Status
| Page | Has Controllers | Has Models | Has Hooks | Has Real UI | Status |
|------|----------------|------------|-----------|-------------|--------|
| Shell (layout) | Empty | Empty | Empty | Sidemenu + Layout | Partial |
| Home | None (uses ratel_post) | None | Empty | TeamPosts component | Partial |
| Draft | Empty | Empty | Empty | "Coming soon..." | Placeholder |
| Group | Empty | Empty | Empty | "Coming soon..." | Placeholder |
| Member | Empty | Empty | Empty | "Coming soon..." | Placeholder |
| DAO | Empty | Empty | Empty | "Coming soon..." | Placeholder |
| Reward | Empty | Empty | Empty | "Coming soon..." | Placeholder |
| Setting | Empty | Empty | Empty | "Coming soon..." | Placeholder |

## React Features by Page
### Home: Team feed with infinite scroll, FeedCard per post
### Drafts: Draft post list, delete drafts, CreatePostButton (team context)
### Groups: List groups, create group popup, invite member popup, delete group, permission checks
### Members: List members with group badges, remove member from group
### DAO: DAO registration card, blockchain interaction (Kaia), MetaMask/wallet, admin-only
### Rewards: Points summary, exchange preview, transaction list with infinite scroll
### Settings: Edit profile (logo upload, nickname, description), delete team popup, permission checks

## main-api v3 Team Endpoints
| Endpoint | Method | Handler | Dioxus Equivalent |
|----------|--------|---------|-------------------|
| /v3/teams | POST | create_team | NONE |
| /v3/teams?username= | GET | find_team | NONE |
| /v3/teams/:pk | GET | get_team | NONE |
| /v3/teams/:pk | PATCH | update_team | NONE |
| /v3/teams/:pk | DELETE | delete_team | NONE |
| /v3/teams/:pk/members | GET | list_members | NONE |
| /v3/teams/:pk/posts | GET | list_team_posts | list_team_posts_handler (in ratel-post) |
| /v3/teams/:pk/groups | GET | list_groups | NONE |
| /v3/teams/:pk/groups | POST | create_group | NONE |
| /v3/teams/:pk/groups/:sk | POST | update_group | NONE |
| /v3/teams/:pk/groups/:sk | DELETE | delete_group | NONE |
| /v3/teams/:pk/groups/:sk/member | POST | add_member | NONE |
| /v3/teams/:pk/groups/:sk/member | DELETE | remove_member | NONE |
| /v3/teams/:pk/membership | GET | get_team_membership | NONE |
| /v3/teams/:pk/membership | POST | change_team_membership | NONE |
| /v3/teams/:pk/membership/history | GET | get_team_purchase_history | NONE |
| /v3/teams/:pk/points | GET | get_team_rewards | NONE |
| /v3/teams/:pk/points/transactions | GET | list_team_point_transactions | NONE |
