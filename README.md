# Dscribe

This is a port of a shell script for searching my notes using ripgrep and fzf

Ripgrep and fzf are currrently external dependencies

# TODO

- remove dependencies
  - add skim and grep-searcher
- add config file support
- remove searching+mathcing by line number
- add TUI for handling front matter
  - add pure text calender front matter parsing/writing
  - add tags system
- add functions to remove front matter upon open. replace front matter when done in editor
    - currently finds, removes, and re-adds front matter while opening and finally closing
        - no error handling or edges cases handled yet
- add link based system across notes
- need to handle errors and remove all unwraps
