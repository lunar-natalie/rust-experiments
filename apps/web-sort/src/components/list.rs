use leptos::*;
use leptos::ev::Event;

#[component]
pub fn List(initial_length: usize) -> impl IntoView {
    type DataType = i32;

    let mut next_id = initial_length;

    let initial_elements = (0..initial_length)
        .map(|id| (id, create_signal(0 as DataType)))
        .collect::<Vec<_>>();

    let (elements, set_elements) = create_signal(initial_elements);

    let add_element = move |_| {
        set_elements.update(move |elements| {
            elements.push((next_id, create_signal(0 as DataType)));
        });
        next_id += 1;
    };

    let remove_element = move |id| {
        set_elements.update(|elements| {
            elements.retain(|(this_id, _)| &id != this_id);
        });
    };

    let update_element = move |ev: Event, set_element: WriteSignal<DataType>| {
        set_element(
            event_target_value(&ev)
                .parse::<DataType>()
                .unwrap_or_default(),
        );
    };

    let element_view = move |(id, (element, set_element)): (
        usize,
        (ReadSignal<DataType>, WriteSignal<DataType>),
    )| {
        view! {
            <li>
                <input
                    type="number"
                    name="Element ".to_owned() + &id.to_string()
                    on:input=move |ev| { update_element(ev, set_element) }
                    prop:value=element
                />
                <button on:click=move |_| { remove_element(id) }>Remove</button>
            </li>
        }
    };

    view! {
        <div>
            <button on:click=add_element>"Add"</button>
            <ul>
                <For each=elements key=|element| element.0 children=element_view/>
            </ul>
        </div>
    }
}
