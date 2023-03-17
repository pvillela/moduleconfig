pub use foo_a_data::*;
pub use foo_an_data::*;
pub use foo_aw_data::*;
pub use foo_data::*;

mod foo_data {
    #[derive(Debug, Clone)]
    pub struct FooSflCfgInfo {
        pub a: String,
        pub b: i32,
    }
}

mod foo_a_data {
    use super::FooSflCfgInfo;
    use crate::web::common_respond_to;
    use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};

    pub type FooASflCfgInfo = FooSflCfgInfo;

    #[derive(Clone, Deserialize)]
    pub struct FooAIn {
        pub sleep_millis: u64,
    }

    #[allow(unused)]
    #[derive(Serialize, Debug)]
    pub struct FooAOut {
        pub res: String,
    }

    impl Responder for FooAOut {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            common_respond_to(self)
        }
    }
}

mod foo_an_data {
    use super::{FooAIn, FooAOut, FooASflCfgInfo};

    pub type FooAnSflCfgInfo = FooASflCfgInfo;

    pub type FooAnIn = FooAIn;

    pub type FooAnOut = FooAOut;
}

mod foo_aw_data {
    use super::{FooAIn, FooAOut, FooASflCfgInfo};

    pub type FooAwSflCfgInfo = FooASflCfgInfo;

    pub type FooAwIn = FooAIn;

    pub type FooAwOut = FooAOut;
}
