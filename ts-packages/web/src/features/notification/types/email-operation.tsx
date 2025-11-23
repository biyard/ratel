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

// Since Rust uses #[serde(untagged)], the data comes as a flat object
// We need to detect the type based on which fields are present
export function getNotificationType(operation: EmailOperation): string {
  const op = operation as Record<string, unknown>;

  // Check for SpacePostNotification (has post_title)
  if ('post_title' in op && 'post_desc' in op) {
    return 'SpacePostNotification';
  }

  // Check for TeamInvite (has team_name)
  if ('team_name' in op && 'team_display_name' in op) {
    return 'TeamInvite';
  }

  // Check for StartSurvey (has survey_title)
  if ('survey_title' in op && 'space_title' in op) {
    return 'StartSurvey';
  }

  // Check for SpaceInviteVerification (has space_desc and cta_url)
  if ('space_desc' in op && 'cta_url' in op) {
    return 'SpaceInviteVerification';
  }

  // Check for SignupSecurityCode (has code_1)
  if ('code_1' in op && 'display_name' in op) {
    return 'SignupSecurityCode';
  }

  return 'Unknown';
}

export function getNotificationData(operation: EmailOperation): unknown {
  // Since the data is untagged, the operation object IS the data
  return operation as Record<string, unknown>;
}
