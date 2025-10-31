import { useTranslation } from 'react-i18next';
import {
  I18nCreatePostPage,
  useCreatePostPageI18n,
} from '../create-post-page/i18n';

export const CreateArtworkPage = {
  en: {
    page_title: 'Create Artwork',
    title_placeholder: 'Artwork Name',
    content_placeholder: 'Describe about artwork',
    attributes: 'Attributes',
    artist_name_label: 'Artist Name',
    artist_name_placeholder: 'Enter artist name',
    creation_year_label: 'Creation Year',
    creation_year_placeholder: 'e.g., 2024',
    dimensions_label: 'Dimensions',
    dimensions_placeholder: 'e.g., 100x150cm',
    medium_label: 'Medium',
    medium_placeholder: 'e.g., Oil on canvas',
    add_trait: 'Add Trait',
    trait_type: 'Trait Type',
    trait_type_placeholder: 'e.g., Edition, Rarity',
    trait_value: 'Value',
    trait_value_placeholder: 'Enter value',
    display_type: 'Display Type',
    select_color: 'Select Color',
    remove_trait: 'Remove Trait',
    image_label: 'Select the Artwork Image',
    image_formats: 'JPG, PNG',
    max_resolution: 'Maximum resolution: 3000px x 3000px (square format)',
    max_file_size: 'Uploads are allowed under 10MB',
  },
  ko: {
    page_title: '아트워크 작성',
    title_placeholder: '작품명',
    content_placeholder: '작품에 대한 설명',
    attributes: '속성',
    artist_name_label: '작가명',
    artist_name_placeholder: '작가명을 입력하세요',
    creation_year_label: '제작년도',
    creation_year_placeholder: '예: 2024',
    dimensions_label: '크기',
    dimensions_placeholder: '예: 100x150cm',
    medium_label: '재료/기법',
    medium_placeholder: '예: 캔버스에 유화',
    add_trait: '속성 추가',
    trait_type: '속성 타입',
    trait_type_placeholder: '예: 에디션, 희귀도',
    trait_value: '값',
    trait_value_placeholder: '값을 입력하세요',
    display_type: '표시 타입',
    select_color: '색상 선택',
    remove_trait: '속성 제거',
    image_label: '작품 이미지 선택',
    image_formats: 'JPG, PNG',
    max_resolution: '최대 해상도: 3000px x 3000px (정사각형 포맷)',
    max_file_size: '10MB 이하 업로드 가능',
  },
};

export interface I18nCreateArtworkPage extends I18nCreatePostPage {
  attributes: string;
  artist_name_label: string;
  artist_name_placeholder: string;
  creation_year_label: string;
  creation_year_placeholder: string;
  dimensions_label: string;
  dimensions_placeholder: string;
  medium_label: string;
  medium_placeholder: string;
  add_trait: string;
  trait_type: string;
  trait_type_placeholder: string;
  trait_value: string;
  trait_value_placeholder: string;
  display_type: string;
  select_color: string;
  remove_trait: string;
  image_label: string;
  image_formats: string;
  max_resolution: string;
  max_file_size: string;
}

export function useCreateArtworkPageI18n(): I18nCreateArtworkPage {
  const post = useCreatePostPageI18n();
  const { t } = useTranslation();

  return {
    ...post,
    page_title: t('CreateArtworkPage:page_title'),
    attributes: t('CreateArtworkPage:attributes'),
    artist_name_label: t('CreateArtworkPage:artist_name_label'),
    artist_name_placeholder: t('CreateArtworkPage:artist_name_placeholder'),
    creation_year_label: t('CreateArtworkPage:creation_year_label'),
    creation_year_placeholder: t('CreateArtworkPage:creation_year_placeholder'),
    dimensions_label: t('CreateArtworkPage:dimensions_label'),
    dimensions_placeholder: t('CreateArtworkPage:dimensions_placeholder'),
    medium_label: t('CreateArtworkPage:medium_label'),
    medium_placeholder: t('CreateArtworkPage:medium_placeholder'),
    add_trait: t('CreateArtworkPage:add_trait'),
    trait_type: t('CreateArtworkPage:trait_type'),
    trait_type_placeholder: t('CreateArtworkPage:trait_type_placeholder'),
    trait_value: t('CreateArtworkPage:trait_value'),
    trait_value_placeholder: t('CreateArtworkPage:trait_value_placeholder'),
    display_type: t('CreateArtworkPage:display_type'),
    select_color: t('CreateArtworkPage:select_color'),
    remove_trait: t('CreateArtworkPage:remove_trait'),
    image_label: t('CreateArtworkPage:image_title'),
    image_formats: t('CreateArtworkPage:image_formats'),
    max_resolution: t('CreateArtworkPage:max_resolution'),
    max_file_size: t('CreateArtworkPage:max_file_size'),
  };
}
