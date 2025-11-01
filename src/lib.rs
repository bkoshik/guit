mod project {
    pub(crate) mod bin {}
    mod lib {
        mod enums {
            mod file_status_kind;

            pub use file_status_kind::*;
        }
        mod structs {
            mod author_info;
            mod branches;
            mod commit_info;
            mod file_status_entry;
            mod message;
            mod repository_info;

            pub use author_info::*;
            pub use branches::*;
            pub use commit_info::*;
            pub use file_status_entry::*;
            pub use message::*;
            pub use repository_info::*;
        }
        pub mod utils {
            pub(crate) mod helpers {
                mod find_branches;
                mod result;

                pub use find_branches::*;
                pub use result::*;
            }
            pub mod commands {
                mod branches;
                mod commit;
                mod head;
                mod tags;

                pub use branches::*;
                pub use commit::*;
                pub use head::*;
                pub use tags::*;
            }
        }

        pub use enums::*;
        pub use structs::*;
    }

    pub use lib::*;
}

pub use project::*;
