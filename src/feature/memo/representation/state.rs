//! representation 계층 전용 앱 상태(VM)입니다.

use super::message::UiSortOption;

/// 메모 목록 아이템 뷰모델입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoItemVm {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at_order: u64,
    pub updated_at_order: u64,
}

/// 우측 상세 영역 뷰모델입니다.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DetailVm {
    pub title_input: String,
    pub content_input: String,
}

/// 메모 앱의 representation 상태입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub memos: Vec<MemoItemVm>,
    pub selected_id: Option<u64>,
    pub detail: DetailVm,
    pub sort: UiSortOption,
    pub next_id: u64,
    pub next_order: u64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            memos: Vec::new(),
            selected_id: None,
            detail: DetailVm::default(),
            sort: UiSortOption::default(),
            next_id: 1,
            next_order: 1,
        }
    }
}

impl AppState {
    pub fn issue_next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn issue_next_order(&mut self) -> u64 {
        let order = self.next_order;
        self.next_order += 1;
        order
    }

    pub fn selected_memo_index(&self) -> Option<usize> {
        let selected_id = self.selected_id?;
        self.memos.iter().position(|memo| memo.id == selected_id)
    }

    pub fn selected_memo_id(&self) -> Option<u64> {
        self.selected_id
    }

    pub fn sync_detail_from_selection(&mut self) {
        if let Some(index) = self.selected_memo_index() {
            let memo = &self.memos[index];
            self.detail.title_input = memo.title.clone();
            self.detail.content_input = memo.content.clone();
            return;
        }
        self.clear_detail();
    }

    pub fn clear_detail(&mut self) {
        self.detail.title_input.clear();
        self.detail.content_input.clear();
    }

    pub fn sort_memos(&mut self) {
        let option = self.sort;
        self.memos.sort_by(|left, right| match option {
            UiSortOption::CreatedAtAsc => left
                .created_at_order
                .cmp(&right.created_at_order)
                .then_with(|| left.id.cmp(&right.id)),
            UiSortOption::CreatedAtDesc => right
                .created_at_order
                .cmp(&left.created_at_order)
                .then_with(|| right.id.cmp(&left.id)),
            UiSortOption::UpdatedAtAsc => left
                .updated_at_order
                .cmp(&right.updated_at_order)
                .then_with(|| left.created_at_order.cmp(&right.created_at_order))
                .then_with(|| left.id.cmp(&right.id)),
            UiSortOption::UpdatedAtDesc => right
                .updated_at_order
                .cmp(&left.updated_at_order)
                .then_with(|| right.created_at_order.cmp(&left.created_at_order))
                .then_with(|| right.id.cmp(&left.id)),
        });
    }
}
