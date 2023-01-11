Citing the Vulgate the first time produces a footnote, as in "In principio erat
Verbum, et Verbum erat apud Deum, et Deus erat Verbum."[@Bible-Vulgatam John
1:1] But a second citation becomes parenthetical with an italicized attribution
as in "Qui non diligit, non novit Deum: quoniam Deus caritas est."
[@Bible-Vulgatam 1 John 4:8]

The first time the Catechism gets cited, it's the full deal. [@CCC.2017 p45] But
what does Tommy Quine-Quine have to say about the same subject? [@aquinasSumma
p48] CCC with a stirring rebuttal! [@CCC.2017 p. 49] (Notice that it just
becomes "CCC" on subsequents.)

That one was easy, because the [Citation Style
Language](https://citationstyles.org/) (note URLs for links go in footnotes, but
only in LaTeX) has ready support for short titles, and since the Catechism
doesn't have an *author* to list, it works out. What's trickier is the USCCB,
which wants to get referenced by its full name the first time,[@usccbPPF52006]
but then afterwards gets "institutionally abbreviated."[@usccbPPF52006] (Support
for author short names is supposed to come in CSL 1.1, but there's no timetable
on that, so gotta use [my little Lua
filters](https://github.com/sjml/paper/tree/main/paper/resources/project_template/.paper_resources/filters)
to get it done.) It works with multiple citations, too. [@CCC.2017
p45; @usccbPPF52006 §17]

For Papal Encyclicals, we need to make sure the capitalization of the Latin name
stays consistent, so this cite [@francisFratelliTutti2020 §42] should mention
"*Fratelli tutti*" but not in title case. Should also happen for its second
mention[@francisFratelliTutti2020 §56] where it only goes by the Latin name. No
quotes around any of it in the citation.

In the end, we give Tommy the final word,[@aquinasSumma I-II, Q. 12] and note
the author drop from his subsequent citation.
