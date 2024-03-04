use crate::domain::user::User;

crud!(User{},"users");
impl_select!(User{select_by_id(id:u32) -> Option => "`where id = #{id} limit 1`"});
