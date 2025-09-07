import { chromium, FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig) {
  const { baseURL } = config.projects[0].use;
  
  // Wait for the application to be ready
  console.log('Waiting for application to be ready...');
  
  const browser = await chromium.launch();
  const page = await browser.newPage();
  
  try {
    // Wait for the home page to load
    await page.goto(baseURL || 'http://localhost:8080', {
      waitUntil: 'networkidle',
      timeout: 30000,
    });
    
    console.log('Application is ready for testing');
  } catch (error) {
    console.error('Failed to connect to application:', error);
    throw error;
  } finally {
    await browser.close();
  }
}

export default globalSetup;