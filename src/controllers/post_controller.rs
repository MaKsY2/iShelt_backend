use crate::{models::post_model::Post, schema::posts};

use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

pub struct PostController {}

pub trait PostControllerImpl {
    fn get_posts_whithout_my(&self, conn: &mut PgConnection, uid: i32) -> Result<Vec<Post>, Error>;

    fn get_posts_by_user_id(&self, conn: &mut PgConnection, uid: i32) -> Result<Vec<Post>, Error>;

    fn create_post(&self, conn: &mut PgConnection, new_post: Post) -> Result<Post, Error>;

    fn delete_post(&self, conn: &mut PgConnection, post_id: i32) -> Result<bool, Error>;
}

impl PostControllerImpl for PostController {
    fn get_posts_whithout_my(
        &self,
        _conn: &mut PgConnection,
        _uid: i32,
    ) -> Result<Vec<Post>, Error> {
        let posts = posts::table
            .filter(posts::user_id.ne(_uid))
            .load(_conn)
            .map_err(|_| Error::NotFound);

        match posts {
            Ok(posts) => return Ok(posts),
            Err(_) => return Err(Error::NotFound),
        }
    }

    fn get_posts_by_user_id(&self, conn: &mut PgConnection, uid: i32) -> Result<Vec<Post>, Error> {
        let posts = posts::table
            .filter(posts::user_id.eq(uid))
            .load(conn)
            .map_err(|_| Error::NotFound);

        match posts {
            Ok(posts) => return Ok(posts),
            Err(_) => return Err(Error::NotFound),
        }
    }

    fn create_post(&self, conn: &mut PgConnection, new_post: Post) -> Result<Post, Error> {
        let post = diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)
            .map_err(|_| Error::NotFound);

        match post {
            Ok(post) => return Ok(post),
            Err(_) => return Err(Error::NotFound),
        }
    }

    fn delete_post(&self, conn: &mut PgConnection, post_id: i32) -> Result<bool, Error> {
        let post = diesel::delete(posts::table.filter(posts::id.eq(post_id)))
            .execute(conn)
            .map_err(|_| Error::NotFound);

        match post {
            Ok(_) => return Ok(true),
            Err(_) => return Err(Error::NotFound),
        }
    }
}
