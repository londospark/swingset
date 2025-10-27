use std::cell::RefCell;
use std::collections::HashMap;

use anathema::component::{Children, Component, Context, KeyCode, KeyEvent, UserEvent};
use anathema::prelude::{Backend, Document, TuiBackend};
use anathema::runtime::Runtime;
use anathema::state::{List, State, Value};

type Registry = HashMap<&'static str, fn() -> String>;

// === Registry ===

// NOTE: use thread local storage instead of a `OnceCell` as we can use the heap and 
// this is single threaded anyway
thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(HashMap::new());
}

// NOTE: use separate functions so as not to hold the `Ref` (foremerly MutexGuard) for
// longer than necessary (no deadlocks with the guard, and no panics with the Ref)
pub fn insert_into_registry(name: &'static str, f: fn() -> String) {
    REGISTRY.with_borrow_mut(|reg| reg.insert(name, f));
}

pub fn get_fun(name: &str) -> fn() -> String {
    REGISTRY.with_borrow(|reg| *reg.get(name).unwrap())
}

// NOTE: since the final values will end up being strings it's 
// simpler to convert the strings inside this function, meaning we can
// drop the `Ref` (and make it possible to borrow again) sooner.
pub fn names() -> Vec<String> {
    REGISTRY.with_borrow(|reg| reg.keys().map(ToString::to_string).collect())
}

// === End Registry ===

pub fn show_menu() {
    let doc = Document::new("@index");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);
    builder.default::<Application>("index", "templates/index.aml").unwrap();
    builder.default::<ListBox>("list", "templates/listbox.aml").unwrap();
    builder.finish(&mut backend, anathema::prelude::Runtime::run).unwrap();
}

#[derive(Default)]
struct Application;

impl Component for Application {
    type Message = ();
    type State = ();

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
            eprintln!("Function '{key}' gave '{}'", get_fun(key.as_str())());
        }
    }
}

#[derive(State, Debug)]
struct ListBoxState {
    items: Value<List<ListItem>>,
    selected: Value<usize>,
}

impl ListBoxState {
    pub fn new() -> Self {
        // NOTE: since `List::from_iter` is a thing we can just make an iterator instead
        // of using a classic for-loop.
        //
        // This removes the need for the `From` impl for the `ListBoxState`.
        // We also don't need `values` to be mutable.
        let values = names().into_iter().enumerate().map(|(id, val)| ListItem::new(id, val));

        Self {
            items: List::from_iter(values).into(),
            selected: 0.into(),
        }
    }
}

impl Default for ListBoxState {
    fn default() -> Self {
        ListBoxState::new()
    }
}

#[derive(State, Debug)]
struct ListItem {
    id: Value<usize>,
    text: Value<String>,
}

impl ListItem {
    fn new(id: usize, text: String) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

#[derive(Default)]
struct ListBox;

impl Component for ListBox {
    type Message = ();
    type State = ListBoxState;

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        _children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        // NOTE: 
        // * shorten the body of each match arm
        // * j / k will now wrap around (to stop wrapping remove the additional match arms)
        let mut selected = state.selected.to_mut();
        match key.code {
            KeyCode::Char('j') if *selected < state.items.len() - 1 => *selected += 1,
            KeyCode::Char('j') => *selected = 0,
            KeyCode::Char('k') if *selected > 0 => *selected -= 1,
            KeyCode::Char('k') => *selected = state.items.len() - 1,
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
