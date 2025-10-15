import { useState } from 'react';
import LinearScaleViewer from './linear-scale-viewer';
import { SurveyAnswerType } from '../../../../../types/survey-type';
const mockT = ((key: string) => key) as any;

export default {
  title: 'Features/Spaces/Survey/Viewer/LinearScaleViewer',
  component: LinearScaleViewer,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export const Default = () => {
  const [selectedValue, setSelectedValue] = useState<number | undefined>(
    undefined,
  );
  return (
    <LinearScaleViewer
      t={mockT}
      answer_type={SurveyAnswerType.LinearScale}
      title="How satisfied are you?"
      is_required={false}
      min_label="Not satisfied"
      min_value={1}
      max_label="Very satisfied"
      max_value={5}
      selectedValue={selectedValue}
      onSelect={setSelectedValue}
    />
  );
};

export const WithCustomRange = () => {
  const [selectedValue, setSelectedValue] = useState<number | undefined>(
    undefined,
  );
  return (
    <LinearScaleViewer
      t={mockT}
      answer_type={SurveyAnswerType.LinearScale}
      title="Rate the difficulty"
      is_required={true}
      min_label="Easy"
      min_value={0}
      max_label="Hard"
      max_value={10}
      selectedValue={selectedValue}
      onSelect={setSelectedValue}
    />
  );
};

export const WithoutLabels = () => {
  const [selectedValue, setSelectedValue] = useState<number | undefined>(
    undefined,
  );
  return (
    <LinearScaleViewer
      t={mockT}
      answer_type={SurveyAnswerType.LinearScale}
      title="Score from 1 to 7"
      is_required={false}
      min_value={1}
      max_value={7}
      min_label={''}
      max_label={''}
      selectedValue={selectedValue}
      onSelect={setSelectedValue}
    />
  );
};
