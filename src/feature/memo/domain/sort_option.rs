/// 메모 목록 정렬에 사용하는 기준 열거형입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOption {
    /// 생성 시각 기준 오름차순 정렬입니다.
    CreatedAtAsc,
    /// 생성 시각 기준 내림차순 정렬입니다.
    CreatedAtDesc,
    /// 마지막 변경 시각 기준 오름차순 정렬입니다.
    UpdatedAtAsc,
    /// 마지막 변경 시각 기준 내림차순 정렬입니다.
    UpdatedAtDesc,
}

impl Default for SortOption {
    /// 기본 정렬 기준을 반환합니다.
    ///
    /// 현재 구현은 미완성으로 `todo!()` 상태입니다.
    fn default() -> Self {
        todo!()
    }
}
