# an unstyled calendar component for Leptos

This component provides a calendar component and a configuration hook to be able to apply styles to it.

## Example Usage

```rust

#[component]
fn Parent() -> into ImplView {
	let calendar_config = CalendarConfig {
		active_classes: Some("border-pink-400 border-2".to_string()),
		header_classes: Some("bg-slate-900 text-white".to_string()),
		month_classes: Some("bg-black p-3 text-center text-4xl font-normal".to_string()),
		cell_classes: Some("bg-black text-center py-3 px-4 border aspect-square".to_string()),
		..Default::default()
	};
	let (selected_date, _) = create_signal(
		NaiveDate::from_ymd_opt(2025, 1, 30)
				.and_then(|x| x.and_hms_opt(12, 12, 12))
				.and_then(|x| Some(x.and_local_timezone(Local).unwrap()))
				.unwrap(),
	);

	view! {
		<CalendarRoot config=calendar_config>
			<Calendar date=selected_date class="border-2 border-white bg-slate-50" />
		</CalendarRoot>
	}
}
```

## Remaining Work

- [ ] Add ability to click the cell to set the current date
- [ ] Move `cell_renderer` into a sub-component instead of config (if possible)
- [ ] Add doctests and documentation

## Please file issues

This isn't super well tested, so if you use it and run into problems or rough edges, [please file issues](https://github.com/graysonarts/leptos-calendar/issues)
