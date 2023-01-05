local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

if FORMAT:match("docx") then
  function Div(d)
    if d.identifier == "refs" then
      label = pandoc.Div(pandoc.Para("Bibliography"))
      label.attr = pandoc.Attr()
      label.attr["attributes"]["custom-style"] = "BibliographyLabel"
      return {label, d}
    end
  end
end
