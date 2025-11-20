export type EmailOperation =
  | {
      SpacePostNotification: {
        author_profile: string;
        author_display_name: string;
        author_username: string;
        post_title: string;
        post_desc: string;
        connect_link: string;
      };
    }
  | {
      TeamInvite: {
        team_name: string;
        team_profile: string;
        team_display_name: string;
        url: string;
      };
    }
  | {
      SpaceInviteVerification: {
        space_title: string;
        space_desc: string;
        author_profile: string;
        author_display_name: string;
        author_username: string;
        cta_url: string;
      };
    }
  | {
      SignupSecurityCode: {
        display_name: string;
        code_1: string;
        code_2: string;
        code_3: string;
        code_4: string;
        code_5: string;
        code_6: string;
      };
    }
  | {
      StartSurvey: {
        space_title: string;
        survey_title: string;
        author_profile: string;
        author_display_name: string;
        author_username: string;
        connect_link: string;
      };
    };

export function getNotificationType(operation: EmailOperation): string {
  if ('SpacePostNotification' in operation) return 'SpacePostNotification';
  if ('TeamInvite' in operation) return 'TeamInvite';
  if ('SpaceInviteVerification' in operation) return 'SpaceInviteVerification';
  if ('SignupSecurityCode' in operation) return 'SignupSecurityCode';
  if ('StartSurvey' in operation) return 'StartSurvey';
  return 'Unknown';
}

export function getNotificationData(operation: EmailOperation): unknown {
  if ('SpacePostNotification' in operation)
    return operation.SpacePostNotification;
  if ('TeamInvite' in operation) return operation.TeamInvite;
  if ('SpaceInviteVerification' in operation)
    return operation.SpaceInviteVerification;
  if ('SignupSecurityCode' in operation) return operation.SignupSecurityCode;
  if ('StartSurvey' in operation) return operation.StartSurvey;
  return null;
}
