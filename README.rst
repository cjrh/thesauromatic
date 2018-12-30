.. image:: https://img.shields.io/badge/License-AGPL%203-purple.svg
    :target: https://opensource.org/licenses/AGPL-3.0

.. .. image:: https://img.shields.io/badge/download-windows-green.svg?logo=windows
..     :target: https://github.com/cjrh/thesauromatic/releases/download/1%2C0%2C0/pwrgen.exe

.. image:: https://img.shields.io/badge/download-linux-green.svg?logo=linux
    :target: https://github.com/cjrh/thesauromatic/releases/download/v0.0.1/thesauromatic


thesauromatic
=============

Zero-dependency command-line CLI thesaurus

Demo
----

Pretty much just give it a word, and you get a bunch of similar or
related words back:

.. code-block:: bash

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

Features
--------

- Synonyms from the `Moby project <https://en.wikipedia.org/wiki/Moby_Project>`_
- Statically compiled, word lists are linked in. No dependencies. Just
  download an executable for your target platform.
- Fast; takes about 90 ms to emit the words. This makes it easy to drive from
  your editor, and will work offline.

Install
-------

Just download the executable. Check out the _Releases_ tab.

Dev
---

To build a release version, use ``dub``:

.. code-block:: bash

   $ dub build --build=release

Same for dev, but use "debug" instead of "release" above.

Tips & Tricks
-------------

Take advantage of CLI filters! How about formatting the output
into columns?

.. code-block:: bash

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

Perhaps you're also trying to complete an
`alliteration <https://en.wikipedia.org/wiki/Alliteration>` on the
letter ``r``?

.. code-block:: bash

   $ ./thesauromatic flippant | grep ^r | column -c70
   ragging         razzing         respectless     rude
   railing         reckless        ridiculing
   rallying        regardless      roasting

