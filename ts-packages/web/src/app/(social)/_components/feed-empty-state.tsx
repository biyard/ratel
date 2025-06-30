type FeedEmptyStateProps = {
  message?: string;
};

export default function FeedEmptyState({
  message = 'Feeds data is empty',
}: FeedEmptyStateProps) {
  return (
    <div
      className="flex flex-row w-full h-fit justify-start items-center px-4 py-5 border border-gray-500 rounded-lg font-medium text-base text-gray-500"
      aria-label="Empty feed state"
    >
      {message}
    </div>
  );
}
