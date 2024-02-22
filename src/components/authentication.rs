use hyper::HeaderMap;

struct Permissions {}
impl Permissions {
    pub fn check_anoynmous_permission(action: u16) -> bool {
        match action {
            //ACTION_STORE_MAIN
            0 => true,
            //UNKNOWN
            _ => false,
        }
    }
}

pub struct Authentication {
    username: String,
    token: String,
}
impl Authentication {
    pub const ACTION_STORE_MAIN: u16 = 0;

    pub fn new(header: HeaderMap) -> Self {
        Authentication {
            username: header
                .get("username")
                .and_then(|value| value.to_str().ok())
                .unwrap_or("anonymous")
                .to_string(),
            token: header
                .get("token")
                .and_then(|value| value.to_str().ok())
                .unwrap_or("")
                .to_string(),
        }
    }

    ///If the user is authenticated and have permission return true, if not return false
    pub fn authenticate(&self, action: u16) -> bool {
        if let "Anonymous" = self.username.as_str() {
            return Permissions::check_anoynmous_permission(action);
        }
        println!("User token: {:?}", self.token);
        false
    }
}
