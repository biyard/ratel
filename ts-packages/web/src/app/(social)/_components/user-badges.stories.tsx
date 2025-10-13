import type { Meta, StoryObj } from '@storybook/react';
import UserBadges from './user-badges';

const meta = {
  title: 'App/Social/UserBadges',
  component: UserBadges,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    badges: {
      description: 'Array of badge objects with id, name, and image_url',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-[300px]">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof UserBadges>;

export default meta;
type Story = StoryObj<typeof meta>;

// Sample badge data
const sampleBadges = [
  {
    id: 1,
    name: 'Early Adopter',
    image_url: 'https://via.placeholder.com/100x100/3B82F6/FFFFFF?text=EA',
  },
  {
    id: 2,
    name: 'Top Contributor',
    image_url: 'https://via.placeholder.com/100x100/10B981/FFFFFF?text=TC',
  },
  {
    id: 3,
    name: 'Verified',
    image_url: 'https://via.placeholder.com/100x100/F59E0B/FFFFFF?text=V',
  },
  {
    id: 4,
    name: 'Pro Member',
    image_url: 'https://via.placeholder.com/100x100/8B5CF6/FFFFFF?text=PM',
  },
  {
    id: 5,
    name: 'Community Leader',
    image_url: 'https://via.placeholder.com/100x100/EF4444/FFFFFF?text=CL',
  },
];

// Few badges
export const FewBadges: Story = {
  args: {
    badges: sampleBadges.slice(0, 3),
  },
};

// Full grid
export const FullGrid: Story = {
  args: {
    badges: sampleBadges,
  },
};

// Many badges (10 badges)
export const ManyBadges: Story = {
  args: {
    badges: [
      ...sampleBadges,
      {
        id: 6,
        name: 'Mentor',
        image_url: 'https://via.placeholder.com/100x100/06B6D4/FFFFFF?text=M',
      },
      {
        id: 7,
        name: 'Helper',
        image_url: 'https://via.placeholder.com/100x100/84CC16/FFFFFF?text=H',
      },
      {
        id: 8,
        name: 'Expert',
        image_url: 'https://via.placeholder.com/100x100/EC4899/FFFFFF?text=E',
      },
      {
        id: 9,
        name: 'Pioneer',
        image_url: 'https://via.placeholder.com/100x100/F97316/FFFFFF?text=P',
      },
      {
        id: 10,
        name: 'Champion',
        image_url: 'https://via.placeholder.com/100x100/14B8A6/FFFFFF?text=C',
      },
    ],
  },
};

// Single badge
export const SingleBadge: Story = {
  args: {
    badges: [sampleBadges[0]],
  },
};

// Empty badges
export const NoBadges: Story = {
  args: {
    badges: [],
  },
};

// In a profile card
export const InProfileCard: Story = {
  render: () => (
    <div className="border rounded-lg p-6 bg-white max-w-md">
      <div className="flex items-center gap-4 mb-4">
        <div className="w-16 h-16 bg-blue-500 rounded-full" />
        <div>
          <h3 className="font-semibold text-lg">John Doe</h3>
          <p className="text-sm text-gray-600">@johndoe</p>
        </div>
      </div>
      <div className="mb-4">
        <p className="text-sm text-gray-700">
          Software engineer passionate about open source and community building.
        </p>
      </div>
      <div className="border-t pt-4">
        <h4 className="font-semibold text-sm mb-3">Achievements</h4>
        <UserBadges badges={sampleBadges} />
      </div>
    </div>
  ),
};

// With tooltips (mockup)
export const WithTooltips: Story = {
  render: () => (
    <div className="p-4">
      <h4 className="font-semibold text-sm mb-3">Badges (hover for details)</h4>
      <div className="grid grid-cols-5 gap-2.5 items-center justify-start">
        {sampleBadges.map((badge) => (
          <div
            className="relative aspect-square group"
            key={`badge-${badge.id}`}
            title={badge.name}
          >
            <img
              className="object-cover w-full h-full rounded hover:scale-110 transition-transform"
              alt={`Badge ${badge.name}`}
              src={badge.image_url}
            />
            <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 bg-black text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none">
              {badge.name}
            </div>
          </div>
        ))}
      </div>
    </div>
  ),
};

// Different grid sizes
export const GridSizes: Story = {
  render: () => (
    <div className="flex flex-col gap-6">
      <div>
        <h4 className="font-semibold text-sm mb-3">3 Columns</h4>
        <div className="grid grid-cols-3 gap-2.5 items-center justify-start max-w-[200px]">
          {sampleBadges.slice(0, 6).map((badge) => (
            <div className="relative aspect-square" key={`3col-${badge.id}`}>
              <img
                className="object-cover w-full h-full"
                alt={`Badge ${badge.name}`}
                src={badge.image_url}
              />
            </div>
          ))}
        </div>
      </div>
      <div>
        <h4 className="font-semibold text-sm mb-3">5 Columns (Default)</h4>
        <div className="max-w-[300px]">
          <UserBadges badges={sampleBadges} />
        </div>
      </div>
      <div>
        <h4 className="font-semibold text-sm mb-3">7 Columns</h4>
        <div className="grid grid-cols-7 gap-2.5 items-center justify-start max-w-[400px]">
          {[...sampleBadges, ...sampleBadges.slice(0, 2)].map((badge, idx) => (
            <div className="relative aspect-square" key={`7col-${idx}`}>
              <img
                className="object-cover w-full h-full"
                alt={`Badge ${badge.name}`}
                src={badge.image_url}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  ),
};
