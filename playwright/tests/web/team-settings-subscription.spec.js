// Team Settings Subscription Page E2E Test
//
// This test covers navigation to the team subscription settings page
// and verification of membership plan cards.
//
// Prerequisites:
// - The backend must be built with --features bypass for auth setup
// - The auth-setup project must run first (provides user.json storage state)
// - The authenticated user (hi+user1@biyard.co) must be an admin of a team
//
// Flow:
// 1. Create a team (ensuring the user is admin)
// 2. Navigate to the team settings page
// 3. Click the "Subscription" tab in the settings sidebar
// 4. Verify the subscription page loads with membership plan cards
// 5. Verify individual plan card content

import { test, expect } from "@playwright/test";
import { click, goto } from "../utils";

test.describe.serial(
  "Team settings subscription page navigation",
  () => {
    const teamNickname = "Subscription Test Team";
    const teamUsername = `e2e_sub_${Date.now()}`;

    test("should create a team so the user is admin", async ({ page }) => {
      // Team creation is setup for this suite; the renewed home arena no
      // longer exposes a profile-dropdown "Create Team" path, so drive it
      // through the same REST endpoint the form submits to.
      const res = await page.request.post("/api/user-shell/teams/create", {
        data: {
          body: {
            username: teamUsername,
            nickname: teamNickname,
            profile_url: "",
            description: "E2E test team for subscription settings",
          },
        },
      });
      expect(res.ok(), `create team: ${await res.text()}`).toBeTruthy();

      await goto(page, `/${teamUsername}/home`);
      await expect(page).toHaveURL(new RegExp(`/${teamUsername}/home`));
    });

    test("should navigate to team settings page", async ({ page }) => {
      // Navigate directly to the team settings page
      await goto(page, `/${teamUsername}/team-settings`);

      // Verify the settings page loaded. The Subscription & Billing card is
      // rendered for admins/owners and contains the Change Plan link.
      await expect(page).toHaveURL(new RegExp(`/${teamUsername}/team-settings`));
      await expect(
        page.getByText("Subscription & Billing", { exact: true }),
      ).toBeVisible();
      await expect(
        page.getByTestId("team-settings-change-plan"),
      ).toBeVisible();
    });

    test("should click the Change Plan link and load the subscription page", async ({
      page,
    }) => {
      // Navigate to team settings first (each test gets a fresh page from storageState)
      await goto(page, `/${teamUsername}/team-settings`);

      // Wait for the admin view to render before clicking.
      await expect(
        page.getByText("Subscription & Billing", { exact: true }),
      ).toBeVisible();

      // The Change Plan link is visible only for admins/owners.
      // Since the user created the team, they are the owner.
      await click(page, { testId: "team-settings-change-plan" });

      // Wait for URL to update to the subscription route
      await page.waitForURL(
        new RegExp(`/${teamUsername}/team-settings/subscription`),
        { waitUntil: "load" },
      );

      // Verify we are on the subscription page
      await expect(page).toHaveURL(
        new RegExp(`/${teamUsername}/team-settings/subscription`),
      );
    });

    test("should display the membership plans header", async ({ page }) => {
      // Navigate to the subscription page directly
      await goto(page, `/${teamUsername}/team-settings/subscription`);

      // The MembershipPlanHeader renders "Membership Plans" as the h1 heading
      const heading = page.getByRole("heading", { name: "Membership Plans" });
      await expect(heading).toBeVisible();
    });

    test("should display all membership plan cards", async ({ page }) => {
      await goto(page, `/${teamUsername}/team-settings/subscription`);

      // Wait for the membership plans heading to confirm page is loaded
      await expect(
        page.getByRole("heading", { name: "Membership Plans" }),
      ).toBeVisible();

      // Verify the Free plan card is visible
      // The Free plan card displays "Free" as the name and its description
      const freePlan = page.getByText("Basic membership open to everyone");
      await expect(freePlan).toBeVisible();

      // Verify the Pro plan card is visible
      const proPlan = page.getByText("Reward Space setup for small communities");
      await expect(proPlan).toBeVisible();

      // Verify the Max plan card is visible
      const maxPlan = page.getByText(
        "Advanced Reward Spaces for large communities",
      );
      await expect(maxPlan).toBeVisible();

      // Verify the VIP plan card is visible
      const vipPlan = page.getByText(
        "Reward Spaces for influencers and promotion",
      );
      await expect(vipPlan).toBeVisible();

      // Verify the Enterprise plan card is visible
      const enterprisePlan = page.getByText(
        "Customized partner plan for enterprises & organizations",
      );
      await expect(enterprisePlan).toBeVisible();
    });

    test("should display plan features and action buttons", async ({
      page,
    }) => {
      await goto(page, `/${teamUsername}/team-settings/subscription`);

      // Wait for the page to fully render
      await expect(
        page.getByRole("heading", { name: "Membership Plans" }),
      ).toBeVisible();

      // Free plan has no action button (btn: None), verify its features
      await expect(page.getByText("Publish posts")).toBeVisible();
      await expect(page.getByText("Publish spaces")).toBeVisible();

      // Pro plan has a "Get Pro" button
      await expect(
        page.getByRole("button", { name: "Get Pro" }),
      ).toBeVisible();

      // Max plan has a "Get Max" button
      await expect(
        page.getByRole("button", { name: "Get Max" }),
      ).toBeVisible();

      // VIP plan has a "Get VIP" button
      await expect(
        page.getByRole("button", { name: "Get VIP" }),
      ).toBeVisible();

      // Enterprise plan has a "Contact Us" button
      await expect(
        page.getByRole("button", { name: "Contact Us" }),
      ).toBeVisible();

      // Verify pricing information is displayed for paid plans
      await expect(page.getByText("₩30,000 / month")).toBeVisible();
      await expect(page.getByText("₩75,000 / month")).toBeVisible();
      await expect(page.getByText("₩150,000 / month")).toBeVisible();
      await expect(
        page.getByText("Starting at $1,000 / month"),
      ).toBeVisible();
    });

    test("should remain on the subscription page after direct navigation", async ({
      page,
    }) => {
      await goto(page, `/${teamUsername}/team-settings/subscription`);

      // Wait for page to load
      await expect(
        page.getByRole("heading", { name: "Membership Plans" }),
      ).toBeVisible();

      // Confirm URL still matches (no redirect for admins/owners).
      await expect(page).toHaveURL(
        new RegExp(`/${teamUsername}/team-settings/subscription`),
      );
    });
  },
);
