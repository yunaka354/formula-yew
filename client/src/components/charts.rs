use crate::components::props::ChartProps;
use plotly::common::{Line, Marker};
use plotly::{Bar, Plot, Scatter, BoxPlot};
use rand_distr::{Uniform, Distribution};
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

#[function_component(LapBoxChart)]
pub fn lap_box_chart() -> Html {
    let chart_id = "lap-box-chart";
    let p = yew_hooks::use_async::<_, _, ()>({
        async move {
            let mut rng = rand::thread_rng();
            let uniform1 = Uniform::new(0.0, 1.0);
            let n = 50;
            let mut plot = Plot::new();
        
            for _ in 0..20 {
                let mut y = Vec::with_capacity(n);
                for _ in 0..n {
                    y.push(uniform1.sample(&mut rng));
                }
                let trace = BoxPlot::<f64, f64>::new(y);
                plot.add_trace(trace);
            }

            let layout = plotly::Layout::new().title(plotly::common::Title::new("Lap Time Box Plot"));
            plot.set_layout(layout);
            plotly::bindings::new_plot(chart_id, &plot).await;
            Ok(())
        }
    });

    use_effect_with(p.clone(), move |_| {
        p.run();
    });

    html! {
        <div id={chart_id}></div>
    }
}
