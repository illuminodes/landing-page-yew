use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppLocale {
    English,
    Spanish,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Translations {
    locale: AppLocale,
    translations: TranslationData,
}
impl Translations {
    pub fn translations(&self) -> &std::collections::HashMap<String, String> {
        &self.translations.translations
    }
    pub fn current_locale(&self) -> AppLocale {
        self.locale
    }
    pub fn find_translation(&self, key: &str) -> String {
        self.translations.translations.get(key).cloned().unwrap_or(format!("NOT FOUND {}", key))
    }
}

pub enum TranslationAction {
    ChangeLocale(AppLocale),
}
impl Reducible for Translations {
    type Action = TranslationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TranslationAction::ChangeLocale(locale) => Rc::new(Translations {
                locale,
                translations: TranslationData::load_translation(locale),
            }),
        }
    }
}
pub type TranslationStore = UseReducerHandle<Translations>;

#[function_component(TranslationProvider)]
pub fn key_handler(props: &yew::html::ChildrenProps) -> Html {
    let ctx = use_reducer(|| Translations {
        locale: AppLocale::Spanish,
        translations: TranslationData::default(),
    });

    html! {
        <ContextProvider<TranslationStore> context={ctx}>
            {props.children.clone()}
        </ContextProvider<TranslationStore>>
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static ENGLISH_TRANSLATIONS: &str = include_str!("./lang/en.json");
static SPANISH_TRANSLATIONS: &str = include_str!("./lang/es.json");

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TranslationData {
    #[serde(flatten)]
    pub translations: HashMap<String, String>,
}
impl Default for TranslationData {
    fn default() -> Self {
        Self::load_translation(AppLocale::Spanish)
    }
}
impl TranslationData {
    pub fn load_translation(locale: AppLocale) -> Self {
        match locale {
            AppLocale::English => serde_json::from_str(ENGLISH_TRANSLATIONS).unwrap(),
            AppLocale::Spanish => serde_json::from_str(SPANISH_TRANSLATIONS).unwrap(),
        }
    }
}
