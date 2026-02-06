# Skillsm

TUI for skills.sh

![Skillsm demo](assets/record.gif)


## Install

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/jossephus/skillsm/releases/latest/download/skillsm-installer.sh | sh
```

Build using cargo:

```bash
cargo build --release
```

Notes:

The detail page is a little bit fragile right now as it tries to guess where the skills.md file is based on the repo name and the skill name, but its working fine for most of the cases.
