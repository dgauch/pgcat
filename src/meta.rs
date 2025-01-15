use crate::errors::Error;

pub struct MetaConnectionPoolResolver {}

impl MetaConnectionPoolResolver {
    pub fn resolve(username: &String) -> Result<String, Error> {
        // when the username matches the regular expression "c0440559-114c-4bea-b7fd-0..........." we return the cluster01
        return if (username.len() == 38 && username.starts_with("s_c0440559_114c_4bea_b7fd_0")) || username.eq("dbadmin_cluster00") {
            Ok("cluster00".to_string())
        } else if (username.len() == 38 && username.starts_with("s_c0440559_114c_4bea_b7fd_1")) || username.eq("dbadmin_cluster01") {
            Ok("cluster01".to_string())
        } else if (username.len() == 38 && username.starts_with("s_c0440559_114c_4bea_b7fd_2")) || username.eq("dbadmin_cluster02") {
            Ok("cluster02".to_string())
        } else if username.len() == 38 && username.starts_with("s_") {
            Err(Error::CannotResolvePool(format!(
                "Cannot resolve pool for username {}",
                username
            )))
        } else {
            // This is the default cluster to be used
            Ok("cluster00".to_string())
        };

        // Err(Error::CannotResolvePool(format!("Cannot resolve pool for username {}", username)))
    }
}
