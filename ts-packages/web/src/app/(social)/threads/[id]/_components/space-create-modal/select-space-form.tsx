'use client';

import { Space, SpaceType } from '@/lib/api/models/spaces';
import { noticeSpaceCreateRequest } from '@/lib/api/models/notice';


import { Discuss, Palace, Mega, Vote } from '@/components/icons';
import { useState, useEffect } from 'react';

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

interface SpaceFormProps {
  type: SpaceType;
  Icon: React.JSX.Element;
  label: string;
  description: string;
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
    label: 'Poll',
    description: 'Collect quick opinions or preferences.',
  },
  {
    type: SpaceType.Notice,
    Icon: <Mega />,
    label: 'Notice',
    description: 'Post announcements or quizzes with optional point boosts',
  },
  {
    type: SpaceType.Deliberation,
    Icon: <Discuss />,
    label: 'Deliberation',
    description: 'Share perspectives and engage in in-depth discussion.',
  },
  {
    type: SpaceType.SprintLeague,
    Icon: <Palace className="[&>path]:stroke-[var(--color-neutral-500)]" />,
    label: 'Sprint League',
    description:
      'Mini social game where three runners compete in a race, and their speed is determined by community voting',
    experiment: true,
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
  }, [showConfigForm, popup]);

  const handleSend = async () => {
    if (!selectedType) return;

    // For all space types, first proceed with direct creation
    // This avoids immediate rendering of complex components like config form
    try {
      if (selectedType === SpaceType.Notice) {
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

  const handleSpaceTypeSelect = (type: SpaceType) => {
    setSelectedType(type);
  };

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

  // Show configuration form for Notice spaces
  if (showConfigForm && selectedType === SpaceType.Notice) {
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
        {SpaceForms.map((form) => (
          <SpaceForm
            key={form.type}
            form={form}
            selected={selectedType === form.type}
            onClick={() => handleSpaceTypeSelect(form.type)}
          />
        ))}
        <LoadablePrimaryButton
          className="w-full mt-4"
          disabled={!selectedType}
          onClick={handleSend}
          isLoading={isLoading}
        >
          Send
        </LoadablePrimaryButton>
      </div>
    </div>
  );
}

function SpaceForm({
  form,
  selected,
  onClick,
}: {
  form: SpaceFormProps;
  selected: boolean;
  onClick: () => void;
}) {
  const disabled = form.disabled || (form.experiment && !config.experiment);
  return (
    <div
      className={`flex flex-row gap-2.5 justify-center items-center w-full p-5 border rounded-[10px] ${selected ? 'border-primary' : 'border-neutral-800'} ${disabled ? 'opacity-50 cursor-not-allowed' : ''}} `}
      onClick={() => {
        if (!disabled) {
          onClick();
        }
      }}
    >
      <div className="size-8 [&>svg]:size-8">{form.Icon}</div>
      <div className="flex flex-col flex-1 gap-1">
        <span className="font-bold text-[15px]/[20px] text-white">
          {form.label}
        </span>
        <span className="font-normal text-[15px]/[24px] text-neutral-300">
          {form.description}
        </span>
      </div>
      <RadioButton selected={selected} onClick={onClick} />
    </div>
  );
}
