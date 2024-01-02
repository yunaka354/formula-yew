use crate::components::props::ChartProps;
use plotly::{common::Marker, Bar, Plot};
use yew::prelude::*;

#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let props = props.clone();
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";

        async move {
            let mut plot = Plot::new();
            let x = props.chart_data.x.clone();
            let y = props.chart_data.y.clone();
            let colors = props.chart_data.color.clone();
            for (i, key) in x.iter().enumerate() {
                let bar = Bar::new(vec![key.clone()], vec![y[i]]);
                let bar = bar.marker(Marker::new().color(colors[i].clone()));
                plot.add_trace(bar);
            }
            let layout = plotly::Layout::new()
                .title(plotly::common::Title::new("Standings"))
                .show_legend(false);
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
