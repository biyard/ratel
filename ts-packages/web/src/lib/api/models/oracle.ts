export default interface Oracle {
  id: number;
  created_at: number;
  updated_at: number;
  user_id: number;
  oracle_type: OracleType;
}

export enum OracleType {
  Artist = 1,
  Gallery = 2,
  Collector = 3,
  Auction = 4,
}

export const OracleTypeMap: Record<OracleType, string> = {
  [OracleType.Artist]: 'Artist',
  [OracleType.Gallery]: 'Gallery',
  [OracleType.Collector]: 'Collector',
  [OracleType.Auction]: 'Auction',
};
