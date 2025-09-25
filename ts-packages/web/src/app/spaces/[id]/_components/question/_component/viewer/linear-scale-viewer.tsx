'use client';
import RadioButton from '@/components/radio-button';
import { Answer } from '@/lib/api/models/response';
import { Question } from '@/lib/api/models/survey';
import React from 'react';
import Wrapper from './_components/wrapper';

export default function LinearScaleViewer({
  answerType,
  title,
  minLabel,
  minValue,
  maxLabel,
  maxValue,
  selected,
  isRequired,
  isCompleted,
  index,
  handleSelect,
}: {
  answerType: Answer['answer_type'];
  title: string;
  isRequired: boolean;
  minLabel?: string;
  minValue?: number;
  maxLabel?: string;
  maxValue?: number;
  selected: Answer;
  isCompleted: boolean;
  index: number;
  handleSelect: (
    qIdx: number,
    optionIdx: number,
    type: Question['answer_type'],
  ) => void;
}) {
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
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (_) {}
  };

  return (
    <div className="flex flex-col w-full gap-4">
      <Wrapper
        isRequired={isRequired}
        answerType={'linear_scale'}
        isMulti={false}
        title={title}
      />

      <div
        ref={wrapRef}
        className="w-full max-tablet:overflow-x-auto no-scrollbar touch-pan-x md:cursor-grab select-none"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerEnd}
        onPointerCancel={onPointerEnd}
        onPointerLeave={onPointerEnd}
      >
        <div className="flex flex-row justify-start gap-5 px-2 items-center w-max">
          <div className="w-10 text-center font-medium text-sm text-neutral-400 break-words shrink-0">
            {minLabel ?? ''}
          </div>

          {Array.from(
            { length: (maxValue ?? 0) - (minValue ?? 0) + 1 },
            (_, i) => {
              const val = (minValue ?? 0) + i;

              const answer =
                selected && selected.answer
                  ? Number(selected?.answer) + 1
                  : selected && selected.answer === 0
                    ? 1
                    : 0;
              const isChecked =
                answerType === 'linear_scale' && selected && answer === val;

              return (
                <div
                  key={`scale-${val}`}
                  className="flex flex-col items-center gap-1 w-8 shrink-0"
                >
                  <div className="text-sm text-neutral-400 font-medium">
                    {val}
                  </div>
                  <div data-stop-drag>
                    <RadioButton
                      selected={!!isChecked}
                      onClick={() =>
                        !isCompleted &&
                        handleSelect(index, val - 1, 'linear_scale')
                      }
                    />
                  </div>
                </div>
              );
            },
          )}

          <div className="w-10 text-center font-medium text-sm text-neutral-400 break-words shrink-0">
            {maxLabel ?? ''}
          </div>
        </div>
      </div>
    </div>
  );
}
