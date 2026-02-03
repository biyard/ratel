export interface TeamGroup {
  id: string; // Updated from 'sk' - just the UUID, not the full EntityType
  name: string;
  description: string;
  members: number;
  permissions: number;
}
