mod racer;

use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use racer::{Racer, Position};
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{console, window, HtmlDivElement, HtmlElement};

thread_local! {
    static RACER: Rc<RefCell<Racer>> = Rc::new(RefCell::new(Racer::new(10, 10)));

    static HANDLE_TICK: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        RACER.with(|racer| racer.borrow_mut().tick());
        render();
    }) as Box<dyn FnMut()>);
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Pew pew pew".into());

    HANDLE_TICK.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                500,
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    RACER.with(|racer| {
        let racer = racer.borrow();
        let document = window().unwrap_throw().document().unwrap_throw();
        let root_container = document
            .get_element_by_id("root")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        root_container.set_inner_html("");

        let width = racer.width;
        let height = racer.height;

        root_container
            .style()
            .set_property("display", "inline-grid")
            .unwrap_throw();
        root_container
            .style()
            .set_property(
                "grid-template",
                &format!("repeat({}, auto) / repeat({}, auto)", height, width),
            )
            .unwrap_throw();

        for y in 0..height {
            for x in 0..width {
                let pos = Position(x as i32, y as i32);
                let field_element = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap_throw();

                field_element.set_class_name("field");

                field_element.set_inner_text({
                    if pos == racer.ship.position {
                        "🟩"
                    } else {
                        " "
                    }
                });

                root_container.append_child(&field_element).unwrap_throw();
            }
        }
    });
}
