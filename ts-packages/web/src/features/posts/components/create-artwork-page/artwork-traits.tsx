import { useState } from 'react';
import { Plus, X } from 'lucide-react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { HexColorPicker } from 'react-colorful';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import {
  ArtworkTrait,
  ArtworkTraitDisplayType,
} from '@/features/posts/types/post-artwork';

// Default required traits that must be present in every artwork
// eslint-disable-next-line react-refresh/only-export-components
export const DEFAULT_REQUIRED_TRAITS: readonly string[] = [
  'artist_name',
  'creation_year',
  'dimensions',
  'medium',
  'background_color',
] as const;

// eslint-disable-next-line react-refresh/only-export-components
export const DEFAULT_TRAIT_CONFIGS: Record<
  string,
  { display_type: ArtworkTraitDisplayType; default_value: string | number }
> = {
  artist_name: {
    display_type: ArtworkTraitDisplayType.String,
    default_value: '',
  },
  creation_year: {
    display_type: ArtworkTraitDisplayType.Number,
    default_value: '',
  },
  dimensions: {
    display_type: ArtworkTraitDisplayType.String,
    default_value: '',
  },
  medium: {
    display_type: ArtworkTraitDisplayType.String,
    default_value: '',
  },
  background_color: {
    display_type: ArtworkTraitDisplayType.Color,
    default_value: '#ffffff',
  },
};

interface ArtworkTraitsProps {
  traits: ArtworkTrait[];
  onTraitAdd: (trait: ArtworkTrait) => void;
  onTraitUpdate: (index: number, trait: ArtworkTrait) => void;
  onTraitRemove: (index: number) => void;
  t: {
    add_trait: string;
    trait_type: string;
    trait_type_placeholder: string;
    trait_value: string;
    trait_value_placeholder: string;
    display_type: string;
    select_color: string;
    remove_trait: string;
  };
}

export function ArtworkTraits({
  traits,
  onTraitAdd,
  onTraitUpdate,
  onTraitRemove,
  t,
}: ArtworkTraitsProps) {
  const [isAdding, setIsAdding] = useState(false);
  const [newTrait, setNewTrait] = useState<ArtworkTrait>({
    trait_type: '',
    value: '',
    display_type: ArtworkTraitDisplayType.String,
  });

  const handleAddTrait = () => {
    if (newTrait.trait_type && newTrait.value) {
      onTraitAdd(newTrait);
      setNewTrait({
        trait_type: '',
        value: '',
        display_type: ArtworkTraitDisplayType.String,
      });
      setIsAdding(false);
    }
  };

  // Separate required and optional traits
  const requiredTraits = traits.filter((trait) =>
    DEFAULT_REQUIRED_TRAITS.includes(trait.trait_type),
  );
  const optionalTraits = traits.filter(
    (trait) => !DEFAULT_REQUIRED_TRAITS.includes(trait.trait_type),
  );

  return (
    <Col className="gap-4">
      <Row className="items-center justify-between">
        <h3 className="text-lg font-semibold text-text-primary">
          Artwork Traits
        </h3>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setIsAdding(!isAdding)}
          className="gap-2"
        >
          <Plus className="w-4 h-4" />
          {t.add_trait}
        </Button>
      </Row>

      {/* Required Traits */}
      {requiredTraits.length > 0 && (
        <Col className="gap-3">
          <div className="text-sm font-medium text-neutral-400">
            Required Metadata
          </div>
          {requiredTraits.map((trait) => {
            const originalIndex = traits.findIndex(
              (t) => t.trait_type === trait.trait_type,
            );
            return (
              <TraitItem
                key={trait.trait_type}
                trait={trait}
                index={originalIndex}
                onUpdate={(updatedTrait) =>
                  onTraitUpdate(originalIndex, updatedTrait)
                }
                onRemove={() => onTraitRemove(originalIndex)}
                isRequired={true}
                t={t}
              />
            );
          })}
        </Col>
      )}

      {/* Optional Traits */}
      {optionalTraits.length > 0 && (
        <Col className="gap-3">
          <div className="text-sm font-medium text-neutral-400">
            Additional Traits
          </div>
          {optionalTraits.map((trait) => {
            const originalIndex = traits.findIndex(
              (t) => t.trait_type === trait.trait_type,
            );
            return (
              <TraitItem
                key={originalIndex}
                trait={trait}
                index={originalIndex}
                onUpdate={(updatedTrait) =>
                  onTraitUpdate(originalIndex, updatedTrait)
                }
                onRemove={() => onTraitRemove(originalIndex)}
                isRequired={false}
                t={t}
              />
            );
          })}
        </Col>
      )}

      {/* Add New Trait Form */}
      {isAdding && (
        <Col className="gap-3 p-4 border border-input-box-border rounded-lg bg-input-box-bg">
          <Row className="gap-3">
            <div className="flex-1">
              <label className="text-sm font-medium text-text-primary mb-1 block">
                {t.trait_type}
              </label>
              <Input
                placeholder={t.trait_type_placeholder}
                value={newTrait.trait_type}
                onChange={(e) =>
                  setNewTrait({ ...newTrait, trait_type: e.target.value })
                }
              />
            </div>

            <div className="w-[200px]">
              <label className="text-sm font-medium text-text-primary mb-1 block">
                {t.display_type}
              </label>
              <Select
                value={newTrait.display_type || ArtworkTraitDisplayType.String}
                onValueChange={(value) =>
                  setNewTrait({
                    ...newTrait,
                    display_type: value as ArtworkTraitDisplayType,
                  })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value={ArtworkTraitDisplayType.String}>
                    Text
                  </SelectItem>
                  <SelectItem value={ArtworkTraitDisplayType.Number}>
                    Number
                  </SelectItem>
                  <SelectItem value={ArtworkTraitDisplayType.Color}>
                    Color
                  </SelectItem>
                </SelectContent>
              </Select>
            </div>
          </Row>

          <div className="flex-1">
            <label className="text-sm font-medium text-text-primary mb-1 block">
              {t.trait_value}
            </label>
            {newTrait.display_type === ArtworkTraitDisplayType.Color ? (
              <ColorPickerField
                value={String(newTrait.value)}
                onChange={(value) => setNewTrait({ ...newTrait, value })}
                t={t}
              />
            ) : (
              <Input
                type={
                  newTrait.display_type === ArtworkTraitDisplayType.Number
                    ? 'number'
                    : 'text'
                }
                placeholder={t.trait_value_placeholder}
                value={String(newTrait.value)}
                onChange={(e) =>
                  setNewTrait({ ...newTrait, value: e.target.value })
                }
              />
            )}
          </div>

          <Row className="gap-2 justify-end">
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                setIsAdding(false);
                setNewTrait({
                  trait_type: '',
                  value: '',
                  display_type: ArtworkTraitDisplayType.String,
                });
              }}
            >
              Cancel
            </Button>
            <Button
              variant="default"
              size="sm"
              onClick={handleAddTrait}
              disabled={!newTrait.trait_type || !newTrait.value}
            >
              Add
            </Button>
          </Row>
        </Col>
      )}
    </Col>
  );
}

interface TraitItemProps {
  trait: ArtworkTrait;
  index: number;
  onUpdate: (trait: ArtworkTrait) => void;
  onRemove: () => void;
  isRequired?: boolean;
  t: ArtworkTraitsProps['t'];
}

function TraitItem({
  trait,
  onUpdate,
  onRemove,
  isRequired = false,
  t,
}: TraitItemProps) {
  const formatTraitType = (traitType: string) => {
    return traitType
      .split('_')
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');
  };

  return (
    <div className="p-4 border border-input-box-border rounded-lg bg-input-box-bg">
      <Row className="items-start gap-3">
        <Col className="flex-1 gap-3">
          <Row className="gap-3 items-center">
            <div className="flex-1">
              <label className="text-sm font-medium text-text-primary mb-1 block">
                {formatTraitType(trait.trait_type)}
              </label>
            </div>
            <div className="flex items-center gap-2">
              {isRequired && (
                <span className="text-xs text-yellow-500 px-2 py-1 rounded bg-yellow-500/10 border border-yellow-500/20">
                  Required
                </span>
              )}
              <div className="text-xs text-neutral-400 px-2 py-1 rounded bg-neutral-700">
                {trait.display_type || 'string'}
              </div>
            </div>
          </Row>

          {trait.display_type === ArtworkTraitDisplayType.Color ? (
            <ColorPickerField
              value={String(trait.value)}
              onChange={(value) => onUpdate({ ...trait, value })}
              t={t}
            />
          ) : (
            <Input
              type={
                trait.display_type === ArtworkTraitDisplayType.Number
                  ? 'number'
                  : 'text'
              }
              value={String(trait.value)}
              onChange={(e) => onUpdate({ ...trait, value: e.target.value })}
            />
          )}
        </Col>

        {!isRequired && (
          <Button
            size="sm"
            onClick={onRemove}
            className="text-red-500 hover:text-red-600 hover:bg-red-500/10"
            aria-label={t.remove_trait}
          >
            <X className="w-4 h-4" />
          </Button>
        )}
      </Row>
    </div>
  );
}

interface ColorPickerFieldProps {
  value: string;
  onChange: (value: string) => void;
  t: Pick<ArtworkTraitsProps['t'], 'select_color'>;
}

function ColorPickerField({ value, onChange }: ColorPickerFieldProps) {
  const [showPicker, setShowPicker] = useState(false);
  const color = value || '#ffffff';

  return (
    <div className="relative">
      <Button
        type="button"
        onClick={() => setShowPicker(!showPicker)}
        className="w-full justify-start gap-3"
        style={{ backgroundColor: color }}
      >
        <div
          className="w-6 h-6 rounded border-2 border-white"
          style={{ backgroundColor: color }}
        />
        <span className="text-white drop-shadow-md">{color}</span>
      </Button>

      {showPicker && (
        <>
          <div
            className="fixed inset-0 z-10"
            onClick={() => setShowPicker(false)}
          />
          <div className="absolute z-20 mt-2 p-4 bg-card-bg border border-input-box-border rounded-lg shadow-lg">
            <HexColorPicker color={color} onChange={onChange} />
            <Button
              className="w-full mt-3"
              size="sm"
              onClick={() => setShowPicker(false)}
            >
              Done
            </Button>
          </div>
        </>
      )}
    </div>
  );
}
