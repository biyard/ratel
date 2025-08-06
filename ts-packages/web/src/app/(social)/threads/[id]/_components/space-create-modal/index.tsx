import SelectSpaceForm from './select-space-form';

export default function SpaceCreateModal({ feed_id }: { feed_id: number }) {
  return (
    <div className="w-full max-w-[95vw] sm:max-w-[906px] sm:w-fit">
      <SelectSpaceForm feed_id={feed_id} />
    </div>
  );
}
