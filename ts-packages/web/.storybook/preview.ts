import type { Preview } from '@storybook/react-vite';

const preview: Preview = {
  // ...rest of preview
  //👇 Enables auto-generated documentation for all stories
  tags: ['autodocs'],
  decorators: [],
};

export default preview;
