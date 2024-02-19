pub struct UserController();

pub trait UserControllerImpl {
    fn login(&self, req: LoginRequest) -> LoginResponse;

    fn register(&self, req: User) -> User;

    fn get_user(&self, uid: i32) -> User;

    fn update_user(&self, uid: i32, req: User) -> User;

    fn delete_user(&self, uid: i32);

    fn get_posts_by_user(&self, uid: i32) -> Vec<Post>;
}

impl UserControllerImpl for UserController {
    fn login(&self, req: LoginRequest) -> LoginResponse {
        let token = auth::create_jwt(req.uid);
        LoginResponse { token, user: req.into() }
    }

    
}