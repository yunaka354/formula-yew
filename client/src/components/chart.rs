use crate::components::props::ChartProps;
use plotly::{Bar, Plot};
use yew::prelude::*;

#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let props = props.clone();
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";

        async move {
            let mut plot = Plot::new();
            let trace = Bar::new(props.chart_data.x.clone(), props.chart_data.y.clone());
            plot.add_trace(trace);

            let layout = plotly::Layout::new().title(plotly::common::Title::new("Standings"));
            plot.set_layout(layout);
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });

    use_effect_with(p.clone(), move |_| {
        p.run();
    });

    html! {
        <div id="plot-div"></div>
    }
}
