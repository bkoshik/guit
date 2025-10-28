mod project {
    pub(crate) mod bin {}
    mod lib {
        pub mod commands {
            pub mod commit {}
            pub mod log {}
            pub mod status {}
        }

        mod structs {
            mod author_info;
            mod branches;
            mod message;
            mod repository_info;

            pub use author_info::*;
            pub use branches::*;
            pub use message::*;
            pub use repository_info::*;
        }

        pub use structs::*;
    }

    pub use lib::*;
}

pub use project::*;
