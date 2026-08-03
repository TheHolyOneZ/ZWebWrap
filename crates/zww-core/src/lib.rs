pub mod autostart;
pub mod bookmarks;
pub mod credentials;
pub mod icon;
pub mod lang;
pub mod media;
pub mod model;
pub mod paths;
pub mod prefs;
pub mod registry;
pub mod scope;
pub mod site;

pub use media::MediaSupport;
pub use model::{
    BlockMode, FrameStyle, OffsitePolicy, Permissions, Rule, SessionMode, WindowState, WrappedApp,
};
pub use prefs::Preferences;
pub use registry::Registry;
