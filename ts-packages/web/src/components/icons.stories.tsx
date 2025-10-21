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
