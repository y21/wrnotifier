use std::fmt::Debug;
use warp::reject::Reject;

pub struct ValidationFail;

impl Debug for ValidationFail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Webhook validation failed")
    }
}

impl Reject for ValidationFail {}
