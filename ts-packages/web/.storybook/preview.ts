import type { Preview } from '@storybook/react-vite';
import '../src/index.css';
import '../src/i18n/config';

const preview: Preview = {
  parameters: {
    backgrounds: {
      options: {
        grey: { name: 'Grey', value: '#D9D9D9' },
        light: { name: 'Light', value: '#FFFFFF' },
        dark: { name: 'Dark', value: '#000000' },
      },
    },
  },
  initialGlobals: {
    backgrounds: { value: 'grey' },
  },
};

export default preview;
