use log::info;
use std::f64::consts::PI;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, PartialEq)]
struct EulerAngles {
    roll: f64,
    pitch: f64,
    yaw: f64,
}

pub struct ToolQuaternion {
    quaternion: Quaternion,
    quaternion_res: Quaternion,
    euler: EulerAngles,
    convert: bool,
    convert_euler: EulerAngles,
    euler_res: EulerAngles,
    convert_quat: Quaternion,
}

pub enum Msg {
    UpdateQuaternion(String, String),
    UpdateEuler(String, String),
    Convert,
    CopyToClipboard(String),
}

impl Component for ToolQuaternion {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            quaternion: Quaternion {
                w: 1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            quaternion_res: Quaternion {
                w: 1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            euler: EulerAngles {
                roll: 0.0,
                pitch: 0.0,
                yaw: 0.0,
            },
            convert_euler: EulerAngles {
                roll: 0.0,
                pitch: 0.0,
                yaw: 0.0,
            },
            euler_res: EulerAngles {
                roll: 0.0,
                pitch: 0.0,
                yaw: 0.0,
            },
            convert_quat: Quaternion {
                w: 1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            convert: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateQuaternion(field, value) => {
                let trimmed_value = value.trim();

                // 파싱된 값 (숫자 형식이 아니면 기존 값 유지)
                if let Ok(parsed_value) = trimmed_value.parse::<f64>() {
                    match field.as_str() {
                        "w" => self.quaternion.w = parsed_value,
                        "x" => self.quaternion.x = parsed_value,
                        "y" => self.quaternion.y = parsed_value,
                        "z" => self.quaternion.z = parsed_value,
                        _ => {}
                    }
                }

                (self.euler, self.quaternion_res) =
                    self.quaternion_to_euler(self.quaternion.clone());

                true
            }
            Msg::UpdateEuler(field, value) => {
                let trimmed_value = value.trim();

                if let Ok(parsed_value) = trimmed_value.parse::<f64>() {
                    match field.as_str() {
                        "roll" => self.convert_euler.roll = parsed_value,
                        "pitch" => self.convert_euler.pitch = parsed_value,
                        "yaw" => self.convert_euler.yaw = parsed_value,
                        _ => {}
                    }
                }

                (self.euler_res, self.convert_quat) =
                    self.euler_to_quaternion(self.convert_euler.clone());

                true
            }
            Msg::Convert => {
                self.convert = !self.convert;
                true
            }
            Msg::CopyToClipboard(value) => {
                // input_ref에서 HtmlInputElement를 가져옴
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    // 클립보드 작업 수행
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
        let euler = self.euler.clone();
        let quaternion_res = self.quaternion_res.clone();
        let quaternion = self.convert_quat.clone();
        let euler_res = self.euler_res.clone();
        let on_convert = _ctx.link().callback(|_| Msg::Convert);

        html! {
                <>
                    <div class="tool-wrapper">
                        <div>
                            <div class="tool-title">
                                { "Quaternion Converter" }
                            </div>
                            <div class="tool-intro">
                                <p>
                                    { "This page provides a simple and intuitive tool for converting quaternions to Euler angles roll, pitch, yaw and vice versa. Quaternions, commonly used
                                    in 3D graphics, robotics, and aerospace, offer a robust way to represent rotations without
                                    the issues of gimbal lock. Euler angles, on the other hand, are often more intuitive for human interpretation." }
                                </p>
                                <p> {"With this tool, you can:"} </p>
                                <ul>
                                    <li>{"Convert a quaternion into its corresponding roll, pitch, and yaw values."}</li>
                                    <li>{"Transform Euler angles (roll, pitch, yaw) back into quaternion form."}</li>
                                </ul>
                                <p>
                                    { "The Euler angles follow the ZYX rotation order, meaning the rotations are applied in the sequence of yaw (rotation around the Z-axis), pitch (rotation around the Y-axis), and roll (rotation around the X-axis). This is a standard convention in robotics and many other fields."}
                                </p>
                                <p>
                                    { "Note:" }
                                </p>
                                <ul>
                                    <li>{" The input quaternion values (x, y, z, w) are automatically normalized to ensure the quaternion represents a valid rotation"}</li>
                                    <li>{"When converting Euler angles (roll, pitch, yaw) back into a quaternion, the input values for the Euler angles are constrained within the range of -π to π"}</li>
                                </ul>
                            </div>
                        </div>
                        <div class="tool-container">
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 90%;">
                                    if !convert {
                                        {"Quaternion to Euler"}
                                    } else {
                                        {"Euler to Quaternion"}
                                    }
                                </div>
                                <div onclick={on_convert} class="tool-change" style="width: 10%;">
                                    <i class="fa-solid fa-arrows-rotate"></i>
                                </div>
                            </div>
                            if !convert {
                                <div class="tool-inner">
                                    <div>
                                        <div class="tool-subtitle" style="margin-bottom: 5px;">{ "W" }</div>
                                        <div class="input-container">
                                            <div class="input-column">
                                                <input
                                                    type="number"
                                                    inputmode="decimal"
                                                    name="w"
                                                    placeholder=1
                                                    autocomplete="off"
                                                    step="any"
                                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        Msg::UpdateQuaternion("w".to_string(), input.value())
                                                    })} />
                                            </div>
                                            <div class="result-column">
                                                <span class="calculated-value">
                                                    { format!("{:.6}", quaternion_res.w) }
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "X" }</div>
                                        <div class="input-container">
                                            <div class="input-column">
                                                <input
                                                    type="number"
                                                    inputmode="decimal"
                                                    name="x"
                                                    placeholder=0
                                                    autocomplete="off"
                                                    step="any"
                                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        Msg::UpdateQuaternion("x".to_string(), input.value())
                                                    })} />
                                            </div>
                                            <div class="result-column">
                                                <span class="calculated-value">
                                                    { format!("{:.6}", quaternion_res.x) }
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Y" }</div>
                                        <div class="input-container">
                                            <div class="input-column">
                                                <input
                                                    type="number"
                                                    inputmode="decimal"
                                                    name="y"
                                                    placeholder=0
                                                    autocomplete="off"
                                                    step="any"
                                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        Msg::UpdateQuaternion("y".to_string(), input.value())
                                                    })} />
                                            </div>
                                            <div class="result-column">
                                                <span class="calculated-value">
                                                    { format!("{:.6}", quaternion_res.y) }
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Z" }</div>
                                        <div class="input-container">
                                            <div class="input-column">
                                                <input
                                                    type="number"
                                                    inputmode="decimal"
                                                    name="z"
                                                    placeholder=0
                                                    autocomplete="off"
                                                    step="any"
                                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        Msg::UpdateQuaternion("z".to_string(), input.value())
                                                    })} />
                                            </div>
                                            <div class="result-column">
                                                <span class="calculated-value">
                                                    { format!("{:.6}", quaternion_res.z) }
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="tool-inner" style="margin-top: 10px;">
                                    <div>
                                        <div class="tool-subtitle">{ "Roll (radian)" }</div>
                                        <input
                                            type="number"
                                            name="roll"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:.6}", euler.roll)}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })} />
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Pitch (radian)" }</div>
                                        <input
                                            type="number"
                                            name="pitch"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:.6}", euler.pitch)}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })} />
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Yaw (radian)" }</div>
                                        <input
                                            type="number"
                                            name="yaw"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:.6}", euler.yaw)}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })} />
                                    </div>
                                </div>
                            } else {
                                <div class="tool-inner">
                                    <div>
                                        <div class="tool-subtitle">{ "Roll (radian)" }</div>
                                        <div class="input-container">
                                            <div class="input-column">
                                                <input
                                                    type="number"
                                                    inputmode="decimal"
                                                    name="roll"
                                                    placeholder=0
                                                    autocomplete="off"
                                                    step="any"
                                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        Msg::UpdateEuler("roll".to_string(), input.value())
                                                    })} />
                                            </div>
                                            <div class="result-column">
                                                <span class="calculated-value">
                                                    { format!("{:.6}", euler_res.roll) }
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Pitch (radian)" }</div>
                                        <div class="input-container">
                                            <div class="input-column">
                                                <input
                                                    type="number"
                                                    inputmode="decimal"
                                                    name="pitch"
                                                    placeholder=0
                                                    autocomplete="off"
                                                    step="any"
                                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        Msg::UpdateEuler("pitch".to_string(), input.value())
                                                    })} />
                                            </div>
                                            <div class="result-column">
                                                <span class="calculated-value">
                                                    { format!("{:.6}", euler_res.pitch) }
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Yaw (radian)" }</div>
                                        <div class="input-container">
                                            <div class="input-column">
                                                <input
                                                    type="number"
                                                    inputmode="decimal"
                                                    name="yaw"
                                                    placeholder=0
                                                    autocomplete="off"
                                                    step="any"
                                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        Msg::UpdateEuler("yaw".to_string(), input.value())
                                                    })} />
                                            </div>
                                            <div class="result-column">
                                                <span class="calculated-value">
                                                    { format!("{:.6}", euler_res.yaw) }
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="tool-inner" style="margin-top: 10px;">
                                    <div>
                                        <div class="tool-subtitle" style="margin-bottom: 5px;">{ "W" }</div>
                                        <input
                                            type="number"
                                            name="w"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:.6}", quaternion.w)}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })} />
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "X" }</div>
                                        <input
                                            type="number"
                                            name="x"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:.6}", quaternion.x)}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })} />
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Y" }</div>
                                        <input
                                            type="number"
                                            name="y"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:.6}", quaternion.y)}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })} />
                                    </div>
                                    <div>
                                        <div class="tool-subtitle" style="margin-top: 15px;">{ "Z" }</div>
                                        <input
                                            type="number"
                                            name="z"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:.6}", quaternion.z)}
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
                    doc.set_title("Quaternion Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This page provides a simple and intuitive tool for converting quaternions to Euler angles roll, pitch, yaw and vice versa.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolQuaternion {
    pub fn quaternion_to_euler(&self, q: Quaternion) -> (EulerAngles, Quaternion) {
        let qw = q.w;
        let qx = q.x;
        let qy = q.y;
        let qz = q.z;

        let norm = (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
        if norm != 0.0 {
            let x = qx / norm;
            let y = qy / norm;
            let z = qz / norm;
            let w = qw / norm;

            let sinr_cosp = 2.0 * (w * x + y * z);
            let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
            let roll = sinr_cosp.atan2(cosr_cosp);

            let sinp = 2.0 * (w * y - z * x);
            let pitch = if sinp.abs() >= 1.0 {
                PI / 2.0 * sinp.signum()
            } else {
                sinp.asin()
            };

            let siny_cosp = 2.0 * (w * z + x * y);
            let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
            let yaw = siny_cosp.atan2(cosy_cosp);

            (EulerAngles { roll, pitch, yaw }, Quaternion { x, y, z, w })
        } else {
            let x = 0.0;
            let y = 0.0;
            let z = 0.0;
            let w = 1.0;

            (
                EulerAngles {
                    roll: 0.0,
                    pitch: 0.0,
                    yaw: 0.0,
                },
                Quaternion { x, y, z, w },
            )
        }
    }

    pub fn euler_to_quaternion(&self, e: EulerAngles) -> (EulerAngles, Quaternion) {
        let roll = self.normalize_angle(e.roll);
        let pitch = self.normalize_angle(e.pitch);
        let yaw = self.normalize_angle(e.yaw);

        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        (EulerAngles { roll, pitch, yaw }, Quaternion { x, y, z, w })
    }

    fn normalize_angle(&self, angle: f64) -> f64 {
        let mut norm_angle = angle % (2.0 * PI);
        if norm_angle > PI {
            norm_angle -= 2.0 * PI;
        } else if norm_angle <= -PI {
            norm_angle += 2.0 * PI;
        }
        norm_angle
    }
}
