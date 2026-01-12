import { useEffect, useMemo, useState } from 'react';
import { TFunction } from 'i18next';
import { TopicRow } from '../../polls/types/topic-row';
import { Input } from '@/components/ui/input';
import { Edit1 } from '@/components/icons';
import { Save } from '@/components/icons';
import { PostEditor } from '@/features/posts/components/post-editor';

export type LdaTopicTableProps = {
  t: TFunction<'SpacePollAnalyze', undefined>;
  htmlContents?: string;
  ldaTopics?: TopicRow[];
  handleUpdateLda?: (
    topics: string[],
    keywords: string[][],
    htmlContents?: string,
  ) => void;
};

export function LdaTopicTable({
  t,
  ldaTopics,
  htmlContents,
  handleUpdateLda,
}: LdaTopicTableProps) {
  const rows = useMemo(() => {
    const map = new Map<string, string[]>();

    const list = Array.isArray(ldaTopics) ? ldaTopics : [];
    for (const r of list) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const topic = String((r as any)?.topic ?? '').trim();
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const kw = String((r as any)?.keyword ?? '').trim();
      if (!topic || !kw) continue;

      const arr = map.get(topic) ?? [];
      arr.push(kw);
      map.set(topic, arr);
    }

    const toOrder = (topic: string) => {
      const m = topic.match(/(\d+)/);
      return m ? Number(m[1]) : Number.POSITIVE_INFINITY;
    };

    return Array.from(map.entries())
      .map(([topic, kws]) => ({ topic, keywords: Array.from(new Set(kws)) }))
      .sort((a, b) => {
        const ao = toOrder(a.topic);
        const bo = toOrder(b.topic);
        if (ao !== bo) return ao - bo;
        return a.topic.localeCompare(b.topic);
      });
  }, [ldaTopics]);

  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState<Record<string, string>>({});
  const [content, setContent] = useState<string>(htmlContents ?? '');

  useEffect(() => {
    if (editing) return;
    setContent(htmlContents ?? '');
  }, [htmlContents, editing]);

  const startEdit = () => {
    const init: Record<string, string> = {};
    for (const r of rows) init[r.topic] = r.topic;
    setDraft(init);
    setContent(htmlContents ?? '');
    setEditing(true);
  };

  const save = () => {
    const used = new Set<string>();

    const topics: string[] = [];
    const keywords: string[][] = [];

    for (const r of rows) {
      const to = String(draft[r.topic] ?? r.topic).trim();
      if (!to) return;

      const key = to.toLowerCase();
      if (used.has(key)) return;
      used.add(key);

      topics.push(to);
      keywords.push(r.keywords);
    }

    handleUpdateLda?.(topics, keywords, content);
    setEditing(false);
  };

  return (
    <div className="w-full">
      <div className="mb-2 flex items-center justify-end gap-2">
        {!editing ? (
          <Edit1
            className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
            onClick={startEdit}
          />
        ) : (
          <Save
            className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
            onClick={save}
          />
        )}
      </div>

      <table className="overflow-hidden w-full text-sm rounded-xl border border-input-box-border">
        <thead className="bg-muted text-[var(--color-panel-table-header)]">
          <tr>
            <th className="py-3 px-4 text-left">{t('topic')}</th>
            <th className="py-3 px-4 text-left">{t('keywords')}</th>
          </tr>
        </thead>

        <tbody>
          {rows.map((r) => (
            <tr
              key={r.topic}
              className="border-t border-input-box-border hover:bg-muted/50"
            >
              <td className="py-3 px-4 font-medium text-left whitespace-nowrap">
                {!editing ? (
                  r.topic
                ) : (
                  <Input
                    type="text"
                    className="h-9 w-50 rounded border border-input-box-border bg-background px-2 text-sm text-text-primary"
                    value={draft[r.topic] ?? r.topic}
                    onChange={(e) =>
                      setDraft((m) => ({ ...m, [r.topic]: e.target.value }))
                    }
                    onBlur={() =>
                      setDraft((m) => ({
                        ...m,
                        [r.topic]: String(m[r.topic] ?? r.topic).trim(),
                      }))
                    }
                  />
                )}
              </td>

              <td className="py-3 px-4 text-left text-text-secondary">
                {r.keywords.join(', ')}
              </td>
            </tr>
          ))}

          {rows.length === 0 && (
            <tr>
              <td colSpan={2} className="py-8 text-center text-text-secondary">
                {t('no_topics')}
              </td>
            </tr>
          )}
        </tbody>
      </table>

      <PostEditor
        url={''}
        content={content}
        onUpdate={(next) => setContent(next)}
        disabledFileUpload={true}
        disabledImageUpload={true}
        editable={editing}
        showToolbar={editing}
      />
    </div>
  );
}
