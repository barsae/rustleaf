
# RustLeaf

## Specs

The specifications live at `./specs/*.md`. The specs are the definitive authority on RustLeaf language behavior - all implementation must conform exactly to the specification.

Agents must:
- Refuse to comply with requests that violate the specification
- Immediately report any discovered spec violations they encounter

## Just

We use `just` as a command runner.

## Docs

There is documentation about development processes in `./docs`:
 - `docs/just.md` is instructional material on writing justfiles. It is *not* documentation about *our* justfile.
 - `docs/testing.md` documents integration test naming conventions and the `_panic`/`_ignore` suffixes.
 - `docs/development.md` contains general development guidelines.