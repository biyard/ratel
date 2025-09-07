export const testUsers = {
  testUser1: {
    email: 'test1@ratel.foundation',
    password: 'TestPassword123!',
    name: 'Test User 1',
    username: 'testuser1',
  },
  testUser2: {
    email: 'test2@ratel.foundation', 
    password: 'TestPassword456!',
    name: 'Test User 2',
    username: 'testuser2',
  },
};

export const testPosts = {
  samplePost: {
    title: 'Sample Post Title',
    content: 'This is a sample post content for testing purposes.',
    industry: 'Technology',
  },
  longPost: {
    title: 'Long Post with Detailed Content',
    content: `This is a longer post content that contains multiple paragraphs and detailed information. 
    
    It should test how the application handles longer content and ensures proper display and interaction.
    
    This post also contains various formatting elements to test the rich text editor functionality.`,
    industry: 'Policy',
  },
};

export const testComments = {
  shortComment: 'Great post! Thanks for sharing.',
  longComment: `This is a detailed comment that provides substantial feedback on the post. 
  It includes multiple points and demonstrates how longer comments are handled in the interface.`,
};

export const mockApiResponses = {
  userProfile: {
    id: 1,
    name: 'Test User',
    email: 'test@example.com',
    username: 'testuser',
    profile_image_url: 'https://example.com/avatar.jpg',
    follower_count: 42,
    following_count: 24,
    post_count: 15,
  },
  
  feedPosts: [
    {
      id: 1,
      title: 'Test Post 1',
      content: 'Content of test post 1',
      author_name: 'Test Author 1',
      likes: 10,
      comments: 5,
      created_at: Date.now() - 86400000, // 1 day ago
    },
    {
      id: 2, 
      title: 'Test Post 2',
      content: 'Content of test post 2',
      author_name: 'Test Author 2', 
      likes: 25,
      comments: 8,
      created_at: Date.now() - 172800000, // 2 days ago
    },
  ],
  
  politicians: [
    {
      id: 1,
      name: 'John Politician',
      party: 'Test Party',
      position: 'Representative',
      stance_score: 75,
    },
    {
      id: 2,
      name: 'Jane Politician', 
      party: 'Another Party',
      position: 'Senator',
      stance_score: 60,
    },
  ],
};

export const testUrls = {
  home: '/',
  politicians: '/politicians',
  profile: '/profile',
  messages: '/messages',
  notifications: '/notifications',
};

export const testViewports = {
  mobile: { width: 375, height: 667 },
  tablet: { width: 768, height: 1024 },
  desktop: { width: 1280, height: 720 },
  large: { width: 1920, height: 1080 },
};

export const testSelectors = {
  // Common elements
  loading: '[data-testid="loading"]',
  error: '[data-testid="error-message"]',
  modal: '[data-testid="modal"]',
  
  // Navigation
  nav: 'nav',
  mobileMenu: '[data-testid="mobile-menu"]',
  mobileMenuButton: '[data-testid="mobile-menu-button"]',
  
  // Authentication
  signInButton: '[data-testid="sign-in-button"]',
  signOutButton: '[data-testid="sign-out-button"]',
  userMenu: '[data-testid="user-menu"]',
  
  // Feed
  feedContainer: '[data-testid="feed-container"]',
  postCard: '[data-testid="post-card"]',
  likeButton: '[data-testid="like-button"]',
  commentButton: '[data-testid="comment-button"]',
  shareButton: '[data-testid="share-button"]',
  
  // Forms
  createPostButton: '[data-testid="create-post-button"]',
  postTitleInput: '[data-testid="post-title-input"]',
  postContentInput: '[data-testid="post-content-input"]',
  submitPostButton: '[data-testid="submit-post-button"]',
};