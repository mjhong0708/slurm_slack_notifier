# slurm_slack_notifier

Slurm에 올린 작업이 끝나면 slack으로 알림을 주는 프로그램

## How to Install

### 1. 미리 빌드된 바이너리 사용 (추천)

- `release` 탭의 가장 최근 릴리즈를 클릭하여 아래 Assets의 `monitorjob` 파일을 다운받는다.
- `monitorjob` 파일을 `$PATH` 환경변수에 있는 경로로 복사한다. (일반적으로 `~.local/bin`)
- `chmod u+x monitorjob`을 실행해 실행 권한을 추가한다.

### 2. 소스 코드 빌드하기 (1번 실패시)

- [`rustup`을 설치한다.](https://rustup.rs/)
- 소스 코드를 다운받는다.
- `cargo build --release` 명령어를 실행한다.
- `target/release` 폴더의 `monitorjob` 파일을 `$PATH` 환경변수에 있는 경로로 복사한다.
- `chmod u+x monitorjob`을 실행해 실행 권한을 추가한다.

## How to Use

### 1. Webhook용 slack app 생성하기

[이 블로그 포스트](https://oingdaddy.tistory.com/146)를 참고하여 slack app을 만들고, 여기서 생성된 endpoint의 url을 잘 복사해놓는다.

### 2. `config.yaml` 파일 작성하기

- `~/.config/monitorjob` 폴더를 만든다.
- 위 폴더 안에 `config.json` 파일을 다음과 같이 작성한다.

    ```json
    {
       "username": "<username>",                  // 리눅스 계정 이름
       "slack": {
           "endpoint": "<webhook endpoint url>",  // 슬랙 앱의 webhook endpoint url
           "channel": "<#channel name>",          // 채널 이름. '#'을 포함하여 작성
           "app_name": "<app name>"               // 슬랙 앱 이름
       }
    }
    ```

### 3. 프로그램 실행하기

- 일시적으로 실행: 원격 터미널을 열고 `monitorjob` 명령어를 실행한다. 터미널 연결이 종료되면 프로그램이 종료된다.
- 반영구적으로 실행: 원격 터미널을 열고 적당한 폴더를 생성한 후 (ex. `~/monitorjob_log`) 해당 폴더에서 `nohup monitorjob > history.log`를 실행한다.
  - 종료 방법: `top` (또는 `htop`)을 실행하고 본인 계정명에서 `monitorjob` 이름으로 돌아가고 있는 프로세스 ID(pid)를 확인한 후 `kill <pid>`로 프로세스를 종료한다.

## TODO

- Refactoring
- User name 자동 인식
- 예외 핸들링 구체화
