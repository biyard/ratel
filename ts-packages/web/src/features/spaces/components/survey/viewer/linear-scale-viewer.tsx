import RadioButton from '@/components/radio-button';
import Title, { type TitleProps } from './title';
import { logger } from '@/lib/logger';
import { LinearScaleQuestion } from '@/features/spaces/polls/types/poll-question';
import React from 'react';

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

  const wrapRef = React.useRef<HTMLDivElement | null>(null);
  const pos = React.useRef({ isDown: false, startX: 0, scrollLeft: 0 });

  const onPointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el) return;

    const target = e.target as Element;
    if (
      target.closest('[data-stop-drag],button,[role="button"],input,label,svg')
    ) {
      return;
    }

    if (e.pointerType !== 'mouse') return;

    el.setPointerCapture(e.pointerId);
    pos.current.isDown = true;
    pos.current.startX = e.clientX;
    pos.current.scrollLeft = el.scrollLeft;
  };

  const onPointerMove = (e: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el || !pos.current.isDown) return;
    const dx = e.clientX - pos.current.startX;
    el.scrollLeft = pos.current.scrollLeft - dx;
  };

  const onPointerEnd = (e: React.PointerEvent<HTMLDivElement>) => {
    const el = wrapRef.current;
    if (!el) return;
    pos.current.isDown = false;
    try {
      el.releasePointerCapture(e.pointerId);
    } catch (e) {
      logger.error('Failed to release pointer capture', e);
    }
  };

  return (
    <div className="flex flex-col gap-4 w-full">
      <Title {...props} />

      <div
        ref={wrapRef}
        className="w-full select-none max-tablet:overflow-x-auto no-scrollbar touch-pan-x md:cursor-grab"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerEnd}
        onPointerCancel={onPointerEnd}
        onPointerLeave={onPointerEnd}
      >
        <div className="flex flex-row gap-5 justify-start items-center px-2 w-max">
          <div className="text-sm font-medium text-center break-words text-neutral-400 shrink-0">
            {min_label ?? ''}
          </div>

          {Array.from(
            { length: (max_value ?? 0) - (min_value ?? 0) + 1 },
            (_, i) => {
              const val = (min_value ?? 0) + i;

              return (
                <div
                  key={`scale-${val}`}
                  className="flex flex-col gap-1 items-center w-8 shrink-0"
                >
                  <div className="text-sm font-medium text-neutral-400">
                    {val}
                  </div>
                  <div data-stop-drag>
                    <RadioButton
                      selected={selectedValue === val}
                      onClick={() => onSelect(val)}
                    />
                  </div>
                </div>
              );
            },
          )}

          <div className="text-sm font-medium text-center break-words text-neutral-400 shrink-0">
            {max_label ?? ''}
          </div>
        </div>
      </div>
    </div>
  );
}
