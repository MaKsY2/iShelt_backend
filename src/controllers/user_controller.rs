use crate::models::auth_models::{LoginRequest, LoginResponse};
use crate::models::post_model::Post;
use crate::models::user_model::{User, UserCreateModel};

use crate::schema::posts;
use crate::schema::users;
use diesel::prelude::*;
use diesel::result::Error;
pub struct UserController();

pub trait UserControllerImpl {
    fn get_user(&self, conn: &mut PgConnection, name: String) -> Result<User, Error>;

    fn update_user(&self, conn: &mut PgConnection, uid: i32, req: User) -> Result<User, Error>;

    fn delete_user(&self, conn: &mut PgConnection, uid: i32);

    fn get_users(&self, conn: &mut PgConnection) -> Result<Vec<User>, Error>;

    fn create_user(
        &self,
        conn: &mut PgConnection,
        new_user: UserCreateModel,
    ) -> Result<bool, Error>;

    fn get_posts_by_user(&self, _conn: &mut PgConnection, _uid: i32) -> Result<Vec<Post>, Error>;
}

impl UserControllerImpl for UserController {
    fn get_user(&self, conn: &mut PgConnection, name: String) -> Result<User, Error> {
        return users::table
            .filter(users::name.eq(name))
            .first(conn)
            .map_err(|_| Error::NotFound);
    }

    fn get_users(&self, conn: &mut PgConnection) -> Result<Vec<User>, Error> {
        return users::table.load::<User>(conn).map_err(|_| Error::NotFound);
    }

    fn update_user(&self, _conn: &mut PgConnection, _uid: i32, _req: User) -> Result<User, Error> {
        return Err(Error::NotFound);
    }

    fn delete_user(&self, _conn: &mut PgConnection, _uid: i32) {}

    fn get_posts_by_user(&self, _conn: &mut PgConnection, _uid: i32) -> Result<Vec<Post>, Error> {
        let posts = posts::table
            .filter(posts::user_id.eq(_uid))
            .load(_conn)
            .map_err(|_| Error::NotFound);

        match posts {
            Ok(posts) => return Ok(posts),
            Err(_) => return Err(Error::NotFound),
        }
    }

    fn create_user(
        &self,
        conn: &mut PgConnection,
        new_user: UserCreateModel,
    ) -> Result<bool, Error> {
        match diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
        {
            Ok(_) => return Ok(true),
            Err(_) => return Err(Error::NotFound),
        }
    }
}
