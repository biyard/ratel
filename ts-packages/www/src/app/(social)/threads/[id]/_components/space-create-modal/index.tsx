import SelectSpaceForm from './select-space-form';

export default function SpaceCreateModal({ feed_id }: { feed_id: string }) {
  return (
    <div className="w-full max-w-[95vw] max-tablet:w-fit">
      <SelectSpaceForm feed_id={feed_id} />
    </div>
  );
}
