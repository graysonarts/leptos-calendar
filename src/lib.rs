use std::{cell::Cell, collections::HashMap};

use chrono::{prelude::*, Duration};
use itertools::Itertools as _;
use leptos::*;

#[derive(Clone, Debug, Default)]
pub struct CalendarConfig {
    pub header_classes: Option<String>,
    pub cell_classes: Option<String>,
    pub month_classes: Option<String>,
    pub active_classes: Option<String>,
    pub cell_renderer: Option<fn(NaiveDate) -> String>,
}

#[derive(Clone, Debug, Default)]
struct CalendarState {
    active_date: MaybeSignal<DateTime<Local>>,
}

#[component]
pub fn CalendarRoot(
    #[prop(into, optional)] config: MaybeSignal<CalendarConfig>,
    children: Children,
) -> impl IntoView {
    provide_context(config);
    view! { {children()} }
}

/// A calendar component that displays a month view.
#[component]
pub fn Calendar(
    #[prop(into)] date: MaybeSignal<DateTime<Local>>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    provide_context(CalendarState { active_date: date });
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
      <table class=move || class.get()>
        <thead>
          <Header date=date />
        </thead>
        <tbody>
          <For
            each=move || dates().into_iter().sorted_by_key(|(_week, days)| days[0])
            key=|key| key.1.get(0).unwrap().to_string()
            children=|(week, days)| {
              view! { <Week week=move || week days=move || days.clone() /> }
            }
          />
        </tbody>
      </table>
    }
}

#[component]
fn Week(
    #[prop(into)] week: Signal<u32>,
    #[prop(into)] days: Signal<Vec<NaiveDate>>,
) -> impl IntoView {
    let active_date = move || expect_context::<CalendarState>().active_date.get();
    let start_padding = move || {
        days.get()
            .get(0)
            .as_ref()
            .unwrap()
            .weekday()
            .num_days_from_monday()
    };
    let cell_classes = move || {
        let config = expect_context::<MaybeSignal<CalendarConfig>>();
        config.get().cell_classes.clone().unwrap_or_default()
    };
    let active_classes = move || {
        let config = expect_context::<MaybeSignal<CalendarConfig>>();
        config.get().active_classes.clone().unwrap_or_default()
    };
    let cell_renderer = move || {
        let config = expect_context::<MaybeSignal<CalendarConfig>>();
        config
            .get()
            .cell_renderer
            .clone()
            .unwrap_or(|day| day.format("%d").to_string())
    };
    view! {
      <tr>
        <Show when=move || (start_padding() > 0)>
          <td colSpan=move || start_padding() />
        </Show>
        <For
          each=move || days.get()
          key=|day| day.to_string()
          children=move |day| {
            let is_active = move || day == active_date().date_naive();
            let cell_classes = move || format!(
              "{} {}",
              cell_classes(),
              if is_active() { active_classes() } else { "".to_owned() },
            );
            view! { <td class=cell_classes>{cell_renderer()(day)}</td> }
          }
        />
      </tr>
    }
}

#[component]
fn Header(#[prop(into)] date: MaybeSignal<DateTime<Local>>) -> impl IntoView {
    let header_classes = move || {
        let config = expect_context::<MaybeSignal<CalendarConfig>>();
        config.get().header_classes.clone().unwrap_or_default()
    };
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
      <tr class=header_classes>
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
