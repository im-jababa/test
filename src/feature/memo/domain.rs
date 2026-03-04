use super::id::ID;
use super::sort_option::SortOption;
use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

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
    /// 이 함수는 ID를 자동 발급하고 `created_at`, `updated_at`를 자동 생성합니다.
    pub fn new(title: String, content: String) -> Self {
        let id = NEXT_ID.fetch_add(1, AtomicOrdering::Relaxed);
        let now = Utc::now();

        Self {
            id,
            title,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    /// 메모 제목을 갱신합니다.
    ///
    /// 이 함수는 제목 변경과 함께 `updated_at`를 자동 갱신합니다.
    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// 메모 내용을 갱신합니다.
    ///
    /// 이 함수는 내용 변경과 함께 `updated_at`를 자동 갱신합니다.
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now();
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

#[cfg(test)]
mod tests {
    use super::Memo;

    #[test]
    fn new_sets_id_and_initial_timestamps() {
        let memo = Memo::new("title".to_string(), "content".to_string());

        assert!(memo.id >= 1);
        assert_eq!(memo.created_at, memo.updated_at);
    }

    #[test]
    fn new_assigns_increasing_ids() {
        let first = Memo::new("first".to_string(), "content".to_string());
        let second = Memo::new("second".to_string(), "content".to_string());

        assert!(second.id > first.id);
    }

    #[test]
    fn update_title_updates_value_and_timestamp() {
        let mut memo = Memo::new("before".to_string(), "content".to_string());
        let before_updated_at = memo.updated_at;

        memo.update_title("after".to_string());

        assert_eq!(memo.title, "after");
        assert!(memo.updated_at >= before_updated_at);
    }

    #[test]
    fn update_content_updates_value_and_timestamp() {
        let mut memo = Memo::new("title".to_string(), "before".to_string());
        let before_updated_at = memo.updated_at;

        memo.update_content("after".to_string());

        assert_eq!(memo.content, "after");
        assert!(memo.updated_at >= before_updated_at);
    }
}
