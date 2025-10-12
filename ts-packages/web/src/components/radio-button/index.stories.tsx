import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import RadioButton from './index';

const meta = {
  title: 'Components/RadioButton',
  component: RadioButton,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    selected: {
      control: 'boolean',
      description: 'Whether the radio button is selected',
    },
    onClick: {
      action: 'clicked',
      description: 'Callback when the radio button is clicked',
    },
  },
} satisfies Meta<typeof RadioButton>;

export default meta;
type Story = StoryObj<typeof meta>;

// Selected state
export const Selected: Story = {
  args: {
    selected: true,
    onClick: () => {},
  },
};

// Unselected state
export const Unselected: Story = {
  args: {
    selected: false,
    onClick: () => {},
  },
};

// Interactive example
export const Interactive: Story = {
  render: () => {
    const [selected, setSelected] = useState(false);
    return (
      <RadioButton selected={selected} onClick={() => setSelected(!selected)} />
    );
  },
};

// Radio button group
export const RadioGroup: Story = {
  render: () => {
    const [selectedOption, setSelectedOption] = useState('option1');

    return (
      <div className="flex flex-col gap-4">
        <div className="flex items-center gap-3">
          <RadioButton
            selected={selectedOption === 'option1'}
            onClick={() => setSelectedOption('option1')}
          />
          <span className="text-sm">Option 1</span>
        </div>
        <div className="flex items-center gap-3">
          <RadioButton
            selected={selectedOption === 'option2'}
            onClick={() => setSelectedOption('option2')}
          />
          <span className="text-sm">Option 2</span>
        </div>
        <div className="flex items-center gap-3">
          <RadioButton
            selected={selectedOption === 'option3'}
            onClick={() => setSelectedOption('option3')}
          />
          <span className="text-sm">Option 3</span>
        </div>
      </div>
    );
  },
};

// Horizontal radio group
export const HorizontalRadioGroup: Story = {
  render: () => {
    const [selectedSize, setSelectedSize] = useState('medium');

    return (
      <div className="flex items-center gap-6">
        <div className="flex items-center gap-2">
          <RadioButton
            selected={selectedSize === 'small'}
            onClick={() => setSelectedSize('small')}
          />
          <span className="text-sm">Small</span>
        </div>
        <div className="flex items-center gap-2">
          <RadioButton
            selected={selectedSize === 'medium'}
            onClick={() => setSelectedSize('medium')}
          />
          <span className="text-sm">Medium</span>
        </div>
        <div className="flex items-center gap-2">
          <RadioButton
            selected={selectedSize === 'large'}
            onClick={() => setSelectedSize('large')}
          />
          <span className="text-sm">Large</span>
        </div>
      </div>
    );
  },
};

// With descriptions
export const WithDescriptions: Story = {
  render: () => {
    const [selectedPlan, setSelectedPlan] = useState('pro');

    return (
      <div className="flex flex-col gap-4 max-w-md">
        <div
          className="flex items-start gap-3 p-4 border rounded-lg cursor-pointer hover:bg-gray-50"
          onClick={() => setSelectedPlan('free')}
        >
          <RadioButton
            selected={selectedPlan === 'free'}
            onClick={() => setSelectedPlan('free')}
          />
          <div>
            <div className="font-semibold text-sm">Free Plan</div>
            <div className="text-xs text-gray-600">
              Basic features for personal use
            </div>
          </div>
        </div>
        <div
          className="flex items-start gap-3 p-4 border rounded-lg cursor-pointer hover:bg-gray-50"
          onClick={() => setSelectedPlan('pro')}
        >
          <RadioButton
            selected={selectedPlan === 'pro'}
            onClick={() => setSelectedPlan('pro')}
          />
          <div>
            <div className="font-semibold text-sm">Pro Plan</div>
            <div className="text-xs text-gray-600">
              Advanced features for professionals
            </div>
          </div>
        </div>
        <div
          className="flex items-start gap-3 p-4 border rounded-lg cursor-pointer hover:bg-gray-50"
          onClick={() => setSelectedPlan('enterprise')}
        >
          <RadioButton
            selected={selectedPlan === 'enterprise'}
            onClick={() => setSelectedPlan('enterprise')}
          />
          <div>
            <div className="font-semibold text-sm">Enterprise Plan</div>
            <div className="text-xs text-gray-600">
              Full features for large teams
            </div>
          </div>
        </div>
      </div>
    );
  },
};

// Form example
export const FormExample: Story = {
  render: () => {
    const [gender, setGender] = useState('');
    const [newsletter, setNewsletter] = useState('');

    return (
      <form className="flex flex-col gap-6 max-w-md">
        <div>
          <label className="text-sm font-medium block mb-3">Gender</label>
          <div className="flex flex-col gap-3">
            <div className="flex items-center gap-3">
              <RadioButton
                selected={gender === 'male'}
                onClick={() => setGender('male')}
              />
              <span className="text-sm">Male</span>
            </div>
            <div className="flex items-center gap-3">
              <RadioButton
                selected={gender === 'female'}
                onClick={() => setGender('female')}
              />
              <span className="text-sm">Female</span>
            </div>
            <div className="flex items-center gap-3">
              <RadioButton
                selected={gender === 'other'}
                onClick={() => setGender('other')}
              />
              <span className="text-sm">Other</span>
            </div>
          </div>
        </div>

        <div>
          <label className="text-sm font-medium block mb-3">
            Newsletter Frequency
          </label>
          <div className="flex flex-col gap-3">
            <div className="flex items-center gap-3">
              <RadioButton
                selected={newsletter === 'daily'}
                onClick={() => setNewsletter('daily')}
              />
              <span className="text-sm">Daily</span>
            </div>
            <div className="flex items-center gap-3">
              <RadioButton
                selected={newsletter === 'weekly'}
                onClick={() => setNewsletter('weekly')}
              />
              <span className="text-sm">Weekly</span>
            </div>
            <div className="flex items-center gap-3">
              <RadioButton
                selected={newsletter === 'never'}
                onClick={() => setNewsletter('never')}
              />
              <span className="text-sm">Never</span>
            </div>
          </div>
        </div>

        <button
          type="submit"
          className="mt-2 bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
          onClick={(e) => e.preventDefault()}
        >
          Submit
        </button>
      </form>
    );
  },
};
