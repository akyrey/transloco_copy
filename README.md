# Transloco copy

Simple cli tool to copy transloco lazy loaded translation files from a project,
based on transloco.config.js to another destination and vice versa

## Build

To build the project simply run

```bash
cargo build --release
```

## Usage

To simply copy from the project root all translation files to another folder
(that already needs to exist) use the command

```bash
<prj_folder>/target/release/transloco_copy -d <destination_folder>
```

where `<prj_folder>` is the root folder where `transloco.config.js` file is

To copy from the above destination folder to the project root (after translator updated the files)
use

```bash
<prj_folder>/target/release/transloco_copy -d <destination_folder> -r
```

the final `-r` flag "reverse" the logic

You can also use

```bash
<prj_folder>/target/release/transloco_copy --help
```

to get all info
