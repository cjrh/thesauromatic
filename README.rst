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
