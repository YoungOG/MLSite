use std::sync::{Arc, Mutex};
use iron::{status, AfterMiddleware, IronResult, Request, Response, Handler};
use iron::headers::ContentType;
use database::Database;
use iron::headers::{AccessControlAllowOrigin, AccessControlAllowCredentials, AccessControlAllowHeaders, AccessControlAllowMethods};
use unicase::UniCase;
use iron::method::Method;
use userdata::extract_token_data_from_header;
use user_handlers::*;
use news_post_handlers::*;
use forum_handlers::*;

macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.description())))
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    }
}

macro_rules! lock {
    ($e:expr) => {$e.lock().unwrap()}
}

macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest)),
                }
            },
            None => return Ok(Response::with(status::InternalServerError))
        }
    }
}

pub struct Handlers {
    pub admin_handler: AdminHandler,
    pub news_post_handler: NewsPostHandler,
    pub news_post_feed_handler: NewsPostFeedHandler,
    pub news_post_post_handler: NewsPostPostHandler,
    pub user_created_handler: UserCreateHandler,
    pub login_request_handler: LoginRequestHandler,
    pub user_get_single_handler: GetSingleUserHandler,
    pub user_register_handler: UserRegisterHandler,
    pub user_get_staff_handler: GetStaffUsersHandler,
    pub get_all_posts_handler: GetAllPostsHandler,
    pub get_category_stats_and_last_post: GetCategoryStatsAndLastPost,
    pub get_forum_listing_data: GetForumListingData,
    pub post_post_to_thread: PostPostToThread,
    pub post_thread_to_forum: PostThreadToForum,
    pub get_forum_thread: GetForumThread,
    pub get_user_register_handler: GetUserRegisterFormHandler,
    pub search_users_handler: SearchUsersHandler,
    pub server_login: ServerLogin,
}

impl Handlers {
    pub fn new(database: Database) -> Handlers {
        let db = Arc::new(Mutex::new(database));
        Handlers {
            admin_handler: AdminHandler::new(),
            news_post_handler: NewsPostHandler::new(db.clone()),
            news_post_post_handler: NewsPostPostHandler::new(db.clone()),
            news_post_feed_handler: NewsPostFeedHandler::new(db.clone()),
            user_created_handler: UserCreateHandler::new(db.clone()),
            login_request_handler: LoginRequestHandler::new(db.clone()),
            user_get_single_handler: GetSingleUserHandler::new(db.clone()),
            user_register_handler: UserRegisterHandler::new(db.clone()),
            user_get_staff_handler: GetStaffUsersHandler::new(db.clone()),
            get_all_posts_handler: GetAllPostsHandler::new(db.clone()),
            get_category_stats_and_last_post: GetCategoryStatsAndLastPost::new(db.clone()),
            get_forum_listing_data: GetForumListingData::new(db.clone()),
            post_post_to_thread: PostPostToThread::new(db.clone()),
            post_thread_to_forum: PostThreadToForum::new(db.clone()),
            get_forum_thread: GetForumThread::new(db.clone()),
            get_user_register_handler: GetUserRegisterFormHandler::new(db.clone()),
            search_users_handler: SearchUsersHandler::new(db.clone()),
            server_login: ServerLogin::new(db.clone()),
        }
    }
}

pub struct AdminHandler;

impl AdminHandler {
    fn new() -> AdminHandler {
        AdminHandler { }
    }
}

impl Handler for AdminHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        if let Some(data) = extract_token_data_from_header(req) {
            if data.admin == true {
                Ok(Response::with((status::Ok, "{}")))
            } else {
                Ok(Response::with(status::Forbidden))
            }
        } else {
            Ok(Response::with(status::Forbidden))
        }
    }
}

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}

pub struct CorsAfterMiddleWare;

impl AfterMiddleware for CorsAfterMiddleWare {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(AccessControlAllowOrigin::Any);
        res.headers.set(AccessControlAllowCredentials);
        res.headers.set(AccessControlAllowHeaders(vec![
            UniCase("Content-Type".to_owned()),
            UniCase("Authorization".to_owned()),
            UniCase("X-Requested-With".to_owned()),
        ]));
        res.headers.set(AccessControlAllowMethods(vec![
            Method::Get,
            Method::Post,
        ]));
        Ok(res)
    }
}