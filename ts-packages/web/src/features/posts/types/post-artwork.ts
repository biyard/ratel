export interface ArtworkTrait {
  trait_type: string;
  value: string | number | boolean | Record<string, unknown> | null;
  display_type?: ArtworkTraitDisplayType | null;
}

export const ArtworkTraitDisplayType = {
  String: 'string',
  Color: 'color',
  Number: 'number',
} as const;

export type ArtworkTraitDisplayType =
  (typeof ArtworkTraitDisplayType)[keyof typeof ArtworkTraitDisplayType];

export interface ArtworkMetadata {
  traits: ArtworkTrait[];
}
