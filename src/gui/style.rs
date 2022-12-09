//! Module defining the application styles: containers, picklists, buttons, radios, scrollbars.

use crate::enums::element_type::ElementType;
use crate::enums::style_type::StyleType;
use crate::get_colors;
use crate::structs::colors::Colors;
use crate::utility::style_constants::{
    BORDER_BUTTON_RADIUS, BORDER_ROUNDED_RADIUS, BORDER_WIDTH, BORDER_WIDTH_TABS,
};
use iced::widget::{button, container::StyleSheet, pick_list};
use iced::{Background, Color, Vector};
use iced_style::application::Appearance;
use iced_style::scrollable::{Scrollbar, Scroller};
use iced_style::Theme;

/// This tuple permits to specify the correct style depending on the style type and on the element type
pub struct StyleTuple(pub StyleType, pub ElementType);

// impl From<Colors> for iced_style::theme::Theme {
//     fn from(colors: Colors) -> Self {
//         iced_style::theme::Theme::Custom(colors)
//     }
// }

impl iced::application::StyleSheet for Colors {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background_color: self.primary,
            text_color: self.text_body,
        }
    }
}

// /// Containers style
// impl StyleSheet for StyleTuple {
//     type Style = ();
//
//     fn appearance(&self, _: Style) -> Style {
//         let colors = get_colors(self.0);
//         Style {
//             text_color: Option::Some(match self {
//                 StyleTuple(_, ElementType::Headers) => colors.text_headers,
//                 _ => colors.text_body,
//             }),
//             background: Option::Some(Background::Color(match self {
//                 StyleTuple(_, ElementType::Headers) => colors.secondary,
//                 _ => colors.primary,
//             })),
//             border_radius: match self {
//                 StyleTuple(_, ElementType::BorderedRound) => BORDER_ROUNDED_RADIUS,
//                 _ => 0.0,
//             },
//             border_width: match self {
//                 StyleTuple(_, ElementType::Standard | ElementType::Headers) => 0.0,
//                 _ => BORDER_WIDTH,
//             },
//             border_color: colors.round_borders,
//         }
//     }
// }
//
// /// Picklists style
// impl pick_list::StyleSheet for StyleTuple {
//     type Style = ();
//
//     fn active(&self, _: Style) -> pick_list::Style {
//         let colors = get_colors(self.0);
//         pick_list::Style {
//             text_color: colors.text_body,
//             placeholder_color: colors.text_body,
//             background: Background::Color(colors.buttons),
//             border_radius: 0.0,
//             border_width: BORDER_WIDTH,
//             border_color: colors.secondary,
//             icon_size: 0.5,
//         }
//     }
//
//     fn hovered(&self, _: Style) -> pick_list::Style {
//         let colors = get_colors(self.0);
//         pick_list::Style {
//             text_color: colors.text_body,
//             placeholder_color: colors.text_body,
//             background: Background::Color(colors.primary),
//             border_radius: 0.0,
//             border_width: BORDER_WIDTH,
//             border_color: colors.secondary,
//             icon_size: 0.5,
//         }
//     }
// }

impl From<Colors> for iced::theme::Button {
    fn from(colors: Colors) -> Self {
        iced_style::theme::Button::Custom(Box::new(colors))
    }
}

/// Buttons style
impl button::StyleSheet for Colors {
    type Style = iced_style::Theme;
    // primary => standard
    // secondary => inactive tabs
    // positive => active tabs

    fn active(&self, kind: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match kind {
                // &Self::Style::Positive => self.primary,
                // _ => self.buttons,
                Theme::Custom(_) | Theme::Light => {self.secondary}
                _ => Color::BLACK
            })),
            border_radius: match kind {
                // &Self::Style::Secondary | &Self::Style::Positive => 0.0,
                // _ => BORDER_BUTTON_RADIUS,
                Theme::Custom(_) => {BORDER_WIDTH}
                _ => BORDER_WIDTH
            },
            border_width: match kind {
                // &Self::Style::Secondary | &Self::Style:Positive => {
                //     BORDER_WIDTH_TABS
                // }
                // _ => BORDER_WIDTH,
                Theme::Custom(_) => {BORDER_WIDTH}
                _ => BORDER_WIDTH
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: self.text_body,
            border_color: match kind {
                // &Self::Style::Secondary | &Style::Positive => self.buttons,
                // _ => self.secondary,
                Theme::Custom(_) => {self.primary}
                _ => Color::BLACK
            },
        }
    }

    fn hovered(&self, kind: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::new(2.0, 2.0),
            background: Some(Background::Color(self.primary)),
            border_radius: match kind {
                // &Self::Style::Secondary | &iced::theme::Button::Positive => 0.0,
                // _ => BORDER_BUTTON_RADIUS,
                Theme::Custom(_) => {BORDER_WIDTH}
                _ => BORDER_WIDTH
            },
            border_width: BORDER_WIDTH,
            border_color: match kind {
                // &Self::Style::Secondary | &iced::theme::Button::Positive => self.buttons,
                // _ => self.secondary,
                Theme::Custom(_) => {self.primary}
                _ => Color::BLACK
            },
            text_color: self.text_body,
        }
    }
}

// /// Radios style
// impl iced_style::radio::StyleSheet for StyleTuple {
//     type Style = ();
//
//     fn active(&self, _: ()) -> () {
//         let colors = get_colors(self.0);
//         iced_style::radio::Style {
//             background: Background::Color(colors.buttons),
//             dot_color: colors.secondary,
//             border_width: match self {
//                 StyleTuple(_, ElementType::SelectedRadio) => BORDER_WIDTH,
//                 _ => 0.0,
//             },
//             border_color: colors.secondary,
//             text_color: match self {
//                 StyleTuple(_, ElementType::SelectedRadio) => Some(colors.secondary),
//                 _ => None,
//             },
//         }
//     }
//
//     fn hovered(&self, _: Style) -> () {
//         let colors = get_colors(self.0);
//         iced_style::radio::Style {
//             background: Background::Color(colors.buttons),
//             dot_color: colors.secondary,
//             border_width: BORDER_WIDTH,
//             border_color: colors.secondary,
//             text_color: Some(colors.secondary),
//         }
//     }
// }
//
// /// Scrollbars style
// impl iced_style::scrollable::StyleSheet for StyleTuple {
//     fn active(&self) -> Scrollbar {
//         let colors = get_colors(self.0);
//         Scrollbar {
//             background: Some(Background::Color(colors.buttons)),
//             border_radius: BORDER_ROUNDED_RADIUS,
//             border_width: 0.0,
//             border_color: colors.round_borders,
//             scroller: Scroller {
//                 color: colors.primary,
//                 border_radius: BORDER_ROUNDED_RADIUS,
//                 border_width: BORDER_WIDTH / 1.5,
//                 border_color: colors.round_borders,
//             },
//         }
//     }
//
//     fn hovered(&self) -> Scrollbar {
//         let colors = get_colors(self.0);
//         Scrollbar {
//             background: Some(Background::Color(colors.buttons)),
//             border_radius: BORDER_ROUNDED_RADIUS,
//             border_width: BORDER_WIDTH / 1.5,
//             border_color: colors.round_borders,
//             scroller: Scroller {
//                 color: colors.secondary,
//                 border_radius: BORDER_ROUNDED_RADIUS,
//                 border_width: BORDER_WIDTH / 1.5,
//                 border_color: colors.round_borders,
//             },
//         }
//     }
// }
