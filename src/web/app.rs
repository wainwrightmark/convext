use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::{prelude::*, self};
use crate::web::prelude::*;
use itertools::Itertools;
use web_sys::{HtmlTextAreaElement, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {

        <div class="paper container margin-bottom-large" style="display: flex; flex-direction: column;">

            <InputBox />
            <ErrorBox />
            <DisplayBox/>
            <SlidersControl/>

        </div>
    }
}

#[function_component(SlidersControl)]
pub fn sliders_control() -> Html {
    let properties = use_selector(|state: &InputState| state.grammar.get_variables());

    let result = properties.iter().map(|p| {
        let prop_key = p.0.clone();
        let p_type = p.1;

        html!(<InputSlider  {p_type} {prop_key} />)
    });
    html!({for result})
}

#[derive(Properties, PartialEq)]
pub struct InputSliderProperties {
    pub prop_key: String,
    #[prop_or(None)]
    pub p_type: Option<PropertyType>,
}

#[function_component(InputSlider)]
pub fn input_slider(properties: &InputSliderProperties) -> Html {
    let key = properties.prop_key.clone();

    let value = use_selector_with_deps(|state: &InputState, k| state.get_variable_value(k), key.clone()).as_ref().clone();

    if let Some(p_type) = properties.p_type {
        let (min, max, step) = p_type.deconstruct();

        let key2 = key.clone();
        let key3 = key.clone();

        let on_slider_input = Dispatch::<InputState>::new().reduce_mut_callback_with(move |s, e: InputEvent|{
            let input : HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            let new_f_value:f32 = new_value.parse().unwrap();
            s.set_variable_value(key2.clone(), new_f_value);
        });
        
        let on_box_input = Dispatch::<InputState>::new().reduce_mut_callback_with(move |s, e: InputEvent|{
            let input : HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            let new_f_value:f32 = new_value.parse().unwrap();
            s.set_variable_value(key3.clone(), new_f_value);
        });

        html!(
                <div class="slider">

            <code style="width:100px" >{format!("{}", key)}</code>
          <input oninput={on_slider_input} type="range"  value={format!("{}",value )} min={format!("{}",min )} max={format!("{}",max )}  step={format!("{}",step )} />
          <input  oninput={on_box_input} type="number"  value={format!("{}",value )} min={format!("{}",min )} max={format!("{}",max )}  step={format!("{}",step )} />
          
          
        </div>
            )
    } else {
        html!(
                <div class="slider">

          <input type="range"  value={format!("{}",value )} disabled=true />
          <code >{format!("{}: {}", key, value)}</code>
        </div>
            )
    }
}

#[function_component(InputBox)]
pub fn input_box() -> Html {
    let text = Dispatch::<InputState>::new().get().text.clone();
    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        let value = input.value();
        s.update_text(value);
    });

    html!(
            <div>
    <p>

    </p>
    //https://css-tricks.com/creating-an-editable-textarea-that-supports-syntax-highlighted-code/
            <textarea id="input-textarea" name="input-textarea" class="input-textarea" rows="10" {oninput}
            value={text}
            spellcheck="false"
            >
            </textarea>
            </div>
        )
}

#[function_component(ErrorBox)]
pub fn erorr_box() -> Html {
    let err = use_selector(|s: &InputState| s.error.clone())
        .as_ref()
        .clone()
        .unwrap_or("‎".to_string());
    html!(<code> {err} </code>)
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html {
    let svg = use_selector(|s: &ImageState| s.svg.clone())
        .as_ref()
        .clone();

    html!(
        <iframe class="display-iframe" srcdoc={svg} scrolling="no"></iframe>
    )
}
