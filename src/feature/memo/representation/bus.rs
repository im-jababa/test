//! UI 메시지를 내부 이벤트로 바꾸고 상태를 갱신하는 버스/리듀서입니다.

use super::message::{BusEvent, UiMessage};
use super::state::{AppState, MemoItemVm};

/// UI 메시지를 내부 버스 이벤트로 변환합니다.
pub fn ui_to_bus_events(message: UiMessage) -> Vec<BusEvent> {
    match message {
        UiMessage::AddClicked => vec![BusEvent::CreateMemo],
        UiMessage::MemoSelected(id) => vec![BusEvent::SelectMemo(id)],
        UiMessage::DeleteClicked(id) => vec![BusEvent::DeleteMemo(id)],
        UiMessage::TitleChanged(title) => vec![BusEvent::UpdateTitle(title)],
        UiMessage::ContentChanged(content) => vec![BusEvent::UpdateContent(content)],
        UiMessage::SortChanged(sort) => vec![BusEvent::ChangeSort(sort)],
    }
}

/// 버스 이벤트를 상태에 적용합니다.
pub fn apply_event(state: &mut AppState, event: BusEvent) {
    match event {
        BusEvent::CreateMemo => {
            let id = state.issue_next_id();
            let order = state.issue_next_order();
            state.memos.push(MemoItemVm {
                id,
                title: String::new(),
                content: String::new(),
                created_at_order: order,
                updated_at_order: order,
            });
            state.selected_id = Some(id);
            state.sort_memos();
            state.sync_detail_from_selection();
        }
        BusEvent::SelectMemo(id) => {
            if state.memos.iter().any(|memo| memo.id == id) {
                state.selected_id = Some(id);
                state.sync_detail_from_selection();
            } else {
                state.selected_id = None;
                state.clear_detail();
            }
        }
        BusEvent::DeleteMemo(id) => {
            let before = state.memos.len();
            state.memos.retain(|memo| memo.id != id);
            let removed = state.memos.len() != before;
            if removed && state.selected_id == Some(id) {
                state.selected_id = None;
                state.clear_detail();
            }
        }
        BusEvent::UpdateTitle(title) => {
            let Some(id) = state.selected_memo_id() else {
                return;
            };
            let updated_at_order = state.issue_next_order();
            if let Some(memo) = state.memos.iter_mut().find(|memo| memo.id == id) {
                memo.title = title.clone();
                memo.updated_at_order = updated_at_order;
            }
            state.detail.title_input = title;
            state.sort_memos();
        }
        BusEvent::UpdateContent(content) => {
            let Some(id) = state.selected_memo_id() else {
                return;
            };
            let updated_at_order = state.issue_next_order();
            if let Some(memo) = state.memos.iter_mut().find(|memo| memo.id == id) {
                memo.content = content.clone();
                memo.updated_at_order = updated_at_order;
            }
            state.detail.content_input = content;
            state.sort_memos();
        }
        BusEvent::ChangeSort(sort) => {
            state.sort = sort;
            state.sort_memos();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::memo::representation::message::UiSortOption;
    use crate::feature::memo::representation::state::MemoItemVm;

    fn seed_state() -> AppState {
        let mut state = AppState {
            memos: vec![
                MemoItemVm {
                    id: 1,
                    title: "first".to_string(),
                    content: "content-1".to_string(),
                    created_at_order: 1,
                    updated_at_order: 3,
                },
                MemoItemVm {
                    id: 2,
                    title: "second".to_string(),
                    content: "content-2".to_string(),
                    created_at_order: 2,
                    updated_at_order: 2,
                },
            ],
            selected_id: None,
            detail: Default::default(),
            sort: UiSortOption::UpdatedAtDesc,
            next_id: 3,
            next_order: 4,
        };
        state.sort_memos();
        state
    }

    #[test]
    fn message_to_bus() {
        let events = ui_to_bus_events(UiMessage::MemoSelected(3));
        assert_eq!(events, vec![BusEvent::SelectMemo(3)]);
    }

    #[test]
    fn reducer_select_syncs_detail() {
        let mut state = seed_state();
        apply_event(&mut state, BusEvent::SelectMemo(2));

        assert_eq!(state.selected_id, Some(2));
        assert_eq!(state.detail.title_input, "second");
        assert_eq!(state.detail.content_input, "content-2");
    }

    #[test]
    fn reducer_create_adds_and_selects() {
        let mut state = AppState::default();
        apply_event(&mut state, BusEvent::CreateMemo);

        assert_eq!(state.memos.len(), 1);
        assert_eq!(state.selected_id, Some(1));
        assert_eq!(state.next_id, 2);
        assert_eq!(state.next_order, 2);
    }

    #[test]
    fn reducer_sort_change_reorders_list() {
        let mut state = seed_state();

        apply_event(&mut state, BusEvent::ChangeSort(UiSortOption::CreatedAtAsc));
        assert_eq!(state.memos.iter().map(|memo| memo.id).collect::<Vec<_>>(), vec![1, 2]);

        apply_event(&mut state, BusEvent::ChangeSort(UiSortOption::CreatedAtDesc));
        assert_eq!(state.memos.iter().map(|memo| memo.id).collect::<Vec<_>>(), vec![2, 1]);
    }
}
