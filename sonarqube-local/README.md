# How to setup Code Coverage CI for Rust with SonarQube

Installation
------------
### Pre-requisites
You'll need to have:
- An running instance of [SonarQube](https://docs.sonarsource.com/sonarqube/latest/try-out-sonarqube/) server
- Install `SonarQube Scanner cli` to interact with SonarQube server.
- [Clippy](https://doc.rust-lang.org/clippy/usage.html) to generate code report
- [Tarpaulin](https://github.com/xd009642/tarpaulin) to generate code coverage

### Installation
1. Download comapatible [SonarQube Community Plugin for Rust](https://github.com/C4tWithShell/community-rust) version.
2. Copy that file into folder `"$SONARQUBE_HOME"/extensions/plugins`
3. Restart SonarQube server to apply the changes

> **_NOTE:_** This is commnunity plugin.

### Generate code report and coverage
Check out [community-rust/DOC.md](https://github.com/C4tWithShell/community-rust/blob/master/DOC.md) for more details

Generate code report
```bash
cargo clippy --message-format=json &> <CLIPPY REPORT FILE>
```

Generate code coverage
```bash
just clean-db # init sqlite db for running tests
cargo tarpaulin --out Lcov
```
> **_NOTE:_** Tarpaulin supports other formats, such as: Cobertura.

> **_Attention:_** Currently, two crates are excluded - `napi` and `uniffi` because `cargo tarpaulin` failed when running these two on based-linux os. Take a loot at `tarpaulin.toml`


### Send reports

There are configurations from rust plugins
```
# sonar-project.properties
community.rust.clippy.reportPaths=clippy.json
community.rust.lcov.reportPaths=lcov.info
```

Using sonar-scanner, run
```bash
sonar-scanner \
  -Dsonar.projectKey=<project_name> \
  -Dsonar.sources=. \
  -Dsonar.host.url=<sonarqube_server_url> \
  -Dsonar.token=<token>
```

### Example with Github actions ###
```yaml
- name: Setup sonarqube
  uses: warchant/setup-sonar-scanner@v3

- name: Run cargo-tarpaulin
  shell: bash
  run: |
    RUN_MODE=local cargo tarpaulin --ignore-tests --release --out Lcov
    sonar-scanner \
      -Dsonar.projectKey=<project_name> \
      -Dsonar.host.url=<sonarqube_server_url> \
      -Dsonar.token=<token>
```

### Local testing
------------
Sample project folder
``` bash
├── docker-compose.yml
└── plugins
```

Sample docker-compose.yml
``` yaml
services:
  sonarqube:
    image: sonarqube:10.7.0-community
    depends_on:
      - db
    environment:
      SONAR_JDBC_URL: jdbc:postgresql://db:5432/sonar
      SONAR_JDBC_USERNAME: sonar
      SONAR_JDBC_PASSWORD: sonar
    volumes:
      # Required volumes of sonarqube
      - sonarqube_data:/opt/sonarqube/data
      - sonarqube_extensions:/opt/sonarqube/extensions
      - sonarqube_logs:/opt/sonarqube/logs
      # Custom volume to store SonarQube plugins
      - ./plugins:/opt/sonarqube/tmp/plugins
    ports:
      - "9000:9000"
  db:
    image: postgres:12
    environment:
      POSTGRES_USER: sonar
      POSTGRES_PASSWORD: sonar
    volumes:
      - postgresql:/var/lib/postgresql
      - postgresql_data:/var/lib/postgresql/data

volumes:
  sonarqube_data:
  sonarqube_extensions:
  sonarqube_logs:
  postgresql:
  postgresql_data:
```

Steps:
Copy sonarqube plugin to `./plugins`
Init docker containters
```bash
docker-composer up -d
```
Copy plugin file into SonarQube Plugins folder: `extensions/plugins`
```bash
docker-compose exec sonarqube bash -c 'cp "$SONARQUBE_HOME"/tmp/plugins/community-rust-plugin-0.2.4.jar "$SONARQUBE_HOME"/extensions/plugins/.'
```
Check existing plugins
```bash
docker-compose exec sonarqube bash -c 'ls -la "$SONARQUBE_HOME"/extensions/plugins'
```
> **_NOTE:_** Only one version of plugin is loaded.

Restart to apply changes
```bash
docker-compose restart sonarqube
```

Access local server to create project: `localhost:9000`. Example: `mls-local`

Generate reports and send to local sonarqube server
```bash
sonar-scanner \
  -Dsonar.projectKey=msl-local \
  -Dsonar.sources=. \
  -Dsonar.host.url=http://localhost:9000 \
  -Dsonar.token=<token>
```
