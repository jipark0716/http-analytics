# UNTITLE

LLM이 마케터의 자연어 요청을 SQL로 변환하여 사용자 행동 데이터를 분석하는 프로그램입니다.  
기존 Firebase Analytics, GA4, GTM과 같은 마케팅 도구와 달리,  
개발자나 데이터 분석가의 도움 없이도 마케터가 직접 인사이트를 얻을 수 있도록 설계되었습니다.  

## 기존 마케팅 도구의 문제
| **문제점** | **해결방안** |
|------------|-------------|
| **데이터 분석이 어려움** <br> (SQL 작성 지식 필요, 개발자·분석가 의존) | **LLM 기반 자동 쿼리 생성** <br> 마케터의 자연어 요청을 SQL로 변환하여 즉시 분석 실행 가능 |
| **데이터 수집이 어려움** <br> (플랫폼 구조·이벤트 정의 복잡) | **Cafe24 등 정형화된 플랫폼 활용** <br> 표준화된 구조를 기반으로 빠르고 안정적인 이벤트 수집 가능 |
| **RAW 데이터 활용이 어려움** <br> (제한적 접근, 추출 비용 높음) | DB 직접 저장해서 추출 가능 |

## 단기적 목표
쇼핑몰 소유자의 고객의 니즈를 알 수 있는 데이터를 제공한다.   
이 데이터를 메시징 서비스에 활용하여 메시지 발송량을 늘린다.

## 장기적 목표
안드로이드 2015년도 부터 지원했으며   
IOS도 최근에 웹푸시를 지원하기 시작했습니다.   
https://documentation.onesignal.com/docs/web-push-for-ios

SDK를 확장하여 firebase cloud message를 사용하여   
알림톡 발송량을 일부를 웹푸시로 발송하여 알림톡 원가를 절감한다.

## 아키텍쳐
<img src="./simple_architect.png" width="600">

## 시연
엔진은 gemini-2.5-flash 을 사용하였습니다.

### 모수 조회
<img width="600" alt="시연" src="https://github.com/user-attachments/assets/3bac12db-8d05-4808-9770-dd9a81e17bab" />  

```sql
SELECT uuid -- 조건의 맞는 사용자 ID 조회
FROM event
WHERE client_id = 1 -- 어느 쇼핑몰인지
  AND event_type = 82 -- 어느 이벤트인지 (상품 조회 시작)
  AND product_id = '3' -- 어느 상품인지
  AND created_at >= toDateTime('2025-08-01 00:00:00', 9) -- 조회 시작 기간
  AND created_at <= toDateTime('2025-08-08 23:59:59', 9) -- 조회 종료 기간
GROUP BY uuid -- 같은 세션으로
HAVING count() >= 2 -- 이 행동을 2번이상 한지
```
gemini가 작성한 쿼리를 실행하니 조건의 맞는 회원의 UUID를 알 수 있었다.  

### 사용자 동향 조회
<img width="594" height="515" alt="스크린샷 2025-08-08 21 08 06" src="https://github.com/user-attachments/assets/212a4518-f10a-40a1-b49a-156352169956" />

```sql
SELECT count(DISTINCT uuid) -- 중복된 이벤트 제거
FROM event
WHERE client_id = 1 -- 어느 쇼핑몰인지
  AND event_type = 31 -- 어느 이벤트인지 (메인 페이지 조회 시작)
  AND created_at >= toDateTime('2025-08-07 15:00:00', 'UTC') -- 조회 시작 기간
  AND created_at <= toDateTime('2025-08-08 14:59:59', 'UTC');-- 조회 종료 기간

SELECT count(DISTINCT uuid) -- 중복된 이벤트 제거
FROM event
WHERE client_id = 1 -- 어느 쇼핑몰인지
  AND created_at >= toDateTime('2025-08-08 00:00:00', 9) -- 조회 시작 기간
  AND created_at <= toDateTime('2025-08-08 23:59:59', 9);-- 조회 종료 기간
```

각각의 결과로 사용자중 90.9%가 메인 페이지를 조회했다는 것을 알 수 있다.

<details>
<summary>
  [개발자용] 테이블 구조
</summary>
  DB는 clickhouse의 MergeTree엔진 사용하였습니다.  
  
```sql
create table event
(
    event_id           UUID,
    client_id          Int32,
    uuid               UUID,
    event_type         UInt8,
    order_id           Nullable(String),
    tracking_id        Nullable(String),
    product_id         Nullable(String),
    product_option_id1 Nullable(String),
    product_option_id2 Nullable(String),
    product_quantity   Nullable(UInt8),
    price              Nullable(UInt32),
    amount             Nullable(UInt32),
    keyword            Nullable(String),
    sort_by            Nullable(String),
    page               Nullable(UInt8),
    board_id           Nullable(String),
    article_id         Nullable(String),
    login_id           Nullable(String),
    phone_number       Nullable(String),
    page_url           Nullable(String),
    category_id        Nullable(String),
    scroll_bucket      Nullable(UInt8),
    created_at         DateTime64(6)
)
    engine = MergeTree PARTITION BY toYYYYMMDD(created_at)
        ORDER BY (client_id, event_type, created_at, uuid)
        SETTINGS index_granularity = 8192;
```
</details>
<details>
<summary>
  [개발자용] 그렇다면 gemini가 작성한 쿼리의 성능은 어떨까?
</summary>

## 이벤트 수집기 성능 테스트

  
엔진에 맞는 쿼리를 작성였는지 각 항목별로 살펴보겠습니다.  
  
* clickhouse는 컬럼베이스로 저정됩니다.  

| **특성** | **작성방법** | **평가** |
|------------|-------------|-------------|
| clickhouse는 컬럼베이스로 저정됩니다. | 필요한 컬럼만 select에 명시하여야 합니다. | 🟢 |
| 카디널리티 넢은 순서대로 where에 명시하여야 한다 | 카디널리티는 예상치는 다음과 같다.<br> client = 2000, event_type = 60 product_id, created_at = 알 수 없음 | 🟢 |
| PREWHERE 키워드로 디스크 페이지 전체를 스킵할 수 있습니다 | ORDER BY 에 명시된 컬럼은 PREWHERE 로 조건을 작성하여야 합니다 | 🔴 |

### 총평
일반적인 쿼리는 잘 작성하지만 엔진이해도는 낮은 수준이다.  
프롬프트개선이나 파인튜닝으로 개선의 여지가 있다.

</details>


## 성능에 관하여
작성중

### 서버 어플리케이션 관점
작성중

### 데이터 베이스 관점
작성중
