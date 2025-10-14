import { useParams } from 'react-router';

export default function DiscussionPage() {
  const { spacePk, discussionPk } = useParams<{
    spacePk: string;
    discussionPk: string;
  }>();

  console.log('spacePK: ', spacePk, ', discussionPk: ', discussionPk);
  return (
    <div className="fixed top-0 left-0 flex flex-row w-full h-full bg-white text-black">
      discussion page
    </div>
  );
}
