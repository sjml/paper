-- a custom writer that just spits out the citation keys for everything used
--   in the given document

local utils = dofile(pandoc.path.join({
  pandoc.path.directory(PANDOC_SCRIPT_FILE),
  "..",
  "project_template",
  ".paper_resources",
  "filters",
  "util.lua",
}))

function Writer(doc, opts)
  -- could just grab the references from the doc object,
  --    but want to handle the vulgate special case
  local cites = {}
  doc:walk({
    Cite = function(cite)
      for _, c in pairs(cite.citations) do
        if not utils.starts_with(c.id, "Bible-") then
          cites[c.id] = true
        end
        if c.id == "Bible-Vulgatam" then
          cites[pandoc.utils.stringify(doc.meta.vulgate_cite_key)] = true
        end
      end
    end,
  })

  local ids = {}
  for ckey, _ in pairs(cites) do
    table.insert(ids, ckey)
  end
  return table.concat(ids, "\n")
end
