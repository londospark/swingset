use anathema::{
    component::Component,
    prelude::{Backend, Document, TuiBackend},
    runtime::Runtime,
    state::{List, State, Value},
};

fn main() {
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
        .default::<()>("index", "templates/index.aml")
        .unwrap();
    builder
        .default::<ListBox>("list", "templates/listbox.aml")
        .unwrap();
    builder
        .finish(&mut backend, |mut runtime, backend| runtime.run(backend))
        .unwrap();
}

#[derive(State)]
struct ListBoxState {
    items: Value<List<ListItem>>,
    selected: Value<usize>,
}

impl Default for ListBoxState {
    fn default() -> Self {
        ListBoxState::from(vec!["Florp".into(), "Blerp".into(), "Lark".into()])
    }
}

impl ListBoxState {
    fn from(values: Vec<String>) -> Self {
        let mut items: Value<List<ListItem>> = List::empty().into();
        let mut count: usize = 1;

        for value in values {
            items.push(ListItem {
                id: count.into(),
                text: value.into(),
            });
            count += 1;
        }
        Self {
            items,
            selected: 1.into(),
        }
    }
}

#[derive(State)]
struct ListItem {
    id: Value<usize>,
    text: Value<String>,
}

#[derive(Default)]
struct ListBox;

impl Component for ListBox {
    type State = ListBoxState;

    type Message = ();
}
