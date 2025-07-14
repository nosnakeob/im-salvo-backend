use crate::domain::db::User;

crud!(User{},"users");
impl_select!(User{select_by_id(id:u32) -> Option => "`where id = #{id} limit 1`"}, "users");
impl_select!(User{select_by_name(name:&str) -> Option => "`where username = #{name} limit 1`"}, "users");
impl_update!(User{update_by_id(id:u32) => "`where id = #{id}`"}, "users");
impl_delete!(User{delete_by_name(name:&str) => "`where username = #{name}`"}, "users");