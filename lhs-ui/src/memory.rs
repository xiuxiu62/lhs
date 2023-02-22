use crate::{cell::Cell, machine::MachineRef};
use lhs_core::runtime;
use yew::{function_component, html, use_context, Component, Html};

// pub struct Memory<const N: usize>(runtime::Memory<N>);

// impl<const N: usize> Memory<N> {
//     pub fn new() -> Self {
//         Self(runtime::Memory::default())
//     }
// }

// impl<const N: usize> Component for Memory<N> {
//     type Message = ();
//     type Properties = ();

//     fn create(_ctx: &yew::Context<Self>) -> Self {
//         Self::new()
//     }

//     fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
//         let rows = self.0.iter().enumerate().map(|(y, row)| {
//             let cells = row.iter().enumerate().map(|(x, cell)| {
//                 html! {
//                     <div class="memory-cell">
//                         <Cell ident={ x } value={ *cell } />
//                     </div>
//                 }
//             });

//             html! {
//                 <div key={y} class="memory-row">
//                     { for cells }
//                 </div>
//             }
//         });

//         html! {
//             <div class="machine-data-component">
//                 <div class="memory-container">
//                     <header class="machine-component-header">
//                         <h1 class="machine-component-title">{ "Memory" }</h1>
//                     </header>
//                     <section class="memory-area">
//                         <div class="memory">
//                             { for rows }
//                         </div>
//                     </section>
//                     <footer class="machine-data-footer">
//                         <strong class="machine-data-footer-text">
//                           { format!(" pointer: {}", self.0.pointer).as_str() }
//                         </strong>
//                     </footer>
//                 </div>
//             </div>
//         }
//     }
// }

#[function_component(Memory)]
pub fn memory() -> Html {
    // TODO: handle uninitialized machine
    // NOTE: machine may always be initialized, given it is done so in a parent node,
    //     but we need to ensure this is the case
    let machine = use_context::<MachineRef>().unwrap();
    let machine_ref = machine.borrow();
    let rows = machine_ref.memory.iter().enumerate().map(|(y, row)| {
        let cells = row.into_iter().enumerate().map(|(x, cell)| {
            html! {
                <div class="memory-cell">
                    <Cell ident={ x } value={ *cell } />
                </div>
            }
        });

        html! {
            <div key={y} class="memory-row">
                { for cells }
            </div>
        }
    });

    html! {
        <div class="machine-data-component">
            <div class="memory-container">
                <header class="machine-component-header">
                    <h1 class="machine-component-title">{ "Memory" }</h1>
                </header>
                <section class="memory-area">
                    <div class="memory">
                        { for rows }
                    </div>
                </section>
                <footer class="machine-data-footer">
                    <strong class="machine-data-footer-text">
                      { format!(" pointer: {}", machine_ref.memory.pointer).as_str() }
                    </strong>
                </footer>
            </div>
        </div>
    }
}
