-- Detects a `\noindent` LaTeX command at the start of a paragraph and applies an
--   appropriate Word style to match.

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

if FORMAT:match("docx") then
  function Para(p)
    if #p.c < 1 then
      return nil
    end
    if p.c[1].tag == "RawInline" then
      if p.c[1].format == "tex" and utils.starts_with(p.c[1].text, "\\noindent") then
        table.remove(p.c, 1)
        local d = pandoc.Div(p)
        d.attr = pandoc.Attr()
        d.attr["attributes"]["custom-style"] = "Body Text 2"
        return d
      end
    end
  end
end
