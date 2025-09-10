'use client';

import { Space, SpaceType } from '@/lib/api/models/spaces';
import { noticeSpaceCreateRequest } from '@/lib/api/models/notice';

import { Discuss, Palace, Mega, Vote } from '@/components/icons';
import { useState, useEffect, useMemo, useCallback } from 'react';

import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { route } from '@/route';
import { useRouter } from 'next/navigation';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import SpaceConfigForm from './space-config-form';
import RadioButton from '@/components/radio-button';
import { Cube } from '@/assets/icons/shopping';
import { useTranslations } from 'next-intl';

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
    Icon: <Cube className="[&>path]:stroke-[var(--color-neutral-500)]" />,
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
  const t = useTranslations('SpaceForms');

  // Update popup title based on current form state
  useEffect(() => {
    // Add a small delay to prevent rapid state changes when modal opens
    const timeoutId = setTimeout(() => {
      if (showConfigForm) {
        // Don't set a title for config form - it has its own header
        // Also disable the close button to remove the X icon
        popup.withTitle('').withoutClose();
      } else {
        popup.withTitle('Select a Space Type');
      }
    }, 10);

    return () => clearTimeout(timeoutId);
  }, [showConfigForm, popup, t]);

  const handleSend = async () => {
    if (!selectedType) return;

    // For all space types, first proceed with direct creation
    // This avoids immediate rendering of complex components like config form
    try {
      if (
        selectedType === SpaceType.Notice ||
        selectedType === SpaceType.SprintLeague
      ) {
        // For Notice space, we'll show config form after a small delay
        // This prevents the Maximum update depth exceeded error
        setTimeout(() => {
          setShowConfigForm(true);
        }, 10);
      } else {
        // For other space types, proceed directly with creation
        await handleCreateSpace(selectedType);
      }
    } catch (error) {
      logger.error('Error handling space creation:', error);
    }
  };

  const handleSpaceTypeSelect = useCallback((type: SpaceType) => {
    setSelectedType(type);
  }, []);

  const handleCreateSpace = async (spaceType: SpaceType) => {
    setLoading(true);
    try {
      // For non-Notice spaces, all special fields are null
      const res = await apiFetch<Space>(
        `${config.api_url}${ratelApi.spaces.createSpace()}`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(
            noticeSpaceCreateRequest(
              spaceType,
              feed_id,
              [],
              0,
              null,
              null,
              null,
            ),
          ),
        },
      );
      if (res.data) {
        logger.debug('Space created successfully:', res.data.id);
        if (res.data.space_type === SpaceType.Deliberation) {
          router.push(route.deliberationSpaceById(res.data.id));
        }
        popup.close();
      }
    } catch (error) {
      logger.error('Error creating space:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleConfigConfirm = () => {
    // Space creation is handled in the config form
    // Just close the modal and reset state
    popup.close();
  };

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
            className={`flex flex-row gap-2.5 justify-center items-center w-full p-5 border rounded-[10px] ${selected ? 'border-primary' : 'border-neutral-800'
              } ${disabled ? 'opacity-50 cursor-not-allowed' : ''} `}
            onClick={() => {
              if (!disabled) onClick();
            }}
          >
            <div className="size-8 [&>svg]:size-8">{form.Icon}</div>
            <div className="flex flex-col flex-1 gap-1">
              <span className="font-bold text-[15px]/[20px] text-white">
                {tt(form.labelKey)}
              </span>
              <span className="font-normal text-[15px]/[24px] text-neutral-300">
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
            feedId={feed_id}
            onBack={handleBackToSelection}
            onConfirm={handleConfigConfirm}
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
        {/* <LoadablePrimaryButton
          className="w-full mt-4"
          disabled={!selectedType}
          onClick={handleSend}
          isLoading={isLoading}
        >
          Send
        </LoadablePrimaryButton> */}

        <div className="flex flex-row gap-2.5">
        <button
          // onClick={onClose}
          className="min-w-50 px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400 hover:text-white transition-colors"
        >
          Cancel
        </button>
        <button
          onClick={handleSend}
          disabled={!selectedType}
          className={`w-full py-[14.5px] font-bold text-base rounded-[10px] ${selectedType && !isLoading
              ? 'bg-primary text-black hover:bg-primary/80'
              : 'bg-neutral-700 text-neutral-500 cursor-not-allowed'
            } transition-colors`}
        >
          {isLoading ? 'Sending...' : 'Send'}
        </button>
        </div>
      </div>
    </div>
  );
}
