//! 메모 앱 뷰를 구성합니다.

use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::{Element, Length};

use super::message::UiMessage;
use super::state::AppState;

pub fn root(state: &AppState) -> Element<'_, UiMessage> {
    let left = container(left_panel(state))
        .width(Length::FillPortion(2))
        .height(Length::Fill);
    let right = container(right_panel(state))
        .width(Length::FillPortion(3))
        .height(Length::Fill);

    container(row![left, right].spacing(16))
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
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
        items = items.push(text("메모가 없습니다."));
    } else {
        for memo in &state.memos {
            let title = if memo.title.trim().is_empty() {
                "(제목 없음)".to_string()
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
        return column![text("메모를 선택하세요")].height(Length::Fill);
    };

    column![
        row![
            text(format!("Memo #{selected_id}")),
            button("Delete").on_press(UiMessage::DeleteClicked(selected_id))
        ]
        .spacing(8),
        text_input("Title", &state.detail.title_input)
            .on_input(UiMessage::TitleChanged)
            .padding(8),
        text_input("Content", &state.detail.content_input)
            .on_input(UiMessage::ContentChanged)
            .padding(8)
    ]
    .spacing(12)
    .height(Length::Fill)
}
