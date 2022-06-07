use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::{self, prelude::*};
use crate::web::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {

    html! {

            <div class="paper container margin-bottom-large" style="display: flex; flex-direction: column;">
            
                <InputBox />
                <DisplayBox/>

            </div>
        }
}

#[function_component(InputBox)]
pub fn input_box() -> Html {
    let text = Dispatch::<InputState>::new().get().text.clone();
    //let text = use_selector(|s: &InputState| s.text.clone() ).as_ref().clone();

    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
            let value = input.value();
            s.update_text(value);
    });

    html!(
        <div>
<p>

</p>

        <textarea id="input-textarea" name="input-textarea" class="input-textarea" rows="10" {oninput} 
        value={text}
        spellcheck="false"
        >
        </textarea>
        </div>
    )
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html{

    let svg = use_selector(|s: &ImageState| s.svg.clone() ).as_ref().clone();
    let err = use_selector(|s: &ImageState| s.error.clone() ).as_ref().clone() .unwrap_or("".to_string());


    html!(
        <>
        
        <iframe class="display-iframe" srcdoc={svg} scrolling="no"></iframe>
        <code> {err} </code>
        </>
)
}


