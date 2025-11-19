import Title, { type TitleProps } from './title';
import { LinearScaleQuestion } from '@/features/spaces/polls/types/poll-question';

interface LinearScaleViewerProps extends LinearScaleQuestion, TitleProps {
  disabled?: boolean;
  selectedValue?: number;
  onSelect: (value: number) => void;
}

export default function LinearScaleViewer(props: LinearScaleViewerProps) {
  const {
    min_label,
    min_value,
    max_label,
    max_value,
    selectedValue,
    onSelect,
  } = props;

  return (
    <div className="flex flex-col gap-4 w-full">
      <Title {...props} />

      <div className="flex justify-between gap-4 text-xs md:text-sm text-neutral-400 px-2">
        <div className="text-left wrap-break-word">{min_label ?? ''}</div>
        <div className="text-right wrap-break-word">{max_label ?? ''}</div>
      </div>

      <div className="flex flex-wrap gap-2 justify-center px-2">
        {Array.from(
          { length: (max_value ?? 0) - (min_value ?? 0) + 1 },
          (_, i) => {
            const val = (min_value ?? 0) + i;
            const isSelected = selectedValue === val;

            return (
              <button
                key={`scale-${val}`}
                type="button"
                onClick={() => onSelect(val)}
                className={`
                  flex items-center justify-center
                  min-w-11 h-11
                  px-3 py-2
                  rounded-lg
                  text-sm font-medium
                  transition-all duration-200
                  ${
                    isSelected
                      ? 'bg-primary text-black shadow-lg scale-105'
                      : 'bg-neutral-800 light:bg-neutral-200 text-neutral-200 light:text-neutral-800 hover:bg-neutral-700 light:hover:bg-neutral-300'
                  }
                `}
              >
                {val}
              </button>
            );
          },
        )}
      </div>
    </div>
  );
}
