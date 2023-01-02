-- Swaps inline links to footnotes so they're useful in a printed document as well
--   NB this doesn't have anything to do with citations that have links, since it
--      runs *before* citeproc has actually inserted them into the document.
--   Only operates on docx writer since the LaTeX template handles this on its own.

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

if FORMAT:match("docx") then
  function Link(elem)
    local footnote = pandoc.Note(pandoc.Para(pandoc.Link(elem.target, elem.target)))
    table.insert(elem.content, footnote)
    return elem.content
  end
end
