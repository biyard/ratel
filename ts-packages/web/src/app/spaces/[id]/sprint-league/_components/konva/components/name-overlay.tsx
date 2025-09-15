import { Group, Text } from 'react-konva';
import { BannerAfterVote } from './banner';

export default function PlayerNameOverlay({
  names = ['Rank 1', 'Rank 2', 'Rank 3'],
}: {
  names?: string[];
}) {
  return (
    <Group>
      <Group>
        <BannerAfterVote y={0} />
        <Text
          text={names[0].slice(0, 8)}
          x={110}
          y={121}
          fontFamily="Raleway"
          fontSize={30}
          fontStyle="900"
          align="center"
          fill="#fcb300"
          listening={false}
        />
      </Group>

      <Text
        text={names[1].slice(0, 8)}
        x={200}
        y={424}
        fontFamily="Raleway"
        fontSize={25}
        fontStyle="900"
        align="center"
        fill="#fcb300"
        listening={false}
      />
      <Text
        text={names[2].slice(0, 8)}
        x={200}
        y={516}
        fontFamily="Raleway"
        fontSize={25}
        fontStyle="900"
        align="center"
        fill="#fcb300"
        listening={false}
      />
    </Group>
  );
}
