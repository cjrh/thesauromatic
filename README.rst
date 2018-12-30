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
