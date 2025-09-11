'use client';

import { createSpaceRequest, SpaceType } from '@/lib/api/models/spaces';
import { BoosterType } from '@/lib/api/models/notice';

import { Discuss, Palace, Mega, Vote } from '@/components/icons';
import { useState, useMemo, useCallback } from 'react';

import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { config } from '@/config';
import { route } from '@/route';
import { useRouter } from 'next/navigation';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import SpaceConfigForm from './space-config-form';
import RadioButton from '@/components/radio-button';
import { Cube } from '@/assets/icons/shopping';
import { useTranslations } from 'next-intl';
import { useSpaceMutation } from '@/hooks/use-space';
import { showErrorToast } from '@/lib/toast';
import { useSprintLeagueSpaceMutation } from '@/hooks/use-sprint-league';

interface SpaceFormProps {
  type: SpaceType;
  Icon: React.JSX.Element;
  labelKey: string;
  descKey: string;
  disabled?: boolean;
  experiment?: boolean;
}

const SpaceForms: SpaceFormProps[] = [
  // {
  //   type: SpaceType.Legislation,
  //   Icon: <Palace />,
  //   label: 'Legislation',
  //   description: 'Propose and decide on new rules or policies.',
  //   disabled: true,
  // },
  {
    type: SpaceType.Poll,
    Icon: <Vote />,
    labelKey: 'poll.label',
    descKey: 'poll.desc',
  },
  {
    type: SpaceType.Notice,
    Icon: <Mega />,
    labelKey: 'notice.label',
    descKey: 'notice.desc',
  },
  {
    type: SpaceType.Deliberation,
    Icon: <Discuss />,
    labelKey: 'deliberation.label',
    descKey: 'deliberation.desc',
  },
  {
    type: SpaceType.SprintLeague,
    Icon: <Palace className="[&>path]:stroke-[var(--color-neutral-500)]" />,
    labelKey: 'sprintLeague.label',
    descKey: 'sprintLeague.desc',
    experiment: true,
  },
  {
    type: SpaceType.dAgit,
    Icon: <Cube />,
    labelKey: 'dAgit.label',
    descKey: 'dAgit.desc',
    disabled: true,
  },
  // {
  //   type: SpaceType.Nft,
  //   Icon: <Cube />,
  //   label: 'NFT',
  //   description: 'Submit information to issue an NFT.',
  //   disabled: true,
  // },
];

export default function SelectSpaceForm({ feed_id }: { feed_id: number }) {
  const [isLoading, setLoading] = useState(false);
  const [selectedType, setSelectedType] = useState<SpaceType | null>(null);
  const [showConfigForm, setShowConfigForm] = useState(false);
  const router = useRouter();
  const popup = usePopup();

  const isBoosterEnabled =
    selectedType === SpaceType.Poll ||
    selectedType === SpaceType.Notice ||
    selectedType === SpaceType.SprintLeague;

  const {
    create: { mutateAsync },
  } = useSpaceMutation();

  const {
    create: { mutateAsync: mutateSprintAsync },
  } = useSprintLeagueSpaceMutation();

  const handleCreateSpace = async ({
    spaceType,
    feedId,
    userIds = [],
    startedAt = null,
    endedAt = null,
    boosterType = null,
  }: {
    spaceType: SpaceType;
    feedId: number;
    userIds: number[];
    startedAt: number | null;
    endedAt: number | null;
    boosterType: BoosterType | null;
  }) => {
    if (isLoading) return;
    setLoading(true);
    try {
      const req = createSpaceRequest(
        spaceType,
        feedId,
        userIds,
        0,
        startedAt,
        endedAt,
        boosterType,
      );
      let spaceId = 0;
      if (spaceType === SpaceType.SprintLeague) {
        const space = await mutateSprintAsync({
          spaceReq: req,
        });
        spaceId = space.id;
      } else {
        const space = await mutateAsync(req);
        spaceId = space.id;
      }

      router.push(route.space(spaceId));
      popup.close();
    } catch {
      logger.error('Error creating space');
      showErrorToast('Failed to create space');
    } finally {
      setLoading(false);
    }
  };

  const handleSend = async () => {
    if (!selectedType) return;

    if (isBoosterEnabled) {
      setShowConfigForm(true);
    } else {
      try {
        await handleCreateSpace({
          spaceType: selectedType,
          feedId: feed_id,
          userIds: [],
          startedAt: null,
          endedAt: null,
          boosterType: null,
        });
      } catch (error) {
        logger.error('Error handling space creation:', error);
      }
    }
  };

  const handleSpaceTypeSelect = useCallback((type: SpaceType) => {
    setSelectedType(type);
  }, []);

  const handleBackToSelection = () => {
    setShowConfigForm(false);
  };

  const SpaceForm = useMemo(
    () =>
      function SpaceFormInner({
        form,
        selected,
        onClick,
      }: {
        form: SpaceFormProps;
        selected: boolean;
        onClick: () => void;
      }) {
        const tt = useTranslations('SpaceForms');
        const disabled =
          form.disabled || (form.experiment && !config.experiment);

        return (
          <div
            className={`flex flex-row gap-2.5 justify-center items-center w-full p-5 border rounded-[10px] bg-modal-card-bg ${
              selected ? 'border-primary' : 'border-modal-card-border'
            } ${disabled ? 'opacity-50 cursor-not-allowed' : ''} `}
            onClick={() => {
              if (!disabled) onClick();
            }}
          >
            <div className="size-8 [&>svg]:size-8">{form.Icon}</div>
            <div className="flex flex-col flex-1 gap-1">
              <span className="font-bold text-[15px]/[20px] text-text-primary">
                {tt(form.labelKey)}
              </span>
              <span className="font-normal text-[15px]/[24px] text-desc-text">
                {tt(form.descKey)}
              </span>
            </div>
            <RadioButton selected={selected} onClick={onClick} />
          </div>
        );
      },
    [],
  );

  const renderedForms = useMemo(
    () =>
      SpaceForms.map((form) => (
        <SpaceForm
          key={form.type}
          form={form}
          selected={selectedType === form.type}
          onClick={() => handleSpaceTypeSelect(form.type)}
        />
      )),
    [selectedType, handleSpaceTypeSelect, SpaceForm],
  );

  // Show configuration form for Notice spaces
  if (showConfigForm && !!selectedType) {
    return (
      <div className="mobile:w-[906px] max-mobile:w-full">
        {/* Use React.lazy or this conditional rendering pattern to 
            ensure the component only renders after initial render */}
        {showConfigForm && (
          <SpaceConfigForm
            spaceType={selectedType}
            onBack={handleBackToSelection}
            onConfirm={(startedAt, endedAt, boosterType) => {
              return handleCreateSpace({
                spaceType: selectedType,
                feedId: feed_id,
                userIds: [],
                startedAt,
                endedAt,
                boosterType,
              });
            }}
            isLoading={isLoading}
          />
        )}
      </div>
    );
  }

  // Show space type selection
  return (
    <div className="mobile:w-[400px] max-mobile:w-full">
      <div className="flex flex-col gap-2.5 p-1.5">
        {renderedForms}
        <LoadablePrimaryButton
          className="w-full mt-4"
          disabled={!selectedType}
          onClick={handleSend}
          isLoading={isLoading}
        >
          {isBoosterEnabled ? 'Next' : 'Create Space'}
        </LoadablePrimaryButton>
      </div>
    </div>
  );
}
