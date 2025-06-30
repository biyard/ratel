import SelectSpaceForm from './select-space-form';

export default function SpaceCreateModal({ feed_id }: { feed_id: number }) {
  return (
    <div className="mobile:w-[400px] max-mobile:w-full ">
      <SelectSpaceForm feed_id={feed_id} />
    </div>
  );
}
