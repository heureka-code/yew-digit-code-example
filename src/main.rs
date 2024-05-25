use yew::prelude::*;

use yew_digit_code::TotpInput;

#[derive(Debug, Properties, PartialEq, Eq)]
pub struct Props {
    #[prop_or(false)]
    pub clear_on_submit: bool,
    #[prop_or(false)]
    pub focus_first_on_submit: bool,
}

#[function_component(DisplayCode)]
pub fn display_code<const LENGTH: usize>(
    Props {
        clear_on_submit,
        focus_first_on_submit,
    }: &Props,
) -> Html {
    let state = use_state(|| None::<String>);
    let flags = use_state(|| {
        if !*clear_on_submit && *focus_first_on_submit {
            yew_digit_code::ControlFlags::default()
                .change()
                .focus_first()
                .apply()
        } else {
            yew_digit_code::ControlFlags::default()
        }
    });

    let oninit = Callback::from(move |_| log::info!("Init finished callback"));

    let cloned_state = state.clone();
    let cloned_flags = flags.clone();
    let (clear, focus) = (*clear_on_submit, *focus_first_on_submit);
    let submit = Callback::from(move |code| {
        let state = cloned_state.clone();
        let (clear, focus) = (clear, focus);
        let flags = cloned_flags.clone();

        if clear || focus {
            let mut f = flags.change();
            if clear {
                f = f.clear();
            }
            if focus {
                f = f.focus_first();
            }
            flags.set(f.apply());
        }

        state.set(Some(code));
    });
    html!(
        <div class={classes!("example-container")}>
            <TotpInput<LENGTH> submit_code={submit} oninit={oninit} flags={flags}/> // Don't forget to set the flags
            {
                if state.is_some(){
                    html!(<code>{"Submit: "}{(*state).clone()}</code>)
                } else {html!()}
            }
        </div>
    )
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div class={classes!("main-container")}>
            <div class={classes!("inner-container")}>
            <h1>{"No flags"}</h1>
            <DisplayCode<9>/>

            <h1>{"Clear flag"}</h1>
            <DisplayCode<9> clear_on_submit={true}/>

            <h1>{"Focus first flag"}</h1>
            <DisplayCode<9> focus_first_on_submit={true}/>

            <h1>{"Clear and focus first flag"}</h1>
            <DisplayCode<9> clear_on_submit={true} focus_first_on_submit={true}/>
            </div>
        </div>
    )
}

fn main() {
    dotenv::dotenv().ok();

    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
