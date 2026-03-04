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
        match sort_option {
            SortOption::CreatedAtAsc => self
                .created_at
                .cmp(&other.created_at)
                .then_with(|| self.updated_at.cmp(&other.updated_at))
                .then_with(|| self.id.cmp(&other.id)),
            SortOption::CreatedAtDesc => self
                .created_at
                .cmp(&other.created_at)
                .reverse()
                .then_with(|| self.updated_at.cmp(&other.updated_at).reverse())
                .then_with(|| self.id.cmp(&other.id)),
            SortOption::UpdatedAtAsc => self
                .updated_at
                .cmp(&other.updated_at)
                .then_with(|| self.created_at.cmp(&other.created_at))
                .then_with(|| self.id.cmp(&other.id)),
            SortOption::UpdatedAtDesc => self
                .updated_at
                .cmp(&other.updated_at)
                .reverse()
                .then_with(|| self.created_at.cmp(&other.created_at).reverse())
                .then_with(|| self.id.cmp(&other.id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn memo(id: ID, created_at_sec: i64, updated_at_sec: i64) -> Memo {
        Memo {
            id,
            title: format!("memo-{id}"),
            content: String::from("content"),
            created_at: DateTime::from_timestamp(created_at_sec, 0).expect("valid timestamp"),
            updated_at: DateTime::from_timestamp(updated_at_sec, 0).expect("valid timestamp"),
        }
    }

    #[test]
    fn compare_created_at_asc_orders_by_created_at() {
        let left = memo(1, 10, 100);
        let right = memo(2, 20, 0);

        assert_eq!(left.compare(&right, SortOption::CreatedAtAsc), Ordering::Less);
        assert_eq!(right.compare(&left, SortOption::CreatedAtAsc), Ordering::Greater);
    }

    #[test]
    fn compare_created_at_desc_orders_by_created_at() {
        let left = memo(1, 10, 0);
        let right = memo(2, 20, 100);

        assert_eq!(left.compare(&right, SortOption::CreatedAtDesc), Ordering::Greater);
        assert_eq!(right.compare(&left, SortOption::CreatedAtDesc), Ordering::Less);
    }

    #[test]
    fn compare_updated_at_asc_orders_by_updated_at() {
        let left = memo(1, 100, 10);
        let right = memo(2, 0, 20);

        assert_eq!(left.compare(&right, SortOption::UpdatedAtAsc), Ordering::Less);
        assert_eq!(right.compare(&left, SortOption::UpdatedAtAsc), Ordering::Greater);
    }

    #[test]
    fn compare_updated_at_desc_orders_by_updated_at() {
        let left = memo(1, 0, 10);
        let right = memo(2, 100, 20);

        assert_eq!(left.compare(&right, SortOption::UpdatedAtDesc), Ordering::Greater);
        assert_eq!(right.compare(&left, SortOption::UpdatedAtDesc), Ordering::Less);
    }

    #[test]
    fn compare_updated_at_desc_uses_created_at_desc_as_tiebreaker() {
        let newer_created = memo(1, 20, 100);
        let older_created = memo(2, 10, 100);

        assert_eq!(
            newer_created.compare(&older_created, SortOption::UpdatedAtDesc),
            Ordering::Less
        );
        assert_eq!(
            older_created.compare(&newer_created, SortOption::UpdatedAtDesc),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_uses_id_asc_when_timestamps_are_equal() {
        let lower_id = memo(1, 10, 20);
        let higher_id = memo(2, 10, 20);

        for sort_option in [
            SortOption::CreatedAtAsc,
            SortOption::CreatedAtDesc,
            SortOption::UpdatedAtAsc,
            SortOption::UpdatedAtDesc,
        ] {
            assert_eq!(lower_id.compare(&higher_id, sort_option), Ordering::Less);
            assert_eq!(higher_id.compare(&lower_id, sort_option), Ordering::Greater);
        }
    }
}
