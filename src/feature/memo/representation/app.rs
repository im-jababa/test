//! iced application 오케스트레이션입니다.

use iced::widget::text_input;
use iced::{Size, Task, Theme, application};

use super::bus;
use super::message::UiMessage;
use super::state::AppState;
use super::view;

pub fn run() -> iced::Result {
    application("Memo", update, view::root)
        .theme(theme)
        .window_size(Size::new(900.0, 600.0))
        .resizable(true)
        .run_with(|| (AppState::default(), Task::none()))
}

fn theme(_state: &AppState) -> Theme {
    Theme::Light
}

fn update(state: &mut AppState, message: UiMessage) -> Task<UiMessage> {
    let focus_new_memo_title = matches!(message, UiMessage::AddClicked);

    for event in bus::ui_to_bus_events(message) {
        bus::apply_event(state, event);
    }

    if focus_new_memo_title {
        return text_input::focus(view::title_input_id());
    }

    Task::none()
}
