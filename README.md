# Bracket-UI

**Work-in-progress. Not ready for human consumption.**

Bracket-UI is intended to provide a user-interface module for use with [Bracket-lib](https://github.com/amethyst/bracket-lib) projects. It won't do *everything*, but should provide enough functionality to reduce the tedium of UI building.

## What currently works?

You can find examples showing in-development functionality of building layouts and querying elements:

* `minimal` is the "hello world" of bracket-ui.
* `panel_layouts` demonstrates early panel functionality, with absolute and relative placements supported. Nesting panels is encouraged.
* `splits` shows how to use the panel `split_horizontal` and `split_vertical` functions to sub-divide panels into sections.
* `updates` uses the widget query/set system to push UI updates. Currently updates a label, a title, and shows/hides a panel.

The syntax is still pretty wretched. That will be a later target; make it work first, and then make it pretty!

## What doesn't work yet?

A lot of things. Most notably, any sort of user interaction with controls. I'm also hoping for a stylesheet system and a builder pattern to make UI building less painful.
