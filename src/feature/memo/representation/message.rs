//! representation 계층에서 사용하는 UI/버스 메시지 타입입니다.

use std::fmt::{self, Display, Formatter};

/// 메모 목록 정렬 옵션입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiSortOption {
    CreatedAtAsc,
    CreatedAtDesc,
    UpdatedAtAsc,
    UpdatedAtDesc,
}

impl UiSortOption {
    /// pick_list 렌더링에 사용하는 전체 옵션입니다.
    pub const ALL: [Self; 4] = [
        Self::CreatedAtAsc,
        Self::CreatedAtDesc,
        Self::UpdatedAtAsc,
        Self::UpdatedAtDesc,
    ];
}

impl Default for UiSortOption {
    fn default() -> Self {
        Self::UpdatedAtDesc
    }
}

impl Display for UiSortOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAtAsc => f.write_str("Created (asc)"),
            Self::CreatedAtDesc => f.write_str("Created (desc)"),
            Self::UpdatedAtAsc => f.write_str("Updated (asc)"),
            Self::UpdatedAtDesc => f.write_str("Updated (desc)"),
        }
    }
}

/// 뷰 계층에서 발생하는 사용자 입력 이벤트입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiMessage {
    AddClicked,
    MemoSelected(u64),
    DeleteClicked(u64),
    TitleChanged(String),
    ContentChanged(String),
    SortChanged(UiSortOption),
}

/// representation 내부 버스 이벤트입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusEvent {
    CreateMemo,
    SelectMemo(u64),
    DeleteMemo(u64),
    UpdateTitle(String),
    UpdateContent(String),
    ChangeSort(UiSortOption),
}
