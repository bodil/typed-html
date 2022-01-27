//! Types for attribute values.

use strum_macros::*;

mod class;
pub use self::class::Class;

mod id;
pub use self::id::Id;

mod spacedlist;
pub use self::spacedlist::SpacedList;

mod spacedset;
pub use self::spacedset::SpacedSet;

pub type ClassList = SpacedSet<Class>;

pub use language_tags::LanguageTag;
pub use mime::Mime;

// FIXME these all need validating types
pub type Uri = String;
pub type CharacterEncoding = String;
pub type Datetime = String;
pub type FeaturePolicy = String;
pub type Integrity = String;
pub type Nonce = String;
pub type Target = String;

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum AreaShape {
    #[strum(to_string = "rect")]
    Rectangle,
    #[strum(to_string = "circle")]
    Circle,
    #[strum(to_string = "poly")]
    Polygon,
    #[strum(to_string = "default")]
    Default,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum BoolOrDefault {
    #[strum(to_string = "true")]
    True,
    #[strum(to_string = "default")]
    Default,
    #[strum(to_string = "false")]
    False,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum ButtonType {
    #[strum(to_string = "submit")]
    Submit,
    #[strum(to_string = "reset")]
    Reset,
    #[strum(to_string = "button")]
    Button,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum Bool {
    #[strum(to_string = "true")]
    True,
    #[strum(to_string = "")]
    False,
}

impl From<bool> for Bool {
    fn from(v: bool) -> Self {
        if v {
            Bool::True
        } else {
            Bool::False
        }
    }
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum CrossOrigin {
    #[strum(to_string = "anonymous")]
    Anonymous,
    #[strum(to_string = "use-credentials")]
    UseCredentials,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum FormEncodingType {
    #[strum(to_string = "application/x-www-form-urlencoded")]
    UrlEncoded,
    #[strum(to_string = "multipart/form-data")]
    FormData,
    #[strum(to_string = "text/plain")]
    Text,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum FormMethod {
    #[strum(to_string = "post")]
    Post,
    #[strum(to_string = "get")]
    Get,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum FormDialogMethod {
    #[strum(to_string = "post")]
    Post,
    #[strum(to_string = "get")]
    Get,
    #[strum(to_string = "dialog")]
    Dialog,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum HTTPEquiv {
    #[strum(to_string = "content-security-policy")]
    ContentSecurityPolicy,
    #[strum(to_string = "refresh")]
    Refresh,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum ImageDecoding {
    #[strum(to_string = "sync")]
    Sync,
    #[strum(to_string = "async")]
    Async,
    #[strum(to_string = "auto")]
    Auto,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum InputType {
    #[strum(to_string = "button")]
    Button,
    #[strum(to_string = "checkbox")]
    Checkbox,
    #[strum(to_string = "color")]
    Color,
    #[strum(to_string = "date")]
    Date,
    #[strum(to_string = "datetime-local")]
    DatetimeLocal,
    #[strum(to_string = "email")]
    Email,
    #[strum(to_string = "file")]
    File,
    #[strum(to_string = "hidden")]
    Hidden,
    #[strum(to_string = "image")]
    Image,
    #[strum(to_string = "month")]
    Month,
    #[strum(to_string = "number")]
    Number,
    #[strum(to_string = "password")]
    Password,
    #[strum(to_string = "radio")]
    Radio,
    #[strum(to_string = "range")]
    Range,
    #[strum(to_string = "reset")]
    Reset,
    #[strum(to_string = "search")]
    Search,
    #[strum(to_string = "submit")]
    Submit,
    #[strum(to_string = "tel")]
    Tel,
    #[strum(to_string = "text")]
    Text,
    #[strum(to_string = "time")]
    Time,
    #[strum(to_string = "url")]
    Url,
    #[strum(to_string = "week")]
    Week,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
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

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum Metadata {
    #[strum(to_string = "application-name")]
    ApplicationName,
    #[strum(to_string = "author")]
    Author,
    #[strum(to_string = "description")]
    Description,
    #[strum(to_string = "generator")]
    Generator,
    #[strum(to_string = "keywords")]
    Keywords,
    #[strum(to_string = "referrer")]
    Referrer,
    #[strum(to_string = "creator")]
    Creator,
    #[strum(to_string = "googlebot")]
    Googlebot,
    #[strum(to_string = "publisher")]
    Publisher,
    #[strum(to_string = "robots")]
    Robots,
    #[strum(to_string = "viewport")]
    Viewport,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum OnOff {
    #[strum(to_string = "on")]
    On,
    #[strum(to_string = "off")]
    Off,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum OrderedListType {
    #[strum(to_string = "a")]
    LowerCaseLetters,
    #[strum(to_string = "A")]
    UpperCaseLetters,
    #[strum(to_string = "i")]
    LowerCaseRomanNumerals,
    #[strum(to_string = "I")]
    UpperCaseRomanNumerals,
    #[strum(to_string = "1")]
    Numbers,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum Preload {
    #[strum(to_string = "none")]
    None,
    #[strum(to_string = "metadata")]
    Metadata,
    #[strum(to_string = "auto")]
    Auto,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum ReferrerPolicy {
    #[strum(to_string = "no-referrer")]
    NoReferrer,
    #[strum(to_string = "no-referrer-when-downgrade")]
    NoReferrerWhenDowngrade,
    #[strum(to_string = "origin")]
    Origin,
    #[strum(to_string = "origin-when-cross-origin")]
    OriginWhenCrossOrigin,
    #[strum(to_string = "unsafe-url")]
    UnsafeUrl,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum Role {
    #[strum(to_string = "any")]
    Any,
    #[strum(to_string = "alert")]
    Alert,
    #[strum(to_string = "alertdialog")]
    AlertDialog,
    #[strum(to_string = "application")]
    Application,
    #[strum(to_string = "article")]
    Article,
    #[strum(to_string = "banner")]
    Banner,
    #[strum(to_string = "checkbox")]
    Checkbox,
    #[strum(to_string = "cell")]
    Cell,
    #[strum(to_string = "columnheader")]
    ColumnHeader,
    #[strum(to_string = "combobox")]
    ComboBox,
    #[strum(to_string = "complementary")]
    Complementary,
    #[strum(to_string = "contentinfo")]
    ContentInfo,
    #[strum(to_string = "definition")]
    Definition,
    #[strum(to_string = "dialog")]
    Dialog,
    #[strum(to_string = "directory")]
    Directory,
    #[strum(to_string = "document")]
    Document,
    #[strum(to_string = "feed")]
    Feed,
    #[strum(to_string = "figure")]
    Figure,
    #[strum(to_string = "form")]
    Form,
    #[strum(to_string = "grid")]
    Grid,
    #[strum(to_string = "gridcell")]
    GridCell,
    #[strum(to_string = "group")]
    Group,
    #[strum(to_string = "heading")]
    Heading,
    #[strum(to_string = "img")]
    Image,
    #[strum(to_string = "link")]
    Link,
    #[strum(to_string = "list")]
    List,
    #[strum(to_string = "listbox")]
    ListBox,
    #[strum(to_string = "listitem")]
    ListItem,
    #[strum(to_string = "log")]
    Log,
    #[strum(to_string = "main")]
    Main,
    #[strum(to_string = "marquee")]
    Marquee,
    #[strum(to_string = "math")]
    Math,
    #[strum(to_string = "menu")]
    Menu,
    #[strum(to_string = "menubar")]
    MenuBar,
    #[strum(to_string = "menuitem")]
    MenuItem,
    #[strum(to_string = "menuitemcheckbox")]
    MenuItemCheckbox,
    #[strum(to_string = "menuitemradio")]
    MenuItemRadio,
    #[strum(to_string = "navigation")]
    Navigation,
    #[strum(to_string = "none")]
    None,
    #[strum(to_string = "note")]
    Note,
    #[strum(to_string = "option")]
    Option,
    #[strum(to_string = "presentation")]
    Presentation,
    #[strum(to_string = "progressbar")]
    ProgressBar,
    #[strum(to_string = "radio")]
    Radio,
    #[strum(to_string = "radiogroup")]
    RadioGroup,
    #[strum(to_string = "region")]
    Region,
    #[strum(to_string = "row")]
    Row,
    #[strum(to_string = "rowgroup")]
    RowGroup,
    #[strum(to_string = "rowheader")]
    RowHeader,
    #[strum(to_string = "scrollbar")]
    ScrollBar,
    #[strum(to_string = "search")]
    Search,
    #[strum(to_string = "searchbox")]
    SearchBox,
    #[strum(to_string = "separator")]
    Separator,
    #[strum(to_string = "slider")]
    Slider,
    #[strum(to_string = "spinbutton")]
    SpinButton,
    #[strum(to_string = "status")]
    Status,
    #[strum(to_string = "switch")]
    Switch,
    #[strum(to_string = "tab")]
    Tab,
    #[strum(to_string = "table")]
    Table,
    #[strum(to_string = "tablist")]
    TabList,
    #[strum(to_string = "tabpanel")]
    TabPanel,
    #[strum(to_string = "term")]
    Term,
    #[strum(to_string = "textbox")]
    TextBox,
    #[strum(to_string = "timer")]
    Timer,
    #[strum(to_string = "toolbar")]
    ToolBar,
    #[strum(to_string = "tooltip")]
    ToolTip,
    #[strum(to_string = "tree")]
    Tree,
    #[strum(to_string = "treegrid")]
    TreeGrid,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum Sandbox {
    #[strum(to_string = "allow-forms")]
    AllowForms,
    #[strum(to_string = "allow-modals")]
    AllowModals,
    #[strum(to_string = "allow-orientation-lock")]
    AllowOrientationLock,
    #[strum(to_string = "allow-pointer-lock")]
    AllowPointerLock,
    #[strum(to_string = "allow-popups")]
    AllowPopups,
    #[strum(to_string = "allow-popups-to-escape-sandbox")]
    AllowPopupsToEscapeSandbox,
    #[strum(to_string = "allow-presentation")]
    AllowPresentation,
    #[strum(to_string = "allow-same-origin")]
    AllowSameOrigin,
    #[strum(to_string = "allow-scripts")]
    AllowScripts,
    #[strum(to_string = "allow-top-navigation")]
    AllowTopNavigation,
    #[strum(to_string = "allow-top-navigation-by-user-navigation")]
    AllowTopNavigationByUserNavigation,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum TableHeaderScope {
    #[strum(to_string = "row")]
    Row,
    #[strum(to_string = "col")]
    Column,
    #[strum(to_string = "rowgroup")]
    RowGroup,
    #[strum(to_string = "colgroup")]
    ColGroup,
    #[strum(to_string = "auto")]
    Auto,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum TextDirection {
    #[strum(to_string = "ltr")]
    LeftToRight,
    #[strum(to_string = "rtl")]
    RightToLeft,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum VideoKind {
    #[strum(to_string = "subtitles")]
    Subtitles,
    #[strum(to_string = "captions")]
    Captions,
    #[strum(to_string = "descriptions")]
    Descriptions,
    #[strum(to_string = "chapters")]
    Chapters,
    #[strum(to_string = "metadata")]
    Metadata,
}

#[derive(EnumString, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr, IntoStaticStr)]
pub enum Wrap {
    #[strum(to_string = "hard")]
    Hard,
    #[strum(to_string = "soft")]
    Soft,
    #[strum(to_string = "off")]
    Off,
}
