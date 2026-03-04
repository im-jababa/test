# 도메인 분류

- `memo`

## memo 도메인

메모의 생성부터 수정, 삭제, 정렬까지를 책임지는 단일 도메인입니다.

**Responsibilities**

- 메모 생성/수정/삭제
- 입력 중 실시간 저장(상태 반영)
- 정렬(생성 시각 오름/내림, 마지막 변경 시각 오름/내림)

**Key Entities**

- `Memo` (id, title, content, created_at, updated_at)
- 정렬 기준/옵션 (예: `CreatedAtAsc`, `CreatedAtDesc`, `UpdatedAtAsc`, `UpdatedAtDesc`)
