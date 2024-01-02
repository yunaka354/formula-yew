use crate::components::table::TableContents;
use crate::models::StandingsBarChart;

use yew::prelude::*;

use super::table::TableColumn;

#[derive(Properties, PartialEq, Clone)]
pub struct DetailProps {
    pub year: String,
    pub round: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    pub chart_data: StandingsBarChart,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ColumnHeaderProps {
    pub title: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableProps {
    pub contents: TableContents,
}

impl TableProps {
    pub fn columns(&self) -> Vec<TableColumn> {
        self.contents.contents.clone()
    }

    pub fn rows(&self) -> Vec<Vec<Html>> {
        let columns = self.contents.contents.clone();
        let rows = self.dynamic_zip(columns.iter().map(|column| column.rows.clone()).collect());
        rows
    }

    fn dynamic_zip(&self, vectors: Vec<Vec<Html>>) -> Vec<Vec<Html>> {
        if vectors.is_empty() {
            return vec![];
        }

        let min_length = vectors.iter().map(|v| v.len()).min().unwrap_or(0);
        let mut result = Vec::with_capacity(min_length);

        for i in 0..min_length {
            let mut temp = Vec::with_capacity(vectors.len());
            for vec in &vectors {
                temp.push(vec[i].clone());
            }
            result.push(temp);
        }

        result
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableRowProps {
    pub content: Html,
}
