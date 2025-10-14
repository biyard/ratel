import SpaceTypeSelectForm from './space-type-select-form';

export default function SpaceCreateModal({ feed_id }: { feed_id: string }) {
  return (
    <div className="w-full max-w-[95vw] max-tablet:w-fit">
      <SpaceTypeSelectForm feed_id={feed_id} />
    </div>
  );
}
