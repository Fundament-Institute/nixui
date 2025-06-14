// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2025 Fundament Research Institute <https://fundament.institute>

use feather_macro::*;
use feather_ui::component::button::Button;
use feather_ui::component::listbox::ListBox;
use feather_ui::component::region::Region;
use feather_ui::component::shape::Shape;
use feather_ui::component::text::Text;
use feather_ui::component::textbox::TextBox;
use feather_ui::component::window::Window;
use feather_ui::component::{ComponentFrom, mouse_area, textbox};
use feather_ui::layout::{base, fixed, leaf, list};
use feather_ui::persist::FnPersist;
use feather_ui::text::{EditObj, Snapshot};
use feather_ui::{
    AUTO_DRECT, AbsRect, App, DAbsRect, DPoint, DRect, DataID, FILL_DRECT, RelRect, Slot, SourceID,
    UNSIZED_AXIS, URect, gen_id,
};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use ultraviolet::{Vec2, Vec4};

#[derive(PartialEq, Clone, Debug, Default)]
struct ConfigState {
    file: String,
    nix: BTreeMap<String, Snapshot>,
    last: HashMap<String, String>,
}

#[derive(Default, Empty, Area, Anchor, ZIndex, Limits, RLimits, Padding, Margin)]
struct FixedData {
    area: DRect,
    anchor: DPoint,
    limits: feather_ui::DLimits,
    rlimits: feather_ui::RelLimits,
    padding: DAbsRect,
    zindex: i32,
    margin: DRect,
}

impl base::Order for FixedData {}
impl fixed::Prop for FixedData {}
impl fixed::Child for FixedData {}
impl leaf::Prop for FixedData {}
impl leaf::Padded for FixedData {}
impl list::Child for FixedData {}

#[derive(Default, Empty, Area, Direction, RLimits)]
struct ListData {
    area: DRect,
    direction: feather_ui::RowDirection,
    rlimits: feather_ui::RelLimits,
}

impl base::Limits for ListData {}
impl list::Prop for ListData {}
impl fixed::Child for ListData {}

#[derive(
    Clone,
    feather_macro::Empty,
    feather_macro::Area,
    feather_macro::TextEdit,
    feather_macro::Padding,
)]
struct MinimalText {
    area: DRect,
    padding: DAbsRect,
    textedit: Snapshot,
}
impl base::Direction for MinimalText {}
impl base::ZIndex for MinimalText {}
impl base::Limits for MinimalText {}
impl base::RLimits for MinimalText {}
impl base::Anchor for MinimalText {}
impl leaf::Padded for MinimalText {}
impl leaf::Prop for MinimalText {}
impl fixed::Child for MinimalText {}
impl textbox::Prop for MinimalText {}

struct BasicApp {}

impl FnPersist<ConfigState, im::HashMap<Rc<SourceID>, Option<Window>>> for BasicApp {
    type Store = (ConfigState, im::HashMap<Rc<SourceID>, Option<Window>>);

    fn init(&self) -> Self::Store {
        (Default::default(), im::HashMap::new())
    }
    fn call(
        &self,
        mut store: Self::Store,
        args: &ConfigState,
    ) -> (Self::Store, im::HashMap<Rc<SourceID>, Option<Window>>) {
        if store.0 != *args {
            let mut children: im::Vector<Option<Box<ComponentFrom<dyn list::Prop>>>> =
                im::Vector::new();

            for (i, (k, v)) in args.nix.iter().enumerate() {
                let mut parts: im::Vector<Option<Box<ComponentFrom<dyn fixed::Prop>>>> =
                    im::Vector::new();

                let rect = Shape::<DRect>::round_rect(
                    Rc::new(gen_id!())
                        .child(DataID::Owned(k.to_string()))
                        .into(),
                    feather_ui::FILL_DRECT.into(),
                    1.0,
                    0.0,
                    Vec4::broadcast(8.0),
                    Vec4::new(0.1, 0.1, 0.1, 1.0),
                    Vec4::new(0.3, 0.3, 0.3, 1.0),
                );

                let text = Text::<FixedData> {
                    id: Rc::new(gen_id!())
                        .child(DataID::Owned(k.to_string()))
                        .into(),
                    props: Rc::new(FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 0.0, 8.0, 0.0),
                            rel: RelRect::new(0.0, 0.0, UNSIZED_AXIS, UNSIZED_AXIS),
                        }
                        .into(),
                        ..Default::default()
                    }),
                    text: format!("[{}]", k),
                    font_size: 40.0,
                    line_height: 56.0,
                    ..Default::default()
                };

                let toggle = if v.get().get_content().eq_ignore_ascii_case("true") {
                    Some(true)
                } else if v.get().get_content().eq_ignore_ascii_case("false") {
                    Some(false)
                } else {
                    None
                };

                const GREEN: Vec4 = Vec4::new(0.2, 0.7, 0.4, 1.0);
                const GRAY: Vec4 = Vec4::new(0.45, 0.45, 0.45, 1.0);

                if let Some(v) = toggle {
                    let button = {
                        let mut children: im::Vector<Option<Box<ComponentFrom<dyn fixed::Prop>>>> =
                            im::Vector::new();

                        let rect = Shape::<DRect>::round_rect(
                            Rc::new(gen_id!())
                                .child(DataID::Owned(k.to_string()))
                                .into(),
                            feather_ui::FILL_DRECT.into(),
                            3.0,
                            0.0,
                            Vec4::broadcast(14.0),
                            Vec4::zero(),
                            if v { GREEN } else { GRAY },
                        );

                        let circle = Shape::<FixedData>::circle(
                            Rc::new(gen_id!())
                                .child(DataID::Owned(k.to_string()))
                                .into(),
                            FixedData {
                                area: URect {
                                    abs: AbsRect::new(0.0, 0.0, 20.0, 20.0),
                                    rel: if v {
                                        RelRect::new(1.0, 0.5, 1.0, 0.5)
                                    } else {
                                        RelRect::new(0.0, 0.5, 0.0, 0.5)
                                    },
                                }
                                .into(),
                                anchor: if v {
                                    feather_ui::UPoint::new(
                                        Vec2::new(5.0, 0.0),
                                        feather_ui::RelPoint::new(1.0, 0.5),
                                    )
                                } else {
                                    feather_ui::UPoint::new(
                                        Vec2::new(-5.0, 0.0),
                                        feather_ui::RelPoint::new(0.0, 0.5),
                                    )
                                }
                                .into(),
                                ..Default::default()
                            }
                            .into(),
                            0.0,
                            0.0,
                            Vec2::new(0.0, 0.0),
                            if v { GREEN } else { GRAY },
                            Vec4::zero(),
                        );

                        children.push_back(Some(Box::new(circle)));
                        children.push_back(Some(Box::new(rect)));

                        Button::<FixedData>::new(
                            Rc::new(gen_id!())
                                .child(DataID::Owned(k.to_string()))
                                .into(),
                            FixedData {
                                area: URect {
                                    abs: AbsRect::new(-10.0, 0.0, 35.0, 30.0),
                                    rel: RelRect::new(1.0, 0.5, 1.0, 0.5),
                                }
                                .into(),
                                anchor: feather_ui::RelPoint(Vec2 { x: 1.0, y: 0.5 }).into(),
                                ..Default::default()
                            },
                            Slot(feather_ui::APP_SOURCE_ID.into(), i as u64 + 2),
                            children,
                        )
                    };
                    parts.push_back(Some(Box::new(button)));
                } else {
                    let textbox = TextBox::new(
                        Rc::new(gen_id!())
                            .child(DataID::Owned(k.to_string()))
                            .into(),
                        MinimalText {
                            area: URect {
                                abs: AbsRect::new(10.0, 0.0, -10.0, 0.0),
                                rel: RelRect::new(0.5, 0.0, 1.0, 1.0),
                            }
                            .into(),
                            padding: AbsRect::broadcast(4.0).into(),
                            textedit: v.clone(),
                        },
                        30.0,
                        42.0,
                        glyphon::FamilyOwned::SansSerif,
                        glyphon::Color::rgba(255, 255, 255, 255),
                        Default::default(),
                        Default::default(),
                        glyphon::Wrap::Word,
                    );
                    parts.push_back(Some(Box::new(textbox)));

                    let rect = Shape::<DRect>::round_rect(
                        Rc::new(gen_id!())
                            .child(DataID::Owned(k.to_string()))
                            .into(),
                        Rc::new(
                            URect {
                                abs: AbsRect::new(6.0, 6.0, -6.0, -6.0),
                                rel: RelRect::new(0.5, 0.0, 1.0, 1.0),
                            }
                            .into(),
                        ),
                        1.0,
                        0.0,
                        Vec4::broadcast(8.0),
                        Vec4::new(0.05, 0.05, 0.05, 1.0),
                        Vec4::new(0.4, 0.4, 0.4, 1.0),
                    );
                    parts.push_back(Some(Box::new(rect)));
                }

                parts.push_back(Some(Box::new(text)));

                parts.push_back(Some(Box::new(rect)));

                children.push_back(Some(Box::new(Region {
                    id: Rc::new(gen_id!())
                        .child(DataID::Owned(k.to_string()))
                        .into(),
                    props: FixedData {
                        area: URect::from(RelRect::new(0.0, 0.0, 1.0, UNSIZED_AXIS)).into(),
                        padding: AbsRect::broadcast(16.0).into(),
                        margin: AbsRect::broadcast(4.0).into(),
                        ..Default::default()
                    }
                    .into(),
                    children: parts,
                })));
            }

            let list = ListBox::<ListData> {
                id: gen_id!().into(),
                props: ListData {
                    area: URect {
                        abs: AbsRect::new(8.0, 68.0, -8.0, -8.0),
                        rel: RelRect::new(0.0, 0.0, 1.0, UNSIZED_AXIS),
                    }
                    .into(),
                    rlimits: feather_ui::RelLimits::new(
                        Vec2::zero(),
                        Vec2::new(f32::INFINITY, f32::INFINITY),
                    ),
                    direction: feather_ui::RowDirection::TopToBottom,
                }
                .into(),
                children,
            };

            let accept = {
                let text = Text::<FixedData> {
                    id: gen_id!().into(),
                    props: Rc::new(FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 0.0, 8.0, 0.0),
                            rel: RelRect::new(0.0, 0.5, UNSIZED_AXIS, UNSIZED_AXIS),
                        }
                        .into(),
                        anchor: feather_ui::RelPoint(Vec2 { x: 0.0, y: 0.5 }).into(),
                        ..Default::default()
                    }),
                    text: "Accept Changes".into(),
                    font_size: 40.0,
                    line_height: 56.0,
                    ..Default::default()
                };

                let mut children: im::Vector<Option<Box<ComponentFrom<dyn fixed::Prop>>>> =
                    im::Vector::new();
                children.push_back(Some(Box::new(text)));

                let rect = Shape::<DRect>::round_rect(
                    gen_id!().into(),
                    feather_ui::FILL_DRECT.into(),
                    0.0,
                    0.0,
                    Vec4::broadcast(10.0),
                    Vec4::new(0.2, 0.7, 0.4, 1.0),
                    Vec4::zero(),
                );
                children.push_back(Some(Box::new(rect)));

                Button::<FixedData>::new(
                    gen_id!().into(),
                    FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 8.0, 8.0, 48.0),
                            rel: RelRect::new(0.0, 0.0, UNSIZED_AXIS, 0.0),
                        }
                        .into(),
                        ..Default::default()
                    },
                    Slot(feather_ui::APP_SOURCE_ID.into(), 0),
                    children,
                )
            };

            let discard = {
                let text = Text::<FixedData> {
                    id: gen_id!().into(),
                    props: Rc::new(FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 0.0, 8.0, 0.0),
                            rel: RelRect::new(0.0, 0.5, UNSIZED_AXIS, UNSIZED_AXIS),
                        }
                        .into(),
                        anchor: feather_ui::RelPoint(Vec2 { x: 0.0, y: 0.5 }).into(),
                        ..Default::default()
                    }),
                    text: "Discard Changes".into(),
                    font_size: 40.0,
                    line_height: 56.0,
                    ..Default::default()
                };

                let mut children: im::Vector<Option<Box<ComponentFrom<dyn fixed::Prop>>>> =
                    im::Vector::new();
                children.push_back(Some(Box::new(text)));

                let rect = Shape::<DRect>::round_rect(
                    gen_id!().into(),
                    feather_ui::FILL_DRECT.into(),
                    0.0,
                    0.0,
                    Vec4::broadcast(10.0),
                    Vec4::new(0.7, 0.2, 0.4, 1.0),
                    Vec4::zero(),
                );
                children.push_back(Some(Box::new(rect)));

                Button::<FixedData>::new(
                    gen_id!().into(),
                    FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 8.0, 8.0, 48.0),
                            rel: RelRect::new(0.5, 0.0, UNSIZED_AXIS, 0.0),
                        }
                        .into(),
                        ..Default::default()
                    },
                    Slot(feather_ui::APP_SOURCE_ID.into(), 1),
                    children,
                )
            };

            let mut children: im::Vector<Option<Box<ComponentFrom<dyn fixed::Prop>>>> =
                im::Vector::new();

            children.push_back(Some(Box::new(list)));
            children.push_back(Some(Box::new(accept)));
            children.push_back(Some(Box::new(discard)));

            let region = Region {
                id: gen_id!().into(),
                props: FixedData {
                    area: RelRect::new(0.0, 0.0, 1.0, UNSIZED_AXIS).into(),
                    ..Default::default()
                }
                .into(),
                children,
            };

            let window = Window::new(
                gen_id!().into(),
                winit::window::Window::default_attributes()
                    .with_title("NixUI")
                    .with_resizable(true),
                Box::new(region),
            );

            store.1 = im::HashMap::new();
            store.1.insert(window.id.clone(), Some(window));
            store.0 = args.clone();
        }
        let windows = store.1.clone();
        (store, windows)
    }
}

use feather_ui::WrapEventEx;

fn main() {
    /*
    let optionfile = match nix_data::cache::nixos::nixosoptions().unwrap();
     */
    const TEST_PATH: &str = "configuration.nix";

    let f = std::fs::read_to_string(Path::new(TEST_PATH)).unwrap();
    let nix = nix_editor::parse::get_collection(f.clone()).unwrap();
    let mut buttons: Vec<
        Box<
            (
                dyn FnMut(
                        (u64, Box<(dyn std::any::Any + 'static)>),
                        ConfigState,
                    ) -> Result<ConfigState, ConfigState>
                    + 'static
            ),
        >,
    > = Vec::new();

    buttons.push(Box::new(
        move |_: mouse_area::MouseAreaEvent,
              mut appdata: ConfigState|
              -> Result<ConfigState, ConfigState> {
            let mut s = appdata.file;
            for (k, v) in &appdata.nix {
                if let Some(prev) = appdata.last.get(k) {
                    if prev.eq_ignore_ascii_case(&v.get().get_content()) {
                        continue;
                    }
                }

                s = nix_editor::write::write(&s, &format!("{}", k), &v.get().get_content())
                    .unwrap();
            }
            appdata.file = s;

            std::fs::write(TEST_PATH, appdata.file.clone()).unwrap();
            appdata.last = nix_editor::parse::get_collection(appdata.file.clone()).unwrap();
            appdata.nix = appdata
                .last
                .iter()
                .map(|(k, v)| (k.clone(), EditObj::new(v.clone(), (0, 0)).into()))
                .collect();

            Ok(appdata)
        }
        .wrap(),
    ));

    buttons.push(Box::new(
        move |_: mouse_area::MouseAreaEvent,
              mut appdata: ConfigState|
              -> Result<ConfigState, ConfigState> {
            appdata.nix = appdata
                .last
                .iter()
                .map(|(k, v)| (k.clone(), EditObj::new(v.clone(), (0, 0)).into()))
                .collect();

            Ok(appdata)
        }
        .wrap(),
    ));

    for i in 0..nix.len() {
        let onclick = Box::new(
            move |_: mouse_area::MouseAreaEvent,
                  mut appdata: ConfigState|
                  -> Result<ConfigState, ConfigState> {
                if let Some((_, v)) = appdata.nix.iter_mut().nth(i) {
                    if v.get().get_content().eq_ignore_ascii_case("true") {
                        v.get().set_content("false");
                    } else if v.get().get_content().eq_ignore_ascii_case("false") {
                        v.get().set_content("true");
                    }
                    *v = v.clone();
                }

                Ok(appdata)
            }
            .wrap(),
        );
        buttons.push(onclick);
    }

    let (mut app, event_loop): (App<ConfigState, BasicApp>, winit::event_loop::EventLoop<()>) =
        App::new(
            ConfigState {
                file: f,
                nix: nix
                    .iter()
                    .map(|(k, v)| (k.clone(), EditObj::new(v.clone(), (0, 0)).into()))
                    .collect(),
                last: nix,
            },
            buttons,
            BasicApp {},
        )
        .unwrap();

    event_loop.run_app(&mut app).unwrap();
}
