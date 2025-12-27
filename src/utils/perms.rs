use bitflags::bitflags;

bitflags! {
    pub struct Permission: u32 {
        const NONE                 = 0;
        const MODERATE_POSTS       = 1 << 0;
        const MODERATE_COMMENTS    = 1 << 1;
        const MODERATE_PROFILES    = 1 << 2;
        const BAN_USERS            = 1 << 3;
        const RED_BUTTON           = 1 << 4;
        const REVIEW_APPELLATIONS  = 1 << 5;
        const ADMIN_PANEL          = 1 << 6;
    }
}

/// User permissions by role
/// 0 -> User
/// 1 -> Trusted
/// 2 -> Trusted + moderator
/// 3 -> Moderator
/// 4 -> Admin
/// 999 -> Owner
/// Trusted means someone has access to 'red button', basically to power off everything
pub fn role_permissions(role_id: &i32) -> Permission {
    match role_id {
        0 => Permission::NONE,
        1 => Permission::RED_BUTTON,
        2 => {
            Permission::RED_BUTTON
                | Permission::MODERATE_POSTS
                | Permission::MODERATE_COMMENTS
                | Permission::MODERATE_PROFILES
        }
        3 => {
            Permission::MODERATE_POSTS
                | Permission::MODERATE_COMMENTS
                | Permission::MODERATE_PROFILES
        }
        4 => {
            Permission::RED_BUTTON
                | Permission::MODERATE_POSTS
                | Permission::MODERATE_COMMENTS
                | Permission::MODERATE_PROFILES
                | Permission::BAN_USERS
                | Permission::ADMIN_PANEL
        }
        999 => Permission::all(),
        _ => Permission::NONE,
    }
}

pub fn permissions_to_list(p: Permission) -> Vec<&'static str> {
    let mut out = Vec::new();

    if p.contains(Permission::MODERATE_POSTS) {
        out.push("MODERATE_POSTS");
    }
    if p.contains(Permission::MODERATE_COMMENTS) {
        out.push("MODERATE_COMMENTS");
    }
    if p.contains(Permission::MODERATE_PROFILES) {
        out.push("MODERATE_PROFILES");
    }
    if p.contains(Permission::BAN_USERS) {
        out.push("BAN_USERS");
    }
    if p.contains(Permission::RED_BUTTON) {
        out.push("RED_BUTTON");
    }
    if p.contains(Permission::REVIEW_APPELLATIONS) {
        out.push("REVIEW_APPELLATIONS");
    }
    if p.contains(Permission::ADMIN_PANEL) {
        out.push("ADMIN_PANEL");
    }

    out
}
