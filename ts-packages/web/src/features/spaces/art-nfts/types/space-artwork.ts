import SpaceArtworkResponse from '../dto/space-artwork-response';

export default class SpaceArtwork {
  public pk: string;
  public sk: string;
  public createdAt: number;
  public updatedAt: number;
  public contractAddress: string;
  public metadataUri: string;
  public metadata: string;

  constructor(data: SpaceArtworkResponse) {
    this.pk = data.pk;
    this.sk = data.sk;
    this.createdAt = data.created_at;
    this.updatedAt = data.updated_at;
    this.contractAddress = data.contract_address;
    this.metadataUri = data.metadata_uri;
    this.metadata = data.metadata;
  }
}
