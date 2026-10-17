//
// TurtKey - Plugin Architecture Root
// Extensible plugin systems for UI Internationalization (i18n) and Input Method Engines
// Clean, modular, zero emojis
//

pub mod i18n;
pub mod input_engine;

pub use i18n::{I18nManager, LanguagePack, LanguagePluginMeta};
pub use input_engine::{
    EnginePluginManager, FrenchEnginePlugin, GermanEnginePlugin, InputEnginePlugin,
    PassthroughEnginePlugin, SpanishEnginePlugin, VietnameseEnginePlugin,
};
