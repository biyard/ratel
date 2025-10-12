import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import { PrimaryButton, LoadablePrimaryButton } from './primary-button';

const metaPrimaryButton = {
  title: 'Components/Button/PrimaryButton',
  component: PrimaryButton,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    disabled: {
      control: 'boolean',
      description: 'Whether the button is disabled',
    },
    onClick: {
      action: 'clicked',
      description: 'Click handler',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-80">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof PrimaryButton>;

export default metaPrimaryButton;
type StoryPrimaryButton = StoryObj<typeof metaPrimaryButton>;

// Enabled
export const Enabled: StoryPrimaryButton = {
  args: {
    disabled: false,
    onClick: () => {},
    children: 'Click Me',
  },
};

// Disabled
export const Disabled: StoryPrimaryButton = {
  args: {
    disabled: true,
    onClick: () => {},
    children: 'Disabled Button',
  },
};

// With longer text
export const WithLongerText: StoryPrimaryButton = {
  args: {
    disabled: false,
    onClick: () => {},
    children: 'Continue to Next Step',
  },
};

// Interactive
export const Interactive: StoryPrimaryButton = {
  render: () => {
    const [clicks, setClicks] = useState(0);
    return (
      <PrimaryButton disabled={false} onClick={() => setClicks(clicks + 1)}>
        Clicked {clicks} times
      </PrimaryButton>
    );
  },
};

// Form example
export const FormExample: StoryPrimaryButton = {
  render: () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const isValid = email.length > 0 && password.length > 0;

    return (
      <form className="flex flex-col gap-4 w-80">
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="px-4 py-2 border rounded"
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="px-4 py-2 border rounded"
        />
        <PrimaryButton
          disabled={!isValid}
          onClick={(e) => {
            e.preventDefault();
            alert('Form submitted!');
          }}
        >
          Sign In
        </PrimaryButton>
      </form>
    );
  },
};

// LoadablePrimaryButton stories
const metaLoadable = {
  title: 'Components/Button/LoadablePrimaryButton',
  component: LoadablePrimaryButton,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    isLoading: {
      control: 'boolean',
      description: 'Whether the button is in loading state',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the button is disabled',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-80">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof LoadablePrimaryButton>;

type StoryLoadable = StoryObj<typeof metaLoadable>;

// Normal state
export const LoadableNormal: StoryLoadable = {
  args: {
    isLoading: false,
    disabled: false,
    children: 'Submit',
  },
};

// Loading state
export const LoadableLoading: StoryLoadable = {
  args: {
    isLoading: true,
    disabled: false,
    children: 'Submit',
  },
};

// Disabled state
export const LoadableDisabled: StoryLoadable = {
  args: {
    isLoading: false,
    disabled: true,
    children: 'Submit',
  },
};

// Loading disabled (both states)
export const LoadableLoadingDisabled: StoryLoadable = {
  args: {
    isLoading: true,
    disabled: true,
    children: 'Submit',
  },
};

// Interactive loading example
export const LoadableInteractive: StoryLoadable = {
  render: () => {
    const [isLoading, setIsLoading] = useState(false);
    const [success, setSuccess] = useState(false);

    const handleClick = () => {
      setIsLoading(true);
      setSuccess(false);
      setTimeout(() => {
        setIsLoading(false);
        setSuccess(true);
        setTimeout(() => setSuccess(false), 2000);
      }, 2000);
    };

    return (
      <div className="flex flex-col gap-4 w-80">
        <LoadablePrimaryButton
          isLoading={isLoading}
          disabled={isLoading}
          onClick={handleClick}
        >
          {success ? 'Success!' : 'Click to Load'}
        </LoadablePrimaryButton>
        {success && (
          <div className="text-sm text-green-600 text-center">
            Operation completed!
          </div>
        )}
      </div>
    );
  },
};

// Form submission example
export const LoadableFormExample: StoryLoadable = {
  render: () => {
    const [email, setEmail] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [submitted, setSubmitted] = useState(false);

    const handleSubmit = () => {
      setIsLoading(true);
      setSubmitted(false);
      // Simulate API call
      setTimeout(() => {
        setIsLoading(false);
        setSubmitted(true);
        setEmail('');
      }, 2000);
    };

    const isValid = email.length > 0 && email.includes('@');

    return (
      <div className="flex flex-col gap-4 w-80">
        <input
          type="email"
          placeholder="Enter your email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="px-4 py-2 border rounded"
          disabled={isLoading}
        />
        <LoadablePrimaryButton
          isLoading={isLoading}
          disabled={!isValid || isLoading}
          onClick={handleSubmit}
        >
          Subscribe
        </LoadablePrimaryButton>
        {submitted && (
          <div className="text-sm text-green-600 text-center">
            Successfully subscribed!
          </div>
        )}
      </div>
    );
  },
};

// All button states comparison
export const AllStatesComparison: StoryLoadable = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <div>
        <p className="text-xs text-gray-600 mb-2">Primary Button - Enabled</p>
        <PrimaryButton disabled={false} onClick={() => {}}>
          Enabled Button
        </PrimaryButton>
      </div>
      <div>
        <p className="text-xs text-gray-600 mb-2">Primary Button - Disabled</p>
        <PrimaryButton disabled={true} onClick={() => {}}>
          Disabled Button
        </PrimaryButton>
      </div>
      <div>
        <p className="text-xs text-gray-600 mb-2">
          Loadable Button - Normal
        </p>
        <LoadablePrimaryButton isLoading={false} disabled={false}>
          Normal
        </LoadablePrimaryButton>
      </div>
      <div>
        <p className="text-xs text-gray-600 mb-2">
          Loadable Button - Loading
        </p>
        <LoadablePrimaryButton isLoading={true} disabled={false}>
          Loading
        </LoadablePrimaryButton>
      </div>
      <div>
        <p className="text-xs text-gray-600 mb-2">
          Loadable Button - Disabled
        </p>
        <LoadablePrimaryButton isLoading={false} disabled={true}>
          Disabled
        </LoadablePrimaryButton>
      </div>
    </div>
  ),
};
