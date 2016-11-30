mod answer {
    #[cfg(feature = "serde_derive")]
    include!("answer.in.rs");

    #[cfg(feature = "serde_codegen")]
    include!(concat!(env!("OUT_DIR"), "/answer.rs"));
}

mod archive {
    #[cfg(feature = "serde_derive")]
    include!("archive.in.rs");

    #[cfg(feature = "serde_codegen")]
    include!(concat!(env!("OUT_DIR"), "/archive.rs"));
}

mod question {
    #[cfg(feature = "serde_derive")]
    include!("question.in.rs");

    #[cfg(feature = "serde_codegen")]
    include!(concat!(env!("OUT_DIR"), "/question.rs"));
}

mod team {
    #[cfg(feature = "serde_derive")]
    include!("team.in.rs");

    #[cfg(feature = "serde_codegen")]
    include!(concat!(env!("OUT_DIR"), "/team.rs"));
}

mod topic {
    #[cfg(feature = "serde_derive")]
    include!("topic.in.rs");

    #[cfg(feature = "serde_codegen")]
    include!(concat!(env!("OUT_DIR"), "/topic.rs"));
}

pub mod settings {
    #[cfg(feature = "serde_derive")]
    include!("settings.in.rs");

    #[cfg(feature = "serde_codegen")]
    include!(concat!(env!("OUT_DIR"), "/settings.rs"));
}

pub use self::answer::Answer;
pub use self::archive::{Archive, ArchiveInfo};
pub use self::question::Question;
pub use self::team::Team;
pub use self::topic::Topic;
