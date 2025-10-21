use anathema::{
    component::{Children, Component, Context, KeyCode, KeyEvent, UserEvent},
    prelude::{Backend, Document, TuiBackend},
    runtime::Runtime,
    state::{List, State, Value},
};
use ctor::ctor;

use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

// === Registry ===
static REGISTRY: OnceLock<Mutex<HashMap<&'static str, fn() -> String>>> = OnceLock::new();

fn get_registry() -> MutexGuard<'static, HashMap<&'static str, fn() -> String>> {
    REGISTRY
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
}
// === End Registry ===

fn main() {
    show_menu();
}

#[ctor]
fn register_test_function_1() {
    get_registry().insert("Test Function 1", test_function_1);
}
pub fn test_function_1() -> String {
    String::from("Hello folks!")
}

#[ctor]
fn register_test_function_2() {
    get_registry().insert("Test Function 2", test_function_2);
}
pub fn test_function_2() -> String {
    String::from("Hello folks from test 2!")
}

fn show_menu() {
    let doc = Document::new("@index");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);
    builder
        .default::<Application>("index", "templates/index.aml")
        .unwrap();
    builder
        .default::<ListBox>("list", "templates/listbox.aml")
        .unwrap();
    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();
}

#[derive(Default)]
struct Application;

impl Component for Application {
    type State = ();
    type Message = ();

    #[allow(unused_variables, unused_mut)]
    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        context.components.nth(1).focus();
    }

    fn on_event(
        &mut self,
        event: &mut UserEvent<'_>,
        _state: &mut Self::State,
        _children: Children<'_, '_>,
        _context: Context<'_, '_, Self::State>,
    ) {
        if event.name() == "function_select" {
            let key = event.data::<String>();
            eprintln!(
                "Function '{key}' gave '{}'",
                get_registry().get(key.as_str()).unwrap()()
            );
        }
    }
}

#[derive(State, Debug)]
struct ListBoxState {
    items: Value<List<ListItem>>,
    selected: Value<usize>,
}

impl Default for ListBoxState {
    fn default() -> Self {
        ListBoxState::from(get_registry().keys())
    }
}

impl ListBoxState {
    fn from<'a, T>(values: T) -> Self
    where
        T: IntoIterator<Item = &'a &'a str>,
        T: std::fmt::Debug,
    {
        let mut items: Value<List<ListItem>> = List::empty().into();
        let mut count: usize = 0;

        for value in values {
            items.push(ListItem {
                id: count.into(),
                text: String::from(*value).into(),
            });
            count += 1;
        }
        Self {
            items,
            selected: 0.into(),
        }
    }
}

#[derive(State, Debug)]
struct ListItem {
    id: Value<usize>,
    text: Value<String>,
}

#[derive(Default)]
struct ListBox;

impl Component for ListBox {
    type State = ListBoxState;

    type Message = ();

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        _children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        let mut selected = state.selected.to_mut();
        match key.code {
            KeyCode::Char('j') => {
                if *selected < state.items.len() - 1 {
                    *selected += 1
                }
            }
            KeyCode::Char('k') => {
                if *selected > 0 {
                    *selected -= 1
                }
            }
            KeyCode::Enter => {
                context.publish(
                    "select",
                    state
                        .items
                        .get(*selected)
                        .expect("the id and the index have gone out of sync.")
                        .text
                        .to_ref()
                        .clone(),
                );
            }
            _ => (),
        }
    }
}
