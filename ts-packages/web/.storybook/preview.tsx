import type { Preview } from '@storybook/nextjs';
import '../src/app/globals.css';
import { Raleway } from 'next/font/google';

const raleway = Raleway({
  variable: '--font-raleway',
  weight: ['100', '200', '300', '400', '500', '600', '700', '800', '900'],
  subsets: ['latin'], // Add the 'latin' subset
});

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
  decorators: [
    (Story) => (
      <div className={`${raleway.className}`}>
        <Story />
      </div>
    ),
  ],
};

export default preview;
