//! 메모 앱 뷰를 구성합니다.

use iced::widget::{
    button, column, container, opaque, pick_list, row, scrollable, stack, text, text_input,
};
use iced::{Element, Length};

use super::message::UiMessage;
use super::state::AppState;

const TITLE_INPUT_ID: &str = "memo-title-input";

pub fn title_input_id() -> text_input::Id {
    text_input::Id::new(TITLE_INPUT_ID)
}

pub fn root(state: &AppState) -> Element<'_, UiMessage> {
    let left = container(left_panel(state))
        .width(Length::FillPortion(2))
        .height(Length::Fill);
    let right = container(right_panel(state))
        .width(Length::FillPortion(3))
        .height(Length::Fill);

    let base: Element<'_, UiMessage> = container(row![left, right].spacing(16))
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    if let Some(id) = state.pending_delete_id {
        return stack([base, confirm_delete_dialog(state, id)])
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
    }

    base
}

fn left_panel(state: &AppState) -> iced::widget::Column<'_, UiMessage> {
    let header = row![
        button("Add Memo").on_press(UiMessage::AddClicked),
        pick_list(
            super::message::UiSortOption::ALL.as_slice(),
            Some(state.sort),
            UiMessage::SortChanged,
        )
        .placeholder("Sort")
    ]
    .spacing(8);

    let mut items = column![].spacing(8);
    if state.memos.is_empty() {
        items = items.push(text("No memos yet."));
    } else {
        for memo in &state.memos {
            let title = if memo.title.trim().is_empty() {
                "(Untitled)".to_string()
            } else {
                memo.title.clone()
            };
            let label = if state.selected_id == Some(memo.id) {
                format!("> {title}")
            } else {
                title
            };
            items = items.push(
                row![
                    button(text(label))
                        .width(Length::Fill)
                        .on_press(UiMessage::MemoSelected(memo.id)),
                    button("Delete").on_press(UiMessage::DeleteClicked(memo.id))
                ]
                .spacing(8),
            );
        }
    }

    column![header, scrollable(items).height(Length::Fill)]
        .spacing(12)
        .height(Length::Fill)
}

fn right_panel(state: &AppState) -> iced::widget::Column<'_, UiMessage> {
    let Some(selected_id) = state.selected_id else {
        return column![text("Select a memo")].height(Length::Fill);
    };

    column![
        row![
            text(format!("Memo #{selected_id}")),
            button("Delete").on_press(UiMessage::DeleteClicked(selected_id))
        ]
        .spacing(8),
        text_input("Title", &state.detail.title_input)
            .id(title_input_id())
            .on_input(UiMessage::TitleChanged)
            .padding(8),
        text_input("Content", &state.detail.content_input)
            .on_input(UiMessage::ContentChanged)
            .padding(8)
    ]
    .spacing(12)
    .height(Length::Fill)
}

fn confirm_delete_dialog(state: &AppState, id: u64) -> Element<'_, UiMessage> {
    let target_title = state
        .memos
        .iter()
        .find(|memo| memo.id == id)
        .map(|memo| {
            if memo.title.trim().is_empty() {
                "(Untitled)".to_string()
            } else {
                memo.title.clone()
            }
        })
        .unwrap_or_else(|| format!("Memo #{id}"));

    let dialog = container(
        column![
            text("Delete this memo?"),
            text(format!("Target: {target_title}")),
            row![
                button("Cancel").on_press(UiMessage::DeleteCanceled),
                button("Delete").on_press(UiMessage::DeleteConfirmed)
            ]
            .spacing(8)
        ]
        .spacing(12),
    )
    .padding(16)
    .width(Length::Fixed(320.0))
    .style(iced::widget::container::rounded_box);

    opaque(
        container(dialog)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
}
