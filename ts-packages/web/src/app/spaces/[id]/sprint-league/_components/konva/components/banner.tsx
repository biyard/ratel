import { Image as KonvaImage } from 'react-konva';
import useImage from 'use-image';

const BannerImage = '/images/sprint_league/banner.png';
export function Banner({ y }: { y: number }) {
  const [image] = useImage(BannerImage);
  return <KonvaImage image={image} x={0} y={y} />;
}

const BannerVoteImage = '/images/sprint_league/banner_vote.png';

export function BannerVote({ y }: { y: number }) {
  const [image] = useImage(BannerVoteImage);
  return <KonvaImage image={image} x={0} y={y} />;
}

const BannerAfterVoteImage = '/images/sprint_league/banner_after_vote.png';
export function BannerAfterVote({ y }: { y: number }) {
  const [image] = useImage(BannerAfterVoteImage);
  return <KonvaImage image={image} x={0} y={y} />;
}
