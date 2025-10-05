'use client';

import { createSpaceRequest, SpaceType } from '@/lib/api/models/spaces';
import { BoosterType } from '@/lib/api/models/notice';

import { Discuss, Palace, Mega, Vote } from '@/components/icons';
import { useState, useMemo, useCallback } from 'react';

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
  experiment?: boolean;
}

const SpaceForms: SpaceFormProps[] = [
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
    experiment: true,
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
    Icon: <Cube className="[&>path]:stroke-[var(--color-neutral-500)]" />,
    labelKey: 'dAgit.label',
    descKey: 'dAgit.desc',
    experiment: true,
  },
];

export default function SelectSpaceForm({ feed_id }: { feed_id: string }) {
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
    feedId: string;
    userIds: number[];
    startedAt: number | null;
    endedAt: number | null;
    boosterType: BoosterType | null;
  }) => {
    if (isLoading) return;
    setLoading(true);
    try {
      // FIXME: create space
      const req = createSpaceRequest(
        spaceType,
        Number(feedId),
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
    // Re-entrancy guard
    if (isLoading || selectedType === null) {
      return;
    }

    try {
      setLoading(true);

      if (isBoosterEnabled) {
        setShowConfigForm(true);
      } else {
        await handleCreateSpace({
          spaceType: selectedType,
          feedId: feed_id,
          userIds: [],
          startedAt: null,
          endedAt: null,
          boosterType: null,
        });
      }
    } catch (error) {
      logger.error('Error in handleSend:', error);
      showErrorToast('Failed to process request');
    } finally {
      setLoading(false);
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
        const disabled = form.experiment && !config.experiment;
        if (disabled) {
          return null;
        }
        return (
          <div
            className={`flex flex-row gap-2.5 justify-center items-center w-full p-5 border rounded-[10px] transition-colors
              ${selected ? 'border-primary' : 'border-modal-card-border'}
              ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-primary'}`}
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
      <div className=" max-mobile:w-full">
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
        <div className="flex flex-col w-full gap-2.5 p-1.5 max-mobile:h-[350px] overflow-y-auto">
          {renderedForms}
        </div>

        <div className="flex flex-row gap-2.5">
          <button
            type="button"
            onClick={() => {
              setSelectedType(null);
              popup.close();
            }}
            className="min-w-[50px] px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400 hover:text-white transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSend}
            disabled={isLoading || selectedType === null}
            className={`w-full py-[14.5px] font-bold text-base rounded-[10px] ${
              selectedType !== null && !isLoading
                ? 'bg-primary text-black hover:bg-primary/80'
                : 'bg-disabled-button-bg text-disabled-button-text cursor-not-allowed'
            } transition-colors`}
          >
            {isLoading ? 'Sending...' : 'Send'}
          </button>
        </div>
      </div>
    </div>
  );
}
