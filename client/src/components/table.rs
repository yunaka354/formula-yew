use yew::prelude::*;

use crate::components::props::{ColumnHeaderProps, TableProps, TableRowProps};
use crate::models::{Race, RaceResult};

#[derive(Clone, PartialEq, Properties)]
pub struct TableContents {
    pub contents: Vec<TableColumn>,
}

impl TableContents {
    pub fn convert_races(races: Vec<Race>) -> Self {
        let mut contents = Vec::new();
        contents.push(TableColumn {
            headers: "Season".to_string(),
            rows: races.iter().map(|race| html! {race.season}).collect(),
        });
        contents.push(TableColumn {
            headers: "Round".to_string(),
            rows: races.iter().map(|race| html! {race.round}).collect(),
        });
        contents.push(TableColumn {
            headers: "Title".to_string(),
            rows: races
                .iter()
                .map(|race| {
                    html! {
                        <a
                            href={format!("/results?year={}&round={}", race.season, race.round)}
                            class="text-blue-500 hover:text-blue-700"
                        >
                            { &race.race_name }
                        </a>
                    }
                })
                .collect(),
        });
        contents.push(TableColumn {
            headers: "Circuit".to_string(),
            rows: races
                .iter()
                .map(|race| html! {&race.circuit_name})
                .collect(),
        });
        contents.push(TableColumn {
            headers: "Date".to_string(),
            rows: races.iter().map(|race| html! {&race.date}).collect(),
        });
        Self { contents }
    }

    pub fn convert_race_result(race_results: Vec<RaceResult>) -> Self {
        let mut contents = Vec::new();
        contents.push(TableColumn {
            headers: "Position".to_string(),
            rows: race_results
                .iter()
                .map(|race_result| html! {race_result.position})
                .collect(),
        });
        contents.push(TableColumn {
            headers: "Code".to_string(),
            rows: race_results
                .iter()
                .map(|race_result| html! {race_result.code.clone()})
                .collect(),
        });
        contents.push(TableColumn {
            headers: "Driver".to_string(),
            rows: race_results.iter().map(|race_result| html!{format!("{} {}", race_result.given_name.clone(), race_result.family_name.clone())}).collect(),
        });
        contents.push(TableColumn {
            headers: "Team".to_string(),
            rows: race_results
                .iter()
                .map(|race_result| html! {race_result.constructor.clone()})
                .collect(),
        });
        contents.push(TableColumn {
            headers: "Points".to_string(),
            rows: race_results
                .iter()
                .map(|race_result| html! {race_result.points})
                .collect(),
        });
        contents.push(TableColumn {
            headers: "Status".to_string(),
            rows: race_results
                .iter()
                .map(|race_result| html! {race_result.status.clone()})
                .collect(),
        });
        Self { contents }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableColumn {
    pub headers: String,
    pub rows: Vec<Html>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let columns = props.columns();
    let rows = props.rows();
    html! {
        <table class="min-w-full table-auto border-collapse border border-gray-200">
            <thead class="bg-gray-100">
                <tr>
                    {
                        for columns.iter().map(|column| {
                            html! {
                                <ColumnHeader title={ column.headers.clone() } />
                            }
                        })
                    }
                </tr>
            </thead>
            <tbody>
                {
                    for rows.iter().map(|row| {
                        html! {
                            <tr>
                                {
                                    for row.iter().map(|row| {
                                        html! {
                                            <TableRow content={ row.clone() } />
                                        }
                                    })
                                }
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>

    }
}

#[function_component(ColumnHeader)]
pub fn column_header(props: &ColumnHeaderProps) -> Html {
    let props = props.clone();
    let title = props.title.clone();

    html! {
        <th
            class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
        >
            { title }
        </th>
    }
}

#[function_component(TableRow)]
pub fn table_row(props: &TableRowProps) -> Html {
    let props = props.clone();

    html! {
        <td class="border-b border-gray-200 px-4 py-2 text-sm text-gray-700">
            { props.content.clone() }
        </td>
    }
}
