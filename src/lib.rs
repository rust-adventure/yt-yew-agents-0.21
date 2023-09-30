pub mod oneshot;
use gloo_timers::callback::Interval;
use log::info;
use oneshot::FibonacciTask;
pub mod reactor;
use reactor::TimeFormatReactor;
use web_sys::HtmlInputElement;
use yew::{
    html::ChildrenRenderer, platform::spawn_local,
    prelude::*, virtual_dom::VNode,
};
use yew_agent::{
    oneshot::{use_oneshot_runner, OneshotProvider},
    reactor::{use_reactor_subscription, ReactorProvider},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <OneshotProvider<FibonacciTask> path="/worker.js">
        <ReactorProvider<TimeFormatReactor> path="/workerreactor.js">
        <main>
            <div>
                <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                <h1>{ "Yew 0.21 Agents" }</h1>
            </div>
            <OneShotExample/>
            <ReactorExample/>
            <RenderPropExample>
                {|p: RenderProps| html!{<>{"Hello, "}{p.name}</>}}
            </RenderPropExample>
        </main>
        </ReactorProvider<TimeFormatReactor>>
        </OneshotProvider<FibonacciTask>>
    }
}
#[function_component(ReactorExample)]
pub fn reactor_example() -> Html {
    let time_formatter =
        use_reactor_subscription::<TimeFormatReactor>();

    let tf = time_formatter.clone();
    use_effect_with((), move |_| {
        let mut i = 0;
        let interval = Interval::new(1_000, move || {
            tf.send(i);
            i = i + 1;
        });
        // leak the interval so it keeps running
        interval.forget();
    });
    let tf = time_formatter.clone();
    html! {
    <div class="ww-container">
        <div class="input">
        <button onclick={{

        move |_| {
            let tf = tf.clone();
            spawn_local(async move {
                info!("send 500 to reactor");
                tf.send(500);
            });
        }
        }}>{ "submit" }</button>
            <div>{ "time" }</div>
        </div>
        <div class="display">
            <h2>{ "reactor" }</h2>
            <p>{ "values are automatically sent"} </p>
            <h3>{ "Output: " } {
                time_formatter.iter().last().cloned().map(|value|{
                    let v = (*value).clone();
                    info!("last time is {}", v);
                    v
                })
            }</h3>
        </div>
    </div>
    }
}

#[function_component(OneShotExample)]
pub fn one_shot_example() -> Html {
    let input_value = use_state_eq(|| 44);
    let output = use_state(|| {
        "Try out some fibonacci calculations!".to_string()
    });
    let fib_task = use_oneshot_runner::<FibonacciTask>();

    let calculate = {
        let input_value = *input_value;
        let output = output.clone();
        move |_e: MouseEvent| {
            let fib_agent = fib_task.clone();
            let output = output.clone();

            spawn_local(async move {
                // start the worker
                let output_value =
                    fib_agent.run(input_value).await;

                output.set(format!(
                    "Fibonacci value: {}",
                    output_value
                ));
            });
        }
    };

    let on_input_change = {
        let input_value = input_value.clone();
        move |e: InputEvent| {
            input_value.set(
                e.target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .parse()
                    .expect("failed to parse"),
            );
        }
    };
    html! {
    <div class="ww-container">
        <div class="input">
            <input type="number" value={input_value.to_string()} max="50" oninput={on_input_change} />
            <button onclick={calculate}>{ "submit" }</button>
        </div>
        <div class="display">
            <h2>{ "oneshot" }</h2>
            <p>{ "Submit a value to calculate"} </p>
            <h3>{ "Output: " } { &*output }</h3>
        </div>
    </div>
        }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RenderProps {
    pub name: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct ComponentProps {
    pub children: Callback<RenderProps, Html>,
}

#[function_component(RenderPropExample)]
pub fn render_prop_example(p: &ComponentProps) -> Html {
    let render_props = RenderProps {
        name: "chris".into(),
    };
    html! {
        <div class="ww-container">
            <ul class="input">
                <li>
                    {p.children.emit(render_props.clone())}
                </li>
                <li>
                    {p.children.emit(render_props.clone())}
                </li>
                <li>
                    {p.children.emit(render_props)}
                </li>
            </ul>
            <div class="display">
                <h2>{"Render Props"}</h2>
            </div>
        </div>
    }
}
