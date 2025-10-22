import Card from '@/components/card';
import { AddDiscussion } from './add-discussion-button';
import { TFunction } from 'i18next';

export default function DiscussionEditor({
  t,
  canEdit,
  onadd,
}: {
  t: TFunction<'SpaceDiscussionEditor', undefined>;
  canEdit: boolean;
  onadd: () => void;
}) {
  //   const [editable, setEditable] = useState(false);
  //   const [content, setContent] = useState(htmlContent);

  //   if (!canEdit) {
  //     return <TextViewer htmlContent={htmlContent} />;
  //   }

  //   const onClick = () => {
  //     if (editable) {
  //       onContentChange(content);
  //     }
  //     setEditable(!editable);
  //   };
  //   const onKeyDown = (e: React.KeyboardEvent) => {
  //     if (!editable) return;
  //     executeOnKeyStroke(
  //       e,
  //       () => {
  //         onContentChange(content);
  //         setEditable(false);
  //       },
  //       () => setEditable(false),
  //     );
  //   };

  return (
    <>
      <Card>
        <div className="flex flex-col w-full gap-5">
          <div className="flex flex-row w-full justify-between items-center">
            <div className="font-bold text-text-primary text-[15px]/[20px]">
              {t('discussions')}
            </div>

            {canEdit && <AddDiscussion onadd={onadd} />}
          </div>
        </div>
      </Card>
    </>
  );
}
