'use client';

import { DeliberationSpaceResponse } from '@/features/deliberation-space/utils/deliberation.spaces.v3';
import SpaceDiscussion from '../space-discussion';
import SpaceElearning from '../space-elearning';
import { TFunction } from 'i18next';
import { Deliberation } from '../../types/deliberation-type';

export type DeliberationPageProps = {
  t: TFunction<'DeliberationSpace', undefined>;
  space: DeliberationSpaceResponse;
  deliberation: Deliberation;
  setDeliberation: (deliberation: Deliberation) => void;
  handleViewRecord: (discussionPk: string, record: string) => void;
};

export default function DeliberationPage({
  space,
  deliberation,
  setDeliberation,
  handleViewRecord,
}: DeliberationPageProps) {
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceDiscussion
          space={space}
          deliberation={deliberation}
          setDeliberation={setDeliberation}
          handleViewRecord={handleViewRecord}
        />
        <SpaceElearning
          deliberation={deliberation}
          setDeliberation={setDeliberation}
        />
      </div>
    </div>
  );
}
