use std::cmp::Ordering;
use std::sync::atomic::AtomicU64;
use chrono::{DateTime, Utc};
use super::id::ID;
use super::sort_option::SortOption;
/// 새 메모 생성 시 사용할 다음 ID를 추적하는 정적 카운터입니다.
static NEXT_ID: AtomicU64 = AtomicU64::new(Memo::INITIAL_NEXT_ID);

/// 메모 도메인 엔티티입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memo {
    /// 메모 고유 식별자입니다.
    pub id: ID,

    /// 메모 제목입니다.
    pub title: String,

    /// 메모 본문 내용입니다.
    pub content: String,

    /// 메모 생성 시각(UTC)입니다.
    pub created_at: DateTime<Utc>,

    /// 메모 마지막 변경 시각(UTC)입니다.
    pub updated_at: DateTime<Utc>,
}

impl Memo {
    /// 자동 발급되는 메모 ID의 시작값입니다.
    const INITIAL_NEXT_ID: u64 = 1;

    /// 제목과 내용으로 새 메모를 생성합니다.
    ///
    /// 이 함수는 ID를 자동 발급하고 `created_at`, `updated_at`를 자동 생성해야 합니다.
    /// 현재 구현은 미완성으로 `todo!()` 상태입니다.
    ///
    pub fn new(title: String, content: String) -> Self {
        let _ = (title, content, &NEXT_ID);
        todo!()
    }

    /// 메모 제목을 갱신합니다.
    ///
    /// 이 함수는 제목 변경과 함께 `updated_at`를 자동 갱신해야 합니다.
    /// 현재 구현은 미완성으로 `todo!()` 상태입니다.
    ///
    pub fn update_title(&mut self, title: String) {
        let _ = title;
        todo!()
    }

    /// 메모 내용을 갱신합니다.
    ///
    /// 이 함수는 내용 변경과 함께 `updated_at`를 자동 갱신해야 합니다.
    /// 현재 구현은 미완성으로 `todo!()` 상태입니다.
    ///
    pub fn update_content(&mut self, content: String) {
        let _ = content;
        todo!()
    }

    /// 현재 정렬 기준을 받아 다른 메모와의 정렬 순서를 비교합니다.
    ///
    /// 현재 구현은 미완성으로 `todo!()` 상태입니다.
    ///
    pub fn compare(&self, other: &Self, sort_option: SortOption) -> Ordering {
        let _ = (other, sort_option);
        todo!()
    }
}
