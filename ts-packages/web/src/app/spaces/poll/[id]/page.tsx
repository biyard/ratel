import { useParams } from 'react-router';

export default function PollSpacePage() {
  const { spaceId } = useParams<{ spaceId: string }>();
  return <div>Poll Space ID: {spaceId}</div>;
}
