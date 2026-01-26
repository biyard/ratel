import { useMemo, useState, useRef, useEffect } from 'react';
import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceBoardsCreateController } from './space-boards-create-controller';
import { Input } from '@/components/ui/input';
import { PostEditor } from '@/features/posts/components/post-editor';
import { Button } from '@/components/ui/button';
import { TimeRangeSetting } from '@/features/spaces/polls/components/time-range-setting';

export function SpaceBoardsCreatePage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceBoardsCreatePage: spacePk=${spacePk}`);
  const ctrl = useSpaceBoardsCreateController(spacePk);
  const t = ctrl.t;

  const isValid = useMemo(() => {
    const title = ctrl.title.get();
    const category = ctrl.categoryName.get();
    const html = ctrl.htmlContents.get();
    return (
      title.trim().length > 0 &&
      category.trim().length > 0 &&
      html.trim().length > 0
    );
  }, [ctrl.title.get(), ctrl.categoryName.get(), ctrl.htmlContents.get()]);

  const [open, setOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(-1);
  const rootRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const q = ctrl.categoryName.get();
  const categories = ctrl.category?.categories ?? [];

  const filtered = useMemo(() => {
    const s = q.trim().toLowerCase();
    const set = new Set(categories.map((c) => c.trim()));
    const base = categories.filter((c) => c.toLowerCase().includes(s));
    if (s.length > 0 && !set.has(q.trim())) return [...base, q.trim()];
    return base;
  }, [q, JSON.stringify(categories)]);

  useEffect(() => {
    const h = (e: MouseEvent) => {
      if (!rootRef.current) return;
      if (!rootRef.current.contains(e.target as Node)) setOpen(false);
    };
    window.addEventListener('mousedown', h);
    return () => window.removeEventListener('mousedown', h);
  }, []);

  return (
    <div className="flex flex-col w-full mt-6 gap-2">
      <TimeRangeSetting
        canEdit={false}
        onChange={ctrl.handleTimeRange}
        startTimestampMillis={ctrl.startedAt.get()}
        endTimestampMillis={ctrl.endedAt.get()}
        alwaysEdit={true}
        className="justify-end"
      />
      <div className="w-full">
        <div className="grid gap-5 w-full">
          <div className="w-full">
            <Input
              variant="post"
              data-testid="board-title-input"
              placeholder={t('title_hint')}
              showRequiredMarker
              value={ctrl.title.get()}
              onChange={(e) => {
                const value = e.target.value;
                if (value.length <= 80) {
                  ctrl.handleTitle(value);
                }
              }}
              maxLength={80}
            />
            <div className="flex justify-end mt-1 text-sm text-[var(--color-post-input-counter)]">
              {ctrl.title.get().length}/80
            </div>
          </div>

          <div>
            <div ref={rootRef} className="relative">
              <Input
                variant="post"
                showRequiredMarker
                className="rounded-[100px]"
                data-testid="board-category-input"
                ref={inputRef}
                value={ctrl.categoryName.get()}
                onChange={(e) => {
                  ctrl.handleCategoryName(e.target.value);
                  setHighlightedIndex(-1);
                  setOpen(true);
                }}
                onFocus={() => setOpen(true)}
                onKeyDown={(e) => {
                  if (!open && (e.key === 'ArrowDown' || e.key === 'Enter')) {
                    setOpen(true);
                    return;
                  }
                  if (e.key === 'ArrowDown') {
                    e.preventDefault();
                    setHighlightedIndex((p) =>
                      Math.min(p + 1, filtered.length - 1),
                    );
                  } else if (e.key === 'ArrowUp') {
                    e.preventDefault();
                    setHighlightedIndex((p) => Math.max(p - 1, 0));
                  } else if (e.key === 'Enter') {
                    if (
                      highlightedIndex >= 0 &&
                      highlightedIndex < filtered.length
                    ) {
                      ctrl.handleCategoryName(filtered[highlightedIndex]);
                      setOpen(false);
                    } else if (q.trim().length > 0) {
                      ctrl.handleCategoryName(q.trim());
                      setOpen(false);
                    }
                  } else if (e.key === 'Escape') {
                    setOpen(false);
                  }
                }}
                placeholder={t('category_hint')}
                aria-autocomplete="list"
                aria-expanded={open}
                aria-controls="category-popover"
              />

              <button
                type="button"
                onMouseDown={(e) => e.preventDefault()}
                onClick={() => {
                  if (open) setOpen(false);
                  else {
                    inputRef.current?.focus();
                    setOpen(true);
                  }
                }}
                className="absolute right-2 top-1/2 -translate-y-1/2 h-6 w-6 flex items-center justify-center text-neutral-400 hover:text-neutral-200 focus:outline-none"
                aria-label="toggle category"
              >
                <svg
                  viewBox="0 0 20 20"
                  fill="currentColor"
                  className="h-5 w-5"
                >
                  <path
                    fillRule="evenodd"
                    d="M5.23 7.21a.75.75 0 011.06.02L10 10.94l3.71-3.71a.75.75 0 111.06 1.06l-4.24 4.25a.75.75 0 01-1.06 0L5.21 8.29a.75.75 0 01.02-1.08z"
                    clipRule="evenodd"
                  />
                </svg>
              </button>

              {open && filtered?.length > 0 && (
                <div
                  id="category-popover"
                  className="absolute z-20 mt-2 w-full rounded-xl border border-[var(--color-dropdown-border)] bg-[var(--color-dropdown-bg)] shadow-xl overflow-hidden"
                >
                  {filtered.map((c, idx) => {
                    const active = c.trim() === ctrl.categoryName.get().trim();
                    const hovered = idx === highlightedIndex;
                    return (
                      <button
                        key={c + idx}
                        type="button"
                        onMouseEnter={() => setHighlightedIndex(idx)}
                        onMouseLeave={() => setHighlightedIndex(-1)}
                        onClick={() => {
                          ctrl.handleCategoryName(c);
                          setOpen(false);
                          inputRef.current?.focus();
                        }}
                        className={[
                          'w-full text-left px-4 py-2 text-sm',
                          hovered
                            ? 'bg-[var(--color-dropdown-item-hover-bg)]'
                            : '',
                          active
                            ? 'text-[var(--color-dropdown-text)]'
                            : 'text-[var(--color-dropdown-text)]',
                        ].join(' ')}
                      >
                        {c}
                      </button>
                    );
                  })}
                </div>
              )}
            </div>
          </div>

          <div>
            <PostEditor
              variant="post"
              placeholder={t('contents_hint')}
              files={ctrl.files.get()}
              content={ctrl.htmlContents.get()}
              onUpdate={async (nextContent) => {
                try {
                  await ctrl.handleContent(nextContent);
                } catch (error) {
                  logger.error('Failed to handle content update', { error });
                }
              }}
              editable
              showToolbar
              uploadAsset={ctrl.uploadAsset}
              onImageUpload={ctrl.handleImageUpload}
              onUploadPDF={ctrl.handlePdfUpload}
              onRemovePdf={ctrl.handleRemovePdf}
              onRemoveImage={ctrl.handleRemoveImage}
              url={ctrl.image.get()}
              disabledFileUpload={false}
              data-pw="space-board-content-editor"
            />
          </div>

          <div className="mt-2 flex items-center justify-end gap-3">
            <Button variant="default" onClick={ctrl.handleCancel}>
              {t('cancel')}
            </Button>
            <Button
              variant="primary"
              disabled={!isValid}
              onClick={async () => {
                if (!isValid) return;
                await ctrl.handleSubmit();
              }}
              data-testid="board-btn-submit"
            >
              {ctrl.postPk.get() ? t('update') : t('write')}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
