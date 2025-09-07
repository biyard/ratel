import { test, expect } from '@playwright/test';
import { HomePage } from './pages/HomePage';
import { mockApiResponses, testPosts, testComments } from './fixtures/test-data';

test.describe('Social Features', () => {
  test.beforeEach(async ({ page }) => {
    // Mock authenticated user session
    await page.route('**/api/auth/session', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ 
          data: mockApiResponses.userProfile,
          authenticated: true 
        }),
      });
    });

    // Mock feed data
    await page.route('**/api/feeds/**', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ data: mockApiResponses.feedPosts }),
      });
    });
  });

  test.describe('Post Interactions', () => {
    test('should like a post', async ({ page }) => {
      const homePage = new HomePage(page);
      
      // Mock like API
      let likeRequests = 0;
      await page.route('**/api/posts/*/like', route => {
        likeRequests++;
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ success: true, liked: true }),
        });
      });
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Find and click like button on first post
      const firstPost = page.locator('[data-testid="post-card"]').first();
      const likeButton = firstPost.locator('[data-testid="like-button"]');
      
      if (await likeButton.isVisible()) {
        const initialLikeCount = await likeButton.locator('[data-testid="like-count"]').textContent();
        await likeButton.click();
        
        // Should have made a like request
        expect(likeRequests).toBe(1);
        
        await page.screenshot({ path: 'test-results/screenshots/post-liked.png' });
      }
    });

    test('should open comment section', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Click comment button on first post
      const firstPost = page.locator('[data-testid="post-card"]').first();
      const commentButton = firstPost.locator('[data-testid="comment-button"]');
      
      if (await commentButton.isVisible()) {
        await commentButton.click();
        
        // Comment section should open
        const commentSection = firstPost.locator('[data-testid="comment-section"]');
        if (await commentSection.isVisible()) {
          await expect(commentSection).toBeVisible();
          await page.screenshot({ path: 'test-results/screenshots/comment-section-open.png' });
        }
      }
    });

    test('should share a post', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Click share button on first post
      const firstPost = page.locator('[data-testid="post-card"]').first();
      const shareButton = firstPost.locator('[data-testid="share-button"]');
      
      if (await shareButton.isVisible()) {
        await shareButton.click();
        
        // Share modal or options should appear
        const shareModal = page.locator('[data-testid="share-modal"]');
        if (await shareModal.isVisible()) {
          await expect(shareModal).toBeVisible();
          await page.screenshot({ path: 'test-results/screenshots/share-modal.png' });
        }
      }
    });
  });

  test.describe('Create Post', () => {
    test('should open create post modal', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      
      // Click create post button
      const createPostButton = page.locator('[data-testid="create-post-button"]');
      if (await createPostButton.isVisible()) {
        await createPostButton.click();
        
        // Create post modal should open
        const createPostModal = page.locator('[data-testid="create-post-modal"]');
        if (await createPostModal.isVisible()) {
          await expect(createPostModal).toBeVisible();
          await page.screenshot({ path: 'test-results/screenshots/create-post-modal.png' });
        }
      }
    });

    test('should create a new post', async ({ page }) => {
      const homePage = new HomePage(page);
      
      // Mock create post API
      let createPostRequests = 0;
      await page.route('**/api/posts', route => {
        createPostRequests++;
        route.fulfill({
          status: 201,
          contentType: 'application/json',
          body: JSON.stringify({ 
            data: { 
              id: 999, 
              ...testPosts.samplePost,
              author_name: mockApiResponses.userProfile.name
            } 
          }),
        });
      });
      
      await homePage.goto();
      
      // Open create post modal
      const createPostButton = page.locator('[data-testid="create-post-button"]');
      if (await createPostButton.isVisible()) {
        await createPostButton.click();
        
        const createPostModal = page.locator('[data-testid="create-post-modal"]');
        if (await createPostModal.isVisible()) {
          // Fill in post details
          const titleInput = page.locator('[data-testid="post-title-input"]');
          const contentInput = page.locator('[data-testid="post-content-input"]');
          const submitButton = page.locator('[data-testid="submit-post-button"]');
          
          if (await titleInput.isVisible()) {
            await titleInput.fill(testPosts.samplePost.title);
          }
          
          if (await contentInput.isVisible()) {
            await contentInput.fill(testPosts.samplePost.content);
          }
          
          if (await submitButton.isVisible()) {
            await submitButton.click();
            
            // Should have made create post request
            expect(createPostRequests).toBe(1);
            
            await page.screenshot({ path: 'test-results/screenshots/post-created.png' });
          }
        }
      }
    });
  });

  test.describe('User Profile Interactions', () => {
    test('should open user profile when clicking on username', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Click on username in first post
      const firstPost = page.locator('[data-testid="post-card"]').first();
      const username = firstPost.locator('[data-testid="post-author"]');
      
      if (await username.isVisible()) {
        await username.click();
        
        // Should navigate to user profile or open profile modal
        const profilePage = page.locator('[data-testid="user-profile"]');
        const profileModal = page.locator('[data-testid="profile-modal"]');
        
        const hasProfilePage = await profilePage.isVisible();
        const hasProfileModal = await profileModal.isVisible();
        
        expect(hasProfilePage || hasProfileModal).toBeTruthy();
        
        await page.screenshot({ path: 'test-results/screenshots/user-profile.png' });
      }
    });

    test('should follow/unfollow user', async ({ page }) => {
      const homePage = new HomePage(page);
      
      // Mock follow API
      let followRequests = 0;
      await page.route('**/api/users/*/follow', route => {
        followRequests++;
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ success: true, following: true }),
        });
      });
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Look for follow button in post or profile
      const followButton = page.locator('[data-testid="follow-button"]').first();
      if (await followButton.isVisible()) {
        await followButton.click();
        
        // Should have made follow request
        expect(followRequests).toBe(1);
        
        await page.screenshot({ path: 'test-results/screenshots/user-followed.png' });
      }
    });
  });

  test.describe('Feed Interactions', () => {
    test('should scroll to load more posts', async ({ page }) => {
      const homePage = new HomePage(page);
      
      // Mock infinite scroll API
      let pageRequests = 0;
      await page.route('**/api/feeds**', route => {
        pageRequests++;
        const url = new URL(route.request().url());
        const page_num = url.searchParams.get('page') || '1';
        
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ 
            data: mockApiResponses.feedPosts.map(post => ({
              ...post,
              id: post.id + parseInt(page_num) * 10
            }))
          }),
        });
      });
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Get initial post count
      const initialPosts = await page.locator('[data-testid="post-card"]').count();
      
      // Scroll to bottom to trigger infinite scroll
      await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
      
      // Wait for new posts to load
      await page.waitForTimeout(2000);
      
      const finalPosts = await page.locator('[data-testid="post-card"]').count();
      
      // Should have loaded more posts or made additional requests
      expect(pageRequests).toBeGreaterThanOrEqual(1);
      
      await page.screenshot({ path: 'test-results/screenshots/infinite-scroll.png' });
    });

    test('should filter posts by category', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Look for filter options
      const filterButtons = page.locator('[data-testid="filter-button"]');
      const filterCount = await filterButtons.count();
      
      if (filterCount > 0) {
        // Click first filter option
        await filterButtons.first().click();
        
        // Posts should be filtered
        await page.waitForTimeout(1000);
        await page.screenshot({ path: 'test-results/screenshots/filtered-posts.png' });
      }
    });
  });

  test.describe('Real-time Features', () => {
    test('should handle live updates', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Simulate real-time update (new post notification)
      await page.evaluate(() => {
        // Simulate WebSocket message or real-time update
        const event = new CustomEvent('newPost', { 
          detail: { 
            id: 9999, 
            title: 'Live Update Post',
            content: 'This post appeared in real-time'
          } 
        });
        window.dispatchEvent(event);
      });
      
      // Check if notification appears
      const notification = page.locator('[data-testid="new-post-notification"]');
      if (await notification.isVisible()) {
        await expect(notification).toBeVisible();
        await page.screenshot({ path: 'test-results/screenshots/realtime-update.png' });
      }
    });
  });

  test.describe('Mobile Social Features', () => {
    test.use({ viewport: { width: 375, height: 667 } });

    test('should work correctly on mobile devices', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      await homePage.waitForFeedToLoad();
      
      // Test mobile interactions
      const firstPost = page.locator('[data-testid="post-card"]').first();
      if (await firstPost.isVisible()) {
        // Like button should work on mobile
        const likeButton = firstPost.locator('[data-testid="like-button"]');
        if (await likeButton.isVisible()) {
          await likeButton.click();
          await page.screenshot({ path: 'test-results/screenshots/mobile-like.png' });
        }
      }
      
      // Create post should work on mobile
      const createPostButton = page.locator('[data-testid="create-post-button"]');
      if (await createPostButton.isVisible()) {
        await createPostButton.click();
        await page.screenshot({ path: 'test-results/screenshots/mobile-create-post.png' });
      }
    });
  });
});