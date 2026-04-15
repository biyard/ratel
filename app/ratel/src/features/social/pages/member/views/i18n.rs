use super::*;

translate! {
    InviteMemberTranslate;

    select_group: {
        en: "Select the group",
        ko: "그룹 선택",
    },

    group_admin: {
        en: "Admin",
        ko: "관리자",
    },

    group_member: {
        en: "Member",
        ko: "멤버",
    },

    email_label: {
        en: "Email, Username, or Phone Number",
        ko: "이메일, 이름, 또는 핸드폰 번호",
    },

    email_hint: {
        en: "Enter email, username, or phone number (ex: john@example.com or john or 01012345678)",
        ko: "이메일, 이름 또는 핸드폰 번호를 입력해주세요. (ex: john@example.com or john or 01012345678)",
    },

    send: {
        en: "Send",
        ko: "전송",
    },

    searching: {
        en: "Searching...",
        ko: "검색 중...",
    },

    user_not_found: {
        en: "User not found",
        ko: "사용자를 찾을 수 없습니다",
    },

    failed_invite: {
        en: "Failed to invite members",
        ko: "멤버 초대에 실패했습니다",
    },

    already_added: {
        en: "is already added",
        ko: "이(가) 이미 추가되었습니다",
    },
}

translate! {
    MemberRowTranslate;

    team_owner: {
        en: "Team owner",
        ko: "팀 소유자",
    },
}

translate! {
    ViewerPageTranslate;

    no_permission: {
        en: "No permission",
        ko: "권한 없음",
    },

    no_permission_desc: {
        en: "You don't have permission to manage team members.",
        ko: "팀 멤버를 관리할 권한이 없습니다.",
    },

    team_prefix: {
        en: "team:",
        ko: "팀:",
    },
}

translate! {
    TeamMemberTranslate;

    members_label: { en: "Members", ko: "멤버" },
    team_management: { en: "Team Management", ko: "팀 관리" },
    members_subhead: { en: "members · manage access and roles", ko: "명 · 권한과 역할을 관리하세요" },
    add_member_btn: { en: "Add Member", ko: "멤버 추가" },

    filter_all: { en: "All", ko: "전체" },
    filter_owner: { en: "Owner", ko: "소유자" },
    filter_admin: { en: "Admin", ko: "관리자" },
    filter_member: { en: "Member", ko: "멤버" },

    role_owner: { en: "Owner", ko: "소유자" },
    role_admin: { en: "Admin", ko: "관리자" },
    role_member: { en: "Member", ko: "멤버" },

    search_placeholder: {
        en: "Search by name…",
        ko: "이름으로 검색…",
    },

    action_make_admin: { en: "Make Admin", ko: "관리자로 변경" },
    action_make_member: { en: "Make Member", ko: "멤버로 변경" },
    action_remove: { en: "Remove from team", ko: "팀에서 제거" },

    role_updated: { en: "Member role updated", ko: "역할이 변경되었습니다" },
    member_removed: { en: "Member removed", ko: "멤버가 제거되었습니다" },

    empty_title: { en: "No members yet", ko: "멤버가 없습니다" },
    empty_desc: {
        en: "Invite teammates to collaborate on proposals and governance.",
        ko: "팀원을 초대해 함께 활동해보세요.",
    },

    invite_title: { en: "Invite Member", ko: "멤버 초대" },
    invite_group_label: { en: "Group", ko: "그룹" },
    invite_role_admin: { en: "Admin — full management access", ko: "관리자 — 전체 관리 권한" },
    invite_role_member: { en: "Member — standard access", ko: "멤버 — 기본 권한" },
    invite_input_label: { en: "Email, name, or phone", ko: "이메일, 이름, 또는 전화번호" },
    invite_input_placeholder: {
        en: "e.g. john@example.com, sarah, 01012345678",
        ko: "예: john@example.com, sarah, 01012345678",
    },
    invite_hint: {
        en: "Separate multiple recipients with commas. Each will receive an invite.",
        ko: "여러 명은 콤마(,)로 구분해 입력하세요. 각각에게 초대장이 전송됩니다.",
    },
    invite_send: { en: "Send Invite", ko: "초대 보내기" },
    invite_sending: { en: "Sending…", ko: "보내는 중…" },
    invite_success: { en: "Invitation sent", ko: "초대를 보냈습니다" },
    invite_searching: { en: "Searching…", ko: "검색 중…" },
    invite_not_found: { en: "Not found:", ko: "찾을 수 없음:" },
    invite_already_added: { en: "Already added:", ko: "이미 추가됨:" },
}
