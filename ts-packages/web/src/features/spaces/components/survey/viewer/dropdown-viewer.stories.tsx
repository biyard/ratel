import { useState } from 'react';
import DropdownViewer from './dropdown-viewer';
import { SurveyAnswerType } from '@/features/spaces/polls/types/poll-question';

const mockT = ((key: string) => key) as any;

export default {
  title: 'Features/Spaces/Survey/Viewer/DropdownViewer',
  component: DropdownViewer,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export const Default = () => {
  const [selectedOption, setSelectedOption] = useState<number | null>(null);
  return (
    <DropdownViewer
      t={mockT}
      answer_type={SurveyAnswerType.Dropdown}
      title="Choose your favorite color"
      is_required={false}
      options={['Red', 'Blue', 'Green', 'Yellow']}
      selectedOption={selectedOption}
      onSelect={setSelectedOption}
    />
  );
};

export const WithSelected = () => {
  const [selectedOption, setSelectedOption] = useState<number | null>(1);
  return (
    <DropdownViewer
      t={mockT}
      answer_type={SurveyAnswerType.Dropdown}
      title="Select a programming language"
      is_required={true}
      options={['JavaScript', 'Python', 'Java', 'C++']}
      selectedOption={selectedOption}
      onSelect={setSelectedOption}
    />
  );
};

export const Disabled = () => {
  const [selectedOption, setSelectedOption] = useState<number | null>(null);
  return (
    <DropdownViewer
      t={mockT}
      answer_type={SurveyAnswerType.Dropdown}
      title="Disabled dropdown"
      is_required={false}
      options={['Option 1', 'Option 2', 'Option 3']}
      selectedOption={selectedOption}
      onSelect={setSelectedOption}
      disabled={true}
    />
  );
};
