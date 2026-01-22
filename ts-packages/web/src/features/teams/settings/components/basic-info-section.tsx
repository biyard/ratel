import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';
import { TeamSettingsI18n } from '../i18n';

interface BasicInfoSectionProps {
  username: string;
  nickname: string;
  htmlContents: string;
  onNicknameChange: (evt: React.FormEvent<HTMLInputElement>) => void;
  onDescriptionChange: (evt: React.FormEvent<HTMLTextAreaElement>) => void;
  i18n: TeamSettingsI18n;
}

export function BasicInfoSection({
  username,
  nickname,
  htmlContents,
  onNicknameChange,
  onDescriptionChange,
  i18n,
}: BasicInfoSectionProps) {
  return (
    <>
      <Row className="max-tablet:flex-col">
        <label className="w-40 font-bold text-text-primary">
          {i18n.username}
        </label>
        <Input
          type="text"
          className="text-text-primary bg-input-box-bg border border-input-box-border"
          disabled
          value={`@${username}`}
          data-pw="team-username-display"
        />
      </Row>
      <Row className="max-tablet:flex-col">
        <label className="w-40 font-bold text-text-primary">
          {i18n.display_name}
        </label>
        <Input
          type="text"
          className="text-text-primary bg-input-box-bg border border-input-box-border"
          placeholder={i18n.display_name_hint}
          value={nickname}
          onInput={onNicknameChange}
          data-pw="team-nickname-input"
        />
      </Row>
      <Col>
        <label className="w-40 font-bold text-text-primary">
          {i18n.description}
        </label>
        <Textarea
          className="text-text-primary bg-input-box-bg border border-input-box-border"
          placeholder={i18n.team_description_hint}
          value={htmlContents}
          onChange={onDescriptionChange}
          data-pw="team-description-input"
        />
      </Col>
    </>
  );
}
