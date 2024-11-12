use std::collections::HashMap;

use chrono::{prelude::*, Duration};
use itertools::Itertools as _;
use leptos::*;

#[derive(Clone, Debug, Default)]
pub struct CalendarConfig {
    pub cell_classes: Option<String>,
    pub month_classes: Option<String>,
}

#[component]
pub fn CalendarRoot(
    #[prop(into, optional)] config: MaybeSignal<CalendarConfig>,
    #[prop(into)] date: MaybeSignal<DateTime<Local>>,
) -> impl IntoView {
    provide_context(config);
    let dates = move || {
        let date = date.get();
        let first_day = date.with_day(1).unwrap().date_naive();
        let first_day_next_month = (first_day + Duration::days(35)).with_day(1).unwrap();

        let mut days: HashMap<u32, Vec<NaiveDate>> = HashMap::new();
        let mut current_day = first_day;
        while current_day < first_day_next_month {
            let week = current_day.iso_week().week();
            days.entry(week).or_insert_with(Vec::new).push(current_day);
            current_day += Duration::days(1);
        }
        return days;
    };

    view! {
      <table>
        <thead>
          <Header date=date />
        </thead>
        <tbody>
          <For
            each=move || dates().into_iter().sorted_by_key(|(week, _)| *week)
            key=|key| key.0.to_string()
            children=|(_week, days)| {
              view! { <Week days=days /> }
            }
          />
        </tbody>
      </table>
    }
}

#[component]
fn Week(#[prop(into)] days: Vec<NaiveDate>) -> impl IntoView {
    let start_padding = days
        .get(0)
        .as_ref()
        .unwrap()
        .weekday()
        .num_days_from_monday();
    let cell_classes = move || {
        let config = expect_context::<MaybeSignal<CalendarConfig>>();
        config.get().cell_classes.clone().unwrap_or_default()
    };
    view! {
      <tr>
        <Show when=move || (start_padding > 0)>
          <td colSpan={start_padding} />
        </Show>
        <For
          each=move || days.clone()
          key=|day| day.to_string()
          children=move |day| {
            view! { <td class=cell_classes.clone()>{day.format("%d").to_string()}</td> }
          }
        />
      </tr>
    }
}

#[component]
fn Header(#[prop(into)] date: MaybeSignal<DateTime<Local>>) -> impl IntoView {
    let month_classes = move || {
        let config = expect_context::<MaybeSignal<CalendarConfig>>();
        config.get().month_classes.clone().unwrap_or_default()
    };
    let cell_classes = move || {
        let config = expect_context::<MaybeSignal<CalendarConfig>>();
        config.get().cell_classes.clone().unwrap_or_default()
    };
    let days = ["M", "T", "W", "T", "F", "S", "S"].map(|day| day.to_string());
    let month_year = move || date.get().format("%B %Y").to_string();

    view! {
      <tr>
        <th colSpan=7 class=month_classes>
          {month_year}
        </th>
      </tr>
      <tr>
        <For
          each=move || days.clone()
          key=|day| day.to_owned()
          children=move |day| {
            view! { <th class=cell_classes.clone()>{day.clone()}</th> }
          }
        />
      </tr>
    }
}
