import type { Meta, StoryObj } from '@storybook/react';
import * as Icons from './icons';

const meta: Meta = {
  title: 'Components/Icons',
  parameters: {
    layout: 'padded',
  },
};

export default meta;

type Story = StoryObj;

// Helper component to display icons in a grid
const IconGrid = ({ icons, title }: { icons: [string, React.ComponentType][]; title: string }) => (
  <div className="mb-8">
    <h2 className="text-2xl font-bold mb-4">{title}</h2>
    <div className="grid grid-cols-6 gap-4">
      {icons.map(([name, Icon]) => (
        <div
          key={name}
          className="flex flex-col items-center justify-center p-4 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors"
        >
          <Icon className="w-6 h-6 mb-2" />
          <span className="text-xs text-center break-words">{name}</span>
        </div>
      ))}
    </div>
  </div>
);

// Group icons by category
const groupIconsByCategory = () => {
  const groups: Record<string, [string, React.ComponentType][]> = {};

  Object.entries(Icons).forEach(([name, Icon]) => {
    // Determine category from name prefix
    let category = 'Root Icons';

    if (name.startsWith('Alignments')) category = 'Alignments';
    else if (name.startsWith('Arrows')) category = 'Arrows';
    else if (name.startsWith('Audio')) category = 'Audio';
    else if (name.startsWith('Bag')) category = 'Bag';
    else if (name.startsWith('BorderLayer')) category = 'Border & Layer';
    else if (name.startsWith('Browser')) category = 'Browser';
    else if (name.startsWith('Calendar')) category = 'Calendar';
    else if (name.startsWith('Chat')) category = 'Chat';
    else if (name.startsWith('Cloud')) category = 'Cloud';
    else if (name.startsWith('Controller')) category = 'Controller';
    else if (name.startsWith('Dashboard')) category = 'Dashboard';
    else if (name.startsWith('Edit')) category = 'Edit';
    else if (name.startsWith('Editor')) category = 'Editor';
    else if (name.startsWith('Email')) category = 'Email';
    else if (name.startsWith('Emoji')) category = 'Emoji';
    else if (name.startsWith('File') && !name.startsWith('Files')) category = 'File';
    else if (name.startsWith('Files')) category = 'Files';
    else if (name.startsWith('Flags')) category = 'Flags';
    else if (name.startsWith('Folder')) category = 'Folder';
    else if (name.startsWith('Game')) category = 'Game';
    else if (name.startsWith('Graph')) category = 'Graph';
    else if (name.startsWith('Health')) category = 'Health';
    else if (name.startsWith('HelpSupport')) category = 'Help & Support';
    else if (name.startsWith('Home')) category = 'Home';
    else if (name.startsWith('InternetScript')) category = 'Internet & Script';
    else if (name.startsWith('LaptopDesktop')) category = 'Laptop & Desktop';
    else if (name.startsWith('Layouts')) category = 'Layouts';
    else if (name.startsWith('LinksShare')) category = 'Links & Share';
    else if (name.startsWith('MoneyPayment')) category = 'Money & Payment';
    else if (name.startsWith('NotesClipboard')) category = 'Notes & Clipboard';
    else if (name.startsWith('Notification')) category = 'Notification';
    else if (name.startsWith('OtherDevices')) category = 'Other Devices';
    else if (name.startsWith('Phone')) category = 'Phone';
    else if (name.startsWith('Photos')) category = 'Photos';
    else if (name.startsWith('Progress')) category = 'Progress';
    else if (name.startsWith('Richtext')) category = 'Richtext';
    else if (name.startsWith('Security')) category = 'Security';
    else if (name.startsWith('Settings')) category = 'Settings';
    else if (name.startsWith('Shapes')) category = 'Shapes';
    else if (name.startsWith('Shopping')) category = 'Shopping';
    else if (name.startsWith('Time')) category = 'Time';
    else if (name.startsWith('Travel')) category = 'Travel';
    else if (name.startsWith('UploadDownload')) category = 'Upload & Download';
    else if (name.startsWith('User')) category = 'User';
    else if (name.startsWith('Validations')) category = 'Validations';
    else if (name.startsWith('VideoCamera')) category = 'Video & Camera';

    if (!groups[category]) {
      groups[category] = [];
    }

    groups[category].push([name, Icon as React.ComponentType]);
  });

  return groups;
};

export const AllIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    const sortedCategories = Object.keys(groupedIcons).sort((a, b) => {
      if (a === 'Root Icons') return -1;
      if (b === 'Root Icons') return 1;
      return a.localeCompare(b);
    });

    return (
      <div className="p-6">
        <h1 className="text-3xl font-bold mb-6">All Icons ({Object.values(Icons).length})</h1>
        {sortedCategories.map(category => (
          <IconGrid
            key={category}
            title={`${category} (${groupedIcons[category].length})`}
            icons={groupedIcons[category]}
          />
        ))}
      </div>
    );
  },
};

// Individual category stories for easier navigation
export const RootIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Root Icons" icons={groupedIcons['Root Icons'] || []} />;
  },
};

export const Alignments: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Alignments" icons={groupedIcons['Alignments'] || []} />;
  },
};

export const ArrowsIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Arrows" icons={groupedIcons['Arrows'] || []} />;
  },
};

export const AudioIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Audio" icons={groupedIcons['Audio'] || []} />;
  },
};

export const EditIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Edit" icons={groupedIcons['Edit'] || []} />;
  },
};

export const EditorIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Editor" icons={groupedIcons['Editor'] || []} />;
  },
};

export const EmailIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Email" icons={groupedIcons['Email'] || []} />;
  },
};

export const FileIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="File" icons={groupedIcons['File'] || []} />;
  },
};

export const UserIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="User" icons={groupedIcons['User'] || []} />;
  },
};

export const SecurityIcons: Story = {
  render: () => {
    const groupedIcons = groupIconsByCategory();
    return <IconGrid title="Security" icons={groupedIcons['Security'] || []} />;
  },
};

export const MembershipIcon: Story = {
  render: () => (
    <div className="p-6 space-y-6">
      <div>
        <h2 className="text-2xl font-bold mb-4">Membership Icon</h2>
        <div className="grid grid-cols-4 gap-6">
          {/* Default size */}
          <div className="flex flex-col items-center p-4 border rounded-lg">
            <Icons.Membership className="w-6 h-6 mb-2" />
            <span className="text-xs text-center">Default (w-6 h-6)</span>
          </div>

          {/* Small */}
          <div className="flex flex-col items-center p-4 border rounded-lg">
            <Icons.Membership className="w-4 h-4 mb-2" />
            <span className="text-xs text-center">Small (w-4 h-4)</span>
          </div>

          {/* Medium */}
          <div className="flex flex-col items-center p-4 border rounded-lg">
            <Icons.Membership className="w-8 h-8 mb-2" />
            <span className="text-xs text-center">Medium (w-8 h-8)</span>
          </div>

          {/* Large */}
          <div className="flex flex-col items-center p-4 border rounded-lg">
            <Icons.Membership className="w-12 h-12 mb-2" />
            <span className="text-xs text-center">Large (w-12 h-12)</span>
          </div>
        </div>
      </div>

      <div>
        <h3 className="text-xl font-bold mb-4">Color Variations</h3>
        <div className="grid grid-cols-4 gap-6">
          {/* Default */}
          <div className="flex flex-col items-center p-4 border rounded-lg">
            <Icons.Membership className="w-8 h-8 mb-2" />
            <span className="text-xs text-center">Default</span>
          </div>

          {/* Primary */}
          <div className="flex flex-col items-center p-4 border rounded-lg">
            <Icons.Membership className="w-8 h-8 mb-2 text-primary" />
            <span className="text-xs text-center">Primary</span>
          </div>

          {/* Gray */}
          <div className="flex flex-col items-center p-4 border rounded-lg">
            <Icons.Membership className="w-8 h-8 mb-2 text-gray-500" />
            <span className="text-xs text-center">Gray</span>
          </div>

          {/* Custom stroke */}
          <div className="flex flex-col items-center p-4 border rounded-lg bg-gray-900">
            <Icons.Membership className="w-8 h-8 mb-2 [&>path]:stroke-white" />
            <span className="text-xs text-center text-white">Custom Stroke</span>
          </div>
        </div>
      </div>

      <div>
        <h3 className="text-xl font-bold mb-4">Usage Examples</h3>
        <div className="space-y-4">
          {/* Button example */}
          <div className="flex items-center gap-3 p-4 border rounded-lg">
            <button className="flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-lg hover:opacity-90">
              <Icons.Membership className="w-5 h-5" />
              <span>Upgrade Membership</span>
            </button>
            <span className="text-sm text-gray-600">Button with icon</span>
          </div>

          {/* Navigation item */}
          <div className="flex items-center gap-3 p-4 border rounded-lg">
            <a href="#" className="flex items-center gap-2 px-3 py-2 rounded hover:bg-gray-100">
              <Icons.Membership className="w-5 h-5" />
              <span>Membership Plans</span>
            </a>
            <span className="text-sm text-gray-600">Navigation link</span>
          </div>

          {/* Card header */}
          <div className="p-4 border rounded-lg">
            <div className="flex items-center gap-3 mb-3">
              <div className="p-2 bg-primary/10 rounded-lg">
                <Icons.Membership className="w-6 h-6 text-primary" />
              </div>
              <div>
                <h4 className="font-semibold">Premium Membership</h4>
                <p className="text-sm text-gray-600">$29.99/month</p>
              </div>
            </div>
            <p className="text-sm text-gray-600">Card header with icon</p>
          </div>

          {/* Badge */}
          <div className="flex items-center gap-3 p-4 border rounded-lg">
            <div className="inline-flex items-center gap-2 px-3 py-1 bg-primary text-white text-sm rounded-full">
              <Icons.Membership className="w-4 h-4" />
              <span>Member</span>
            </div>
            <span className="text-sm text-gray-600">Badge with icon</span>
          </div>
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'The Membership icon in various sizes, colors, and usage contexts.',
      },
    },
  },
};
