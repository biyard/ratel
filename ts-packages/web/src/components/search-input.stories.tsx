import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import SearchInput from './search-input';

const meta = {
  title: 'Components/SearchInput',
  component: SearchInput,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    placeholder: {
      control: 'text',
      description: 'Placeholder text',
    },
    value: {
      control: 'text',
      description: 'Input value',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-[500px]">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof SearchInput>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default
export const Default: Story = {
  args: {
    placeholder: 'Search...',
  },
};

// With value
export const WithValue: Story = {
  args: {
    value: 'Search query',
    placeholder: 'Search...',
  },
};

// Interactive
export const Interactive: Story = {
  render: () => {
    const [value, setValue] = useState('');

    return (
      <SearchInput
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Search..."
      />
    );
  },
};

// With tags
export const WithTags: Story = {
  render: () => {
    const [value, setValue] = useState('');
    const [tags, setTags] = useState([
      { id: '1', label: 'React' },
      { id: '2', label: 'TypeScript' },
    ]);

    const handleTagRemove = (tagId: string) => {
      setTags(tags.filter((tag) => tag.id !== tagId));
    };

    return (
      <SearchInput
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Search..."
        tags={tags}
        onTagRemove={handleTagRemove}
      />
    );
  },
};

// With many tags
export const WithManyTags: Story = {
  render: () => {
    const [value, setValue] = useState('');
    const [tags, setTags] = useState([
      { id: '1', label: 'React' },
      { id: '2', label: 'TypeScript' },
      { id: '3', label: 'JavaScript' },
      { id: '4', label: 'CSS' },
    ]);

    const handleTagRemove = (tagId: string) => {
      setTags(tags.filter((tag) => tag.id !== tagId));
    };

    return (
      <SearchInput
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Search..."
        tags={tags}
        onTagRemove={handleTagRemove}
      />
    );
  },
};

// With icons in tags
export const WithIconsInTags: Story = {
  render: () => {
    const [value, setValue] = useState('');
    const [tags, setTags] = useState([
      {
        id: '1',
        label: 'Project A',
        icon: <div className="w-3 h-3 bg-blue-500 rounded-sm" />,
      },
      {
        id: '2',
        label: 'Project B',
        icon: <div className="w-3 h-3 bg-green-500 rounded-sm" />,
      },
    ]);

    const handleTagRemove = (tagId: string) => {
      setTags(tags.filter((tag) => tag.id !== tagId));
    };

    return (
      <SearchInput
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Search projects..."
        tags={tags}
        onTagRemove={handleTagRemove}
      />
    );
  },
};

// Read-only
export const ReadOnly: Story = {
  render: () => {
    const tags = [
      { id: '1', label: 'Filter 1' },
      { id: '2', label: 'Filter 2' },
    ];

    return (
      <SearchInput
        value="Read-only search"
        placeholder="Search..."
        tags={tags}
        readOnly
      />
    );
  },
};

// Search with suggestions example
export const WithSuggestions: Story = {
  render: () => {
    const [value, setValue] = useState('');
    const [tags, setTags] = useState<Array<{ id: string; label: string }>>([]);
    const [suggestions] = useState([
      'React',
      'TypeScript',
      'JavaScript',
      'Vue',
      'Angular',
      'Svelte',
    ]);
    const [filteredSuggestions, setFilteredSuggestions] = useState<string[]>(
      [],
    );

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = e.target.value;
      setValue(newValue);

      if (newValue) {
        setFilteredSuggestions(
          suggestions.filter((s) =>
            s.toLowerCase().includes(newValue.toLowerCase()),
          ),
        );
      } else {
        setFilteredSuggestions([]);
      }
    };

    const handleSuggestionClick = (suggestion: string) => {
      const newTag = {
        id: Date.now().toString(),
        label: suggestion,
      };
      setTags([...tags, newTag]);
      setValue('');
      setFilteredSuggestions([]);
    };

    const handleTagRemove = (tagId: string) => {
      setTags(tags.filter((tag) => tag.id !== tagId));
    };

    return (
      <div className="relative">
        <SearchInput
          value={value}
          onChange={handleChange}
          placeholder="Search frameworks..."
          tags={tags}
          onTagRemove={handleTagRemove}
        />
        {filteredSuggestions.length > 0 && (
          <div className="absolute top-full mt-2 w-full bg-[#171717] border border-[#fcb300] rounded-lg shadow-lg z-10">
            {filteredSuggestions.map((suggestion) => (
              <div
                key={suggestion}
                className="px-4 py-2 hover:bg-[#262626] cursor-pointer text-white"
                onClick={() => handleSuggestionClick(suggestion)}
              >
                {suggestion}
              </div>
            ))}
          </div>
        )}
      </div>
    );
  },
};

// Filter search example
export const FilterSearch: Story = {
  render: () => {
    const [value, setValue] = useState('');
    const [tags, setTags] = useState([
      { id: '1', label: 'JavaScript' },
    ]);

    const items = [
      'JavaScript',
      'TypeScript',
      'Python',
      'Java',
      'C++',
      'Ruby',
      'Go',
      'Rust',
      'Swift',
      'Kotlin',
    ];

    const filteredItems = items.filter((item) => {
      const matchesSearch = item.toLowerCase().includes(value.toLowerCase());
      const matchesTags =
        tags.length === 0 ||
        tags.some((tag) => item.toLowerCase().includes(tag.label.toLowerCase()));
      return matchesSearch && matchesTags;
    });

    const handleTagRemove = (tagId: string) => {
      setTags(tags.filter((tag) => tag.id !== tagId));
    };

    return (
      <div className="flex flex-col gap-4">
        <SearchInput
          value={value}
          onChange={(e) => setValue(e.target.value)}
          placeholder="Filter languages..."
          tags={tags}
          onTagRemove={handleTagRemove}
        />
        <div className="border rounded-lg p-4 bg-white">
          <div className="text-sm text-gray-600 mb-2">
            Results: {filteredItems.length}
          </div>
          <div className="flex flex-wrap gap-2">
            {filteredItems.map((item) => (
              <div
                key={item}
                className="px-3 py-1 bg-gray-100 rounded text-sm"
              >
                {item}
              </div>
            ))}
          </div>
        </div>
      </div>
    );
  },
};

// Custom placeholder
export const CustomPlaceholder: Story = {
  render: () => {
    const [value, setValue] = useState('');

    return (
      <SearchInput
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Type to search users, teams, or projects..."
      />
    );
  },
};
