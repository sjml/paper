-- Detects a `\Adonai` LaTeX macro and swaps in a small-caps "Lord" in the docx writer

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

if FORMAT:match("docx") then
  function RawInline(ri)
    if ri.format == "tex" and utils.starts_with(ri.text, "\\Adonai") then
      local ri = pandoc.RawInline("openxml", '<w:r><w:rPr><w:smallCaps w:val="true"/></w:rPr><w:t>Lord </w:t></w:r>')
      return ri
    end
  end
end
