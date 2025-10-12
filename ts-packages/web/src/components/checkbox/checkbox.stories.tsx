import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import { Checkbox } from './checkbox';

const meta = {
  title: 'Components/Checkbox',
  component: Checkbox,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    isRounded: {
      control: 'boolean',
      description: 'Whether the checkbox is rounded',
    },
    onChange: {
      action: 'changed',
      description: 'Callback when checkbox state changes',
    },
  },
} satisfies Meta<typeof Checkbox>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default (square)
export const Default: Story = {
  args: {
    isRounded: false,
    id: 'default-checkbox',
    onChange: () => {},
    children: <span>Default checkbox</span>,
  },
};

// Rounded
export const Rounded: Story = {
  args: {
    isRounded: true,
    id: 'rounded-checkbox',
    onChange: () => {},
    children: <span>Rounded checkbox</span>,
  },
};

// Without label
export const WithoutLabel: Story = {
  args: {
    isRounded: false,
    id: 'no-label-checkbox',
    onChange: () => {},
  },
};

// Interactive square
export const InteractiveSquare: Story = {
  render: () => {
    const [checked, setChecked] = useState(false);
    return (
      <div className="flex flex-col gap-2">
        <Checkbox isRounded={false} id="interactive-square" onChange={setChecked}>
          <span>Square checkbox</span>
        </Checkbox>
        <div className="text-sm text-gray-500">
          Status: {checked ? 'Checked' : 'Unchecked'}
        </div>
      </div>
    );
  },
};

// Interactive rounded
export const InteractiveRounded: Story = {
  render: () => {
    const [checked, setChecked] = useState(false);
    return (
      <div className="flex flex-col gap-2">
        <Checkbox isRounded={true} id="interactive-rounded" onChange={setChecked}>
          <span>Rounded checkbox</span>
        </Checkbox>
        <div className="text-sm text-gray-500">
          Status: {checked ? 'Checked' : 'Unchecked'}
        </div>
      </div>
    );
  },
};

// Multiple checkboxes
export const MultipleCheckboxes: Story = {
  render: () => {
    const [checks, setChecks] = useState({
      option1: false,
      option2: false,
      option3: false,
    });

    return (
      <div className="flex flex-col gap-3">
        <Checkbox
          isRounded={false}
          id="option1"
          onChange={(checked) => setChecks({ ...checks, option1: checked })}
        >
          <span>Option 1</span>
        </Checkbox>
        <Checkbox
          isRounded={false}
          id="option2"
          onChange={(checked) => setChecks({ ...checks, option2: checked })}
        >
          <span>Option 2</span>
        </Checkbox>
        <Checkbox
          isRounded={false}
          id="option3"
          onChange={(checked) => setChecks({ ...checks, option3: checked })}
        >
          <span>Option 3</span>
        </Checkbox>
        <div className="text-sm text-gray-500 mt-2">
          Selected: {Object.values(checks).filter(Boolean).length}
        </div>
      </div>
    );
  },
};

// Form example
export const FormExample: Story = {
  render: () => {
    const [agreeTerms, setAgreeTerms] = useState(false);
    const [newsletter, setNewsletter] = useState(false);
    const [updates, setUpdates] = useState(false);

    const canSubmit = agreeTerms;

    return (
      <form className="flex flex-col gap-4 max-w-md">
        <div className="space-y-3">
          <Checkbox
            isRounded={false}
            id="terms"
            onChange={setAgreeTerms}
          >
            <span>
              I agree to the{' '}
              <a href="#" className="text-blue-400 hover:underline">
                Terms and Conditions
              </a>
              {' '}*
            </span>
          </Checkbox>
          <Checkbox
            isRounded={false}
            id="newsletter"
            onChange={setNewsletter}
          >
            <span>Subscribe to newsletter (optional)</span>
          </Checkbox>
          <Checkbox
            isRounded={false}
            id="updates"
            onChange={setUpdates}
          >
            <span>Receive product updates (optional)</span>
          </Checkbox>
        </div>
        <button
          type="submit"
          disabled={!canSubmit}
          className={`py-2 px-4 rounded ${
            canSubmit
              ? 'bg-blue-500 text-white hover:bg-blue-600'
              : 'bg-gray-600 text-gray-400 cursor-not-allowed'
          }`}
          onClick={(e) => e.preventDefault()}
        >
          Submit
        </button>
      </form>
    );
  },
};

// Settings panel
export const SettingsPanel: Story = {
  render: () => {
    const [settings, setSettings] = useState({
      notifications: true,
      autoSave: true,
      darkMode: false,
      animations: true,
    });

    return (
      <div className="max-w-md border rounded-lg p-4">
        <h3 className="font-semibold mb-4 text-white">Preferences</h3>
        <div className="space-y-3">
          <Checkbox
            isRounded={false}
            id="notifications"
            onChange={(checked) =>
              setSettings({ ...settings, notifications: checked })
            }
          >
            <div>
              <div className="font-medium text-sm">Enable Notifications</div>
              <div className="text-xs text-gray-400">
                Receive alerts and updates
              </div>
            </div>
          </Checkbox>
          <Checkbox
            isRounded={false}
            id="autoSave"
            onChange={(checked) =>
              setSettings({ ...settings, autoSave: checked })
            }
          >
            <div>
              <div className="font-medium text-sm">Auto-save</div>
              <div className="text-xs text-gray-400">
                Automatically save your work
              </div>
            </div>
          </Checkbox>
          <Checkbox
            isRounded={false}
            id="darkMode"
            onChange={(checked) =>
              setSettings({ ...settings, darkMode: checked })
            }
          >
            <div>
              <div className="font-medium text-sm">Dark Mode</div>
              <div className="text-xs text-gray-400">
                Use dark color theme
              </div>
            </div>
          </Checkbox>
          <Checkbox
            isRounded={false}
            id="animations"
            onChange={(checked) =>
              setSettings({ ...settings, animations: checked })
            }
          >
            <div>
              <div className="font-medium text-sm">Animations</div>
              <div className="text-xs text-gray-400">
                Enable UI animations
              </div>
            </div>
          </Checkbox>
        </div>
      </div>
    );
  },
};

// Todo list example
export const TodoList: Story = {
  render: () => {
    const [todos, setTodos] = useState([
      { id: '1', text: 'Review pull requests', completed: false },
      { id: '2', text: 'Update documentation', completed: true },
      { id: '3', text: 'Fix bug #123', completed: false },
      { id: '4', text: 'Deploy to production', completed: false },
    ]);

    const toggleTodo = (id: string, checked: boolean) => {
      setTodos(
        todos.map((todo) =>
          todo.id === id ? { ...todo, completed: checked } : todo,
        ),
      );
    };

    const completedCount = todos.filter((t) => t.completed).length;

    return (
      <div className="max-w-md border rounded-lg p-4">
        <div className="flex justify-between items-center mb-4">
          <h3 className="font-semibold text-white">Tasks</h3>
          <span className="text-sm text-gray-400">
            {completedCount} / {todos.length} completed
          </span>
        </div>
        <div className="space-y-2">
          {todos.map((todo) => (
            <Checkbox
              key={todo.id}
              isRounded={false}
              id={todo.id}
              onChange={(checked) => toggleTodo(todo.id, checked)}
            >
              <span
                className={
                  todo.completed ? 'line-through text-gray-500' : ''
                }
              >
                {todo.text}
              </span>
            </Checkbox>
          ))}
        </div>
      </div>
    );
  },
};

// Both styles comparison
export const StylesComparison: Story = {
  render: () => (
    <div className="flex flex-col gap-6">
      <div>
        <h4 className="font-medium mb-3 text-white">Square Checkboxes</h4>
        <div className="space-y-2">
          <Checkbox isRounded={false} id="square1" onChange={() => {}}>
            <span>First option</span>
          </Checkbox>
          <Checkbox isRounded={false} id="square2" onChange={() => {}}>
            <span>Second option</span>
          </Checkbox>
          <Checkbox isRounded={false} id="square3" onChange={() => {}}>
            <span>Third option</span>
          </Checkbox>
        </div>
      </div>
      <div>
        <h4 className="font-medium mb-3 text-white">Rounded Checkboxes</h4>
        <div className="space-y-2">
          <Checkbox isRounded={true} id="round1" onChange={() => {}}>
            <span>First option</span>
          </Checkbox>
          <Checkbox isRounded={true} id="round2" onChange={() => {}}>
            <span>Second option</span>
          </Checkbox>
          <Checkbox isRounded={true} id="round3" onChange={() => {}}>
            <span>Third option</span>
          </Checkbox>
        </div>
      </div>
    </div>
  ),
};
