use crate::components::props::ChartProps;
use plotly::common::{Line, Marker};
use plotly::{Bar, Plot, Scatter};
use yew::prelude::*;

use crate::components::props::LapLineChartProps;

#[function_component(StandingChart)]
pub fn standing_chart(props: &ChartProps<String, i32>) -> Html {
    let props = props.clone();
    let chart_id = props.plot_id.clone();
    let p = yew_hooks::use_async::<_, _, ()>({
        async move {
            let mut plot = Plot::new();
            let x = props.chart_data.x.clone();
            let y = props.chart_data.y.clone();
            let colors = props.chart_data.color.clone().unwrap();
            for (i, key) in x.iter().enumerate() {
                let bar = Bar::new(vec![key.clone()], vec![y[i]]);
                let bar = bar.marker(Marker::new().color(colors[i].clone()));
                plot.add_trace(bar);
            }
            let layout = plotly::Layout::new()
                .title(plotly::common::Title::new("Standings"))
                .show_legend(false);
            plot.set_layout(layout);
            plotly::bindings::new_plot(&props.plot_id, &plot).await;
            Ok(())
        }
    });

    use_effect_with(p.clone(), move |_| {
        p.run();
    });

    html! {
        <div id={chart_id.clone()}></div>
    }
}

#[function_component(LapChart)]
pub fn lap_chart(props: &LapLineChartProps) -> Html {
    let props = props.clone();
    let chart_id = props.plot_id.clone();
    let p = yew_hooks::use_async::<_, _, ()>({
        async move {
            let mut plot = Plot::new();

            for line in props.chart_data.iter() {
                let x = line.laps.clone();
                let y = line.laptime.clone();
                let color = line.color.clone();
                let bar = Scatter::new(x, y)
                    .mode(plotly::common::Mode::Lines)
                    .name(line.driver_id.clone())
                    .line(Line::new().color(color).width(2.0));
                plot.add_trace(bar);
            }

            let layout = plotly::Layout::new().title(plotly::common::Title::new("Lap Times"));
            plot.set_layout(layout);
            plotly::bindings::new_plot(&props.plot_id, &plot).await;
            Ok(())
        }
    });

    use_effect_with(p.clone(), move |_| {
        p.run();
    });

    html! {
        <div id={chart_id.clone()}></div>
    }
}
