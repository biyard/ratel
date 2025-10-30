import ArtNftResponse from '../dto/art-nft-response';

class ArtNft {
  pk: string;
  sk: string;

  constructor(data: ArtNftResponse) {
    this.pk = data.pk;
    this.sk = data.sk;
  }
}

export default ArtNft;
