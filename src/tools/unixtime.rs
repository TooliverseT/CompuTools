use chrono::{
    DateTime, Datelike, FixedOffset, Local, NaiveDateTime, Offset, TimeZone, Timelike, Utc,
};
use chrono_tz::TZ_VARIANTS;
use gloo_timers::callback::Interval;
use js_sys::Date;
use log::info;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn getTimeZoneOffset() -> JsValue;
}

pub struct ToolUnixtime {
    selected_timezone: String,
    timezones: Vec<(String, i32)>,

    current_offset: i32,

    current_unixtime: i64,
    current_datetime: String,

    input_unixtime: i64,
    output_datetime: String,

    input_datetime: String,
    output_unixtime: i64,

    convert: bool,

    interval: Option<Interval>,
}

pub enum Msg {
    TimezoneSelect(String),
    UpdateUnixtime(String),
    UpdateDatetime(String),
    Convert,
    Tick,
    CopyToClipboard(String),
}

impl Component for ToolUnixtime {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (selected_timezone, current_offset) = ToolUnixtime::fetch_time_zone_offset();
        let input_unixtime = Date::now() as i64 / 1000;
        let current_unixtime = input_unixtime.clone();
        _ctx.link()
            .send_message(Msg::UpdateUnixtime(input_unixtime.to_string()));
        let current_datetime = ToolUnixtime::unixtime_to_datetime_str_form2(
            current_unixtime,
            selected_timezone.clone(),
        );

        let link = _ctx.link().clone();
        let interval = Interval::new(1000, move || {
            link.send_message(Msg::Tick);
        });

        let input_datetime = ToolUnixtime::unixtime_to_datetime_str_form3(
            current_unixtime,
            selected_timezone.clone(),
        );
        let output_unixtime = ToolUnixtime::datetime_str_to_unixtime(
            input_datetime.clone(),
            selected_timezone.clone(),
        )
        .unwrap_or(0);

        let timezones: Vec<(String, i32)> = TZ_VARIANTS
            .iter()
            .map(|tz| {
                let now = Utc::now();
                let offset = tz
                    .offset_from_utc_datetime(&now.naive_utc())
                    .fix()
                    .local_minus_utc();
                let hours = offset / 3600;
                let minutes = (offset % 3600) / 60;
                let offset_str = format!("UTC{:+03}:{:02}", hours, minutes.abs());
                (format!("{} ({})", tz.name(), offset_str), offset)
            })
            .collect();

        Self {
            selected_timezone,
            timezones,

            current_offset,

            current_unixtime,
            current_datetime,

            input_unixtime,
            output_datetime: String::new(),

            input_datetime,
            output_unixtime,

            convert: false,
            interval: Some(interval),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TimezoneSelect(value) => {
                self.selected_timezone = value;

                self.output_datetime = ToolUnixtime::unixtime_to_datetime_str_form1(
                    self.input_unixtime,
                    self.selected_timezone.clone(),
                );

                self.output_unixtime = ToolUnixtime::datetime_str_to_unixtime(
                    self.input_datetime.clone(),
                    self.selected_timezone.clone(),
                )
                .unwrap_or(0);

                self.current_datetime = ToolUnixtime::unixtime_to_datetime_str_form2(
                    self.current_unixtime,
                    self.selected_timezone.clone(),
                );

                true
            }
            Msg::UpdateUnixtime(value) => {
                let mut parsed_value = value.trim().parse::<i64>().unwrap_or(0);

                if parsed_value < 0 {
                    parsed_value = 0;
                } else if parsed_value > 100000000000 {
                    parsed_value = 0;
                }

                self.input_unixtime = parsed_value;
                self.output_datetime = ToolUnixtime::unixtime_to_datetime_str_form1(
                    self.input_unixtime,
                    self.selected_timezone.clone(),
                );
                true
            }
            Msg::UpdateDatetime(value) => {
                let mut value = value;
                if value.len() <= 16 {
                    // "YYYY-MM-DDTHH:MM" 길이 확인
                    value.push_str(":00");
                }

                self.input_datetime = value.clone();
                self.output_unixtime = ToolUnixtime::datetime_str_to_unixtime(
                    self.input_datetime.clone(),
                    self.selected_timezone.clone(),
                )
                .unwrap_or(0);
                true
            }
            Msg::Convert => {
                self.convert = !self.convert;
                true
            }
            Msg::Tick => {
                self.current_unixtime = Date::now() as i64 / 1000;
                self.current_datetime = ToolUnixtime::unixtime_to_datetime_str_form2(
                    self.current_unixtime,
                    self.selected_timezone.clone(),
                );
                true
            }
            Msg::CopyToClipboard(value) => {
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    wasm_bindgen_futures::spawn_local(async move {
                        let promise = clipboard.write_text(&value);
                        let future = JsFuture::from(promise);

                        match future.await {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    });
                } else {
                    {};
                }
                false // 리렌더링 필요 없음
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let convert = self.convert.clone();
        let on_convert = _ctx.link().callback(|_| Msg::Convert);
        let output_unixtime = self.output_unixtime.clone();
        let onchange_timezone = _ctx.link().callback(|e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            Msg::TimezoneSelect(select.value())
        });

        html! {
                <>
                <h1 class="tool-title">{ "Unix Timestamp Converter" }</h1>
                <div class="tool-wrapper">
                    <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"⏳ What is Unix Time?"}</h2>
                            <div style="display: flex; flex-direction: column; justify-content: center; align-items: center;">
                                <div class="unix-current">
                                    {self.current_unixtime}
                                </div>
                                <div class="date-current">
                                    {self.current_datetime.clone()}
                                </div>
                                <div class="date-timezone">
                                <select onchange={onchange_timezone}>
                                    {self.timezones.iter().map(|(name, _)| {
                                        let tz_name = name.split_whitespace().next().unwrap_or("");
                                        html! {
                                            <option
                                                value={tz_name.to_string()}
                                                selected={self.selected_timezone == tz_name}
                                            >
                                                {name.clone()}
                                            </option>
                                        }
                                    }).collect::<Html>()}
                                </select>
                                </div>
                            </div>
                            <p>{"Unix Time (or Unix Timestamp) is the number of seconds that have elapsed since January 1, 1970 (UTC). It is widely used in computing for time representation and storage."}</p>
                        </div>
                        <div class="content-section">
                            <h2>{"⚙️ How This Converter Works"}</h2>
                            <ul>
                                <li><strong>{"Unix Timestamp → Date & Time:"}</strong> {"Convert a Unix timestamp to a human-readable date and time."}</li>
                                <li><strong>{"Date & Time → Unix Timestamp:"}</strong> {"Convert a date and time to a Unix timestamp."}</li>
                                <li><strong>{"Timezone Support:"}</strong> {"Select your timezone for accurate conversions."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All calculations happen in your browser for privacy and speed."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"📚 Example"}</h2>
                            <div class="example-box">
                                <p><strong>{"Unix Timestamp input:"}</strong></p>
                                <ul><li>{"1700000000"}</li></ul>
                                <p><strong>{"Date & Time output (UTC):"}</strong></p>
                                <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"Nov 14, 2023, 10:13:20 AM"#}
                                </pre>
                            </div>
                        </div>
                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <ul>
                                <li><strong>{"Programming & Development:"}</strong> {"Convert timestamps for logging, scheduling, and debugging."}</li>
                                <li><strong>{"Data Analysis:"}</strong> {"Interpret and process time-based data in databases and files."}</li>
                                <li><strong>{"APIs & Webhooks:"}</strong> {"Work with APIs that use Unix timestamps for time fields."}</li>
                                <li><strong>{"Scheduling & Reminders:"}</strong> {"Convert between human-readable dates and Unix time for reminders and events."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What is the range of valid Unix timestamps?"}</h3>
                                <p>{"A: Typically from 0 (Jan 1, 1970) to 2147483647 (Jan 19, 2038) for 32-bit systems, but modern systems support much larger ranges."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Are results always in UTC?"}</h3>
                                <p>{"A: You can select your timezone for conversion. By default, results are shown in your local timezone."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I use this tool offline?"}</h3>
                                <p>{"A: Yes, all calculations are performed locally in your browser."}</p>
                            </div>
                        </div>
                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Validate Input:"}</strong> {"Ensure timestamps and dates are within valid ranges."}</li>
                                <li><strong>{"Timezone Awareness:"}</strong> {"Always check the timezone when converting times."}</li>
                                <li><strong>{"Copy Carefully:"}</strong> {"Double-check copied values before using in code or databases."}</li>
                                <li><strong>{"Test Edge Cases:"}</strong> {"Test with leap years, daylight saving changes, and far-future dates."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <p>{"Explore more tools for developers:"}</p>
                            // <ul>
                            //     <li><a href="/base/">{"Base Converter"}</a> {" - For converting numbers between different bases."}</li>
                            // </ul>
                        </div>
                    </div>
                    <div class="tool-container">
                        <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                            <div style="width: 90%;">
                                if !convert {
                                    {"Unix Timestamp to Date & Time"}
                                } else {
                                    {"Date & Time to Unix Timestamp"}
                                }
                            </div>
                            <div onclick={on_convert} class="tool-change" style="width: 10%;">
                                <i class="fa-solid fa-arrows-rotate"></i>
                            </div>
                        </div>
                        if !convert {
                            <div class="tool-inner">
                                <div>
                                    <div class="tool-subtitle">{ "Unix Timestamp" }</div>
                                    <input
                                        type="number"
                                        name="unixtime"
                                        inputmode="decimal"
                                        placeholder={format!("{}", self.input_unixtime)}
                                        autocomplete="off"
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateUnixtime(input.value())
                                        })} />
                                </div>
                            </div>
                            // TODO: Date Time 표현식 선택할 수 있게
                            <div class="tool-inner" style="margin-top: 10px;">
                                <div>
                                    <div class="tool-subtitle">{ "Date Time" }</div>
                                    <input
                                        type="text"
                                        name="date"
                                        readonly=true
                                        style="cursor: pointer;"
                                        value={format!("{}", self.output_datetime.clone())}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })} />
                                </div>
                            </div>
                        } else {
                            <div class="tool-inner">
                                <div>
                                    <div class="tool-subtitle">{ "Date Time" }</div>
                                    <input
                                        type="datetime-local"
                                        name="year"
                                        autocomplete="off"
                                        value={self.input_datetime.clone()}
                                        step="1"
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateDatetime(input.value())
                                        })} />
                                </div>
                            </div>
                            <div class="tool-inner" style="margin-top: 10px;">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Unix Timestamp" }</div>
                                    <input
                                        type="number"
                                        name="unixtime"
                                        readonly=true
                                        style="cursor: pointer;"
                                        value={format!("{}", output_unixtime)}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })} />
                                </div>
                            </div>
                        }
                    </div>
                </div>
                </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = window() {
                let document = window.document();
                if let Some(doc) = document {
                    doc.set_title("Unix Timestamp Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This tool allows you to convert Unix Timestamps to Date & Time formats and vice versa. Unix Timestamps represent the number of seconds since January 1, 1970 (UTC), while Date & Time offers a more readable format for daily use.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolUnixtime {
    pub fn unixtime_to_datetime_str_form1(unixtime: i64, selected_timezone: String) -> String {
        let naive = NaiveDateTime::from_timestamp_opt(unixtime, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());

        if let Ok(tz) = selected_timezone.parse::<chrono_tz::Tz>() {
            let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
            let local_time = datetime.with_timezone(&tz);
            format!("{}", local_time.format("%m/%d/%Y, %I:%M:%S %p"))
        } else {
            String::from("01/01/1970, 12:00:00 AM")
        }
    }

    pub fn unixtime_to_datetime_str_form2(unixtime: i64, selected_timezone: String) -> String {
        let naive = NaiveDateTime::from_timestamp_opt(unixtime, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());

        if let Ok(tz) = selected_timezone.parse::<chrono_tz::Tz>() {
            let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
            let local_time = datetime.with_timezone(&tz);
            format!("{}", local_time.format("%b %d, %Y, %I:%M:%S %p"))
        } else {
            String::from("Jan 01, 1970, 12:00:00 AM")
        }
    }

    pub fn unixtime_to_datetime_str_form3(unixtime: i64, selected_timezone: String) -> String {
        let naive = NaiveDateTime::from_timestamp_opt(unixtime, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());

        if let Ok(tz) = selected_timezone.parse::<chrono_tz::Tz>() {
            let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
            let local_time = datetime.with_timezone(&tz);
            format!("{}", local_time.format("%Y-%m-%dT%H:%M:%S"))
        } else {
            String::from("1970-01-01T00:00:00")
        }
    }

    pub fn fetch_time_zone_offset() -> (String, i32) {
        let js_value = getTimeZoneOffset(); // JavaScript 함수 호출
        let js_obj: js_sys::Object = js_value.into();

        // 타임존과 오프셋 추출
        let time_zone: String = js_sys::Reflect::get(&js_obj, &"timeZone".into())
            .unwrap()
            .as_string()
            .unwrap();
        let offset: i32 = js_sys::Reflect::get(&js_obj, &"offset".into())
            .unwrap()
            .as_f64()
            .unwrap() as i32;

        let offset_hour = -(offset / 60);

        (time_zone, offset_hour)
    }

    // pub fn datetime_str_to_unixtime(datetime_str: String, timezone_offset: i32) -> Option<i64> {
    //     let naive_datetime =
    //         NaiveDateTime::parse_from_str(datetime_str.as_str(), "%Y-%m-%dT%H:%M:%S").ok()?;
    //     let offset = FixedOffset::east_opt(timezone_offset * 3600)?;
    //     let datetime_with_offset = offset.from_local_datetime(&naive_datetime).single()?;
    //     Some(datetime_with_offset.timestamp())
    // }

    pub fn datetime_str_to_unixtime(
        datetime_str: String,
        selected_timezone: String,
    ) -> Option<i64> {
        let naive_datetime =
            NaiveDateTime::parse_from_str(datetime_str.as_str(), "%Y-%m-%dT%H:%M:%S").ok()?;

        // Parse the selected timezone
        let tz = selected_timezone.parse::<chrono_tz::Tz>().ok()?;

        // Get current offset for the timezone
        let offset = tz
            .offset_from_utc_datetime(&naive_datetime)
            .fix()
            .local_minus_utc();

        // Create DateTime with offset
        let fixed_offset = FixedOffset::east_opt(offset)?;
        let datetime_with_offset = fixed_offset.from_local_datetime(&naive_datetime).single()?;

        Some(datetime_with_offset.timestamp())
    }
}
