[![image](https://img.shields.io/badge/License-AGPL%203-purple.svg)](https://opensource.org/licenses/AGPL-3.0)
[![image](https://img.shields.io/badge/download-windows-green.svg?logo=windows)](https://github.com/cjrh/thesauromatic/releases/latest/download/thesauromatic.exe)
[![image](https://img.shields.io/badge/download-linux-green.svg?logo=linux)](https://github.com/cjrh/thesauromatic/releases/latest/download/thesauromatic)

thesauromatic
=============

Zero-dependency command-line CLI thesaurus

Demo
----

Pretty much just give it a word, and you get a bunch of similar or
related words back:

``` {.bash}
$ ./thesauromatic deluge
Niagara
abound
affusion
alluvion
alluvium

<...snip...>

waterflood
watering
waterspout
wet
wetting
whelm
whelming
```

Features
--------

-   Synonyms from the [Moby
    project](https://en.wikipedia.org/wiki/Moby_Project)
-   Statically compiled, word lists are linked in. No dependencies. Just
    download an executable for your target platform.
-   Fast; takes about 40 ms to emit the words. This makes it easy to
    drive from your editor, and will work offline.

Install
-------

Install from [crates.io](https://crates.io/crates/thesauromatic) with
either of the following.

Build from source with cargo:

``` {.bash}
$ cargo install thesauromatic
```

Or download a prebuilt binary with
[cargo-binstall](https://github.com/cargo-bins/cargo-binstall) (no
compilation, just fetches the release artifact for your platform):

``` {.bash}
$ cargo binstall thesauromatic
```

Alternatively, grab the executable directly from the *Releases* tab.

Making a new release
--------------------

Releases are driven by [cargo-release](https://github.com/crate-ci/cargo-release):

``` {.bash}
$ cargo release patch --execute
```

This bumps the version, commits, publishes to crates.io, and creates and
pushes a matching `vX.Y.Z` tag. Pushing that tag triggers the *Release*
workflow, which builds the prebuilt binaries and attaches them to a new
GitHub Release. `cargo binstall` then finds those binaries via the
`[package.metadata.binstall]` entry in `Cargo.toml`.

Tips & Tricks
-------------

Take advantage of CLI filters! How about formatting the output into
columns?

``` {.bash}
$ ./thesauromatic flippant | column -c70
airy            facetious       leering         smart
bantering       facy            light-hearted   smart-alecky
belittling      fleering        malapert        smart-ass
biggety         flip            mocking         smirking
bluff           fooling         nervy           sneering
booing          forgetful       oblivious       snickering
brash           free and easy   offhand         sniggering
brazen          fresh           offhanded       snorting
careless        frivolous       panning         supercilious
casual          gally           perfunctory     superficial
catcalling      gratuitous      pert            tactless
chaffing        grinning        quizzical       taunting
cheeky          heedless        ragging         teasing
chutzpadik      hissing         railing         thoughtless
cocky           hooting         rallying        twitting
contemptuous    impertinent     razzing         uncalled-for
crusty          impudent        reckless        undiplomatic
cursory         inconsiderate   regardless      unheedful
degage          indifferent     respectless     unheeding
derisive        insolent        ridiculing      unmindful
derisory        insouciant      roasting        unprepared
discourteous    irreverent      rude            unready
dismissive      jeering         sassy           unserious
disregardant    jocular         saucy           unsolicitous
disregardful    joshing         scoffing        untactful
disrespectful   kidding         scornful        unthinking
easygoing       lazy            shallow         wise-ass
```

Perhaps you\'re also trying to complete an
[alliteration](https://en.wikipedia.org/wiki/Alliteration) on the letter
`r`?

``` {.bash}
$ ./thesauromatic flippant | grep ^r | column -c70
ragging         razzing         respectless     rude
railing         reckless        ridiculing
rallying        regardless      roasting
```
