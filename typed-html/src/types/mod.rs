mod class;
pub use self::class::Class;

mod id;
pub use self::id::Id;

mod classlist;
pub use self::classlist::ClassList;

pub use http::Uri;
pub use language_tags::LanguageTag;
pub use mime::Mime;

#[derive(EnumString, Display)]
pub enum CrossOrigin {
    #[strum(to_string = "anonymous")]
    Anonymous,
    #[strum(to_string = "use-credentials")]
    UseCredentials,
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum LinkType {
        #[strum(to_string = "alternate")]
        Alternate,
        #[strum(to_string = "author")]
        Author,
        #[strum(to_string = "bookmark")]
        Bookmark,
        #[strum(to_string = "canonical")]
        Canonical,
        #[strum(to_string = "external")]
        External,
        #[strum(to_string = "help")]
        Help,
        #[strum(to_string = "icon")]
        Icon,
        #[strum(to_string = "license")]
        License,
        #[strum(to_string = "manifest")]
        Manifest,
        #[strum(to_string = "modulepreload")]
        ModulePreload,
        #[strum(to_string = "next")]
        Next,
        #[strum(to_string = "nofollow")]
        NoFollow,
        #[strum(to_string = "noopener")]
        NoOpener,
        #[strum(to_string = "noreferrer")]
        NoReferrer,
        #[strum(to_string = "pingback")]
        PingBack,
        #[strum(to_string = "prefetch")]
        Prefetch,
        #[strum(to_string = "preload")]
        Preload,
        #[strum(to_string = "prev")]
        Prev,
        #[strum(to_string = "search")]
        Search,
        #[strum(to_string = "shortlink")]
        ShortLink,
        #[strum(to_string = "stylesheet")]
        StyleSheet,
        #[strum(to_string = "tag")]
        Tag,
    }
}
